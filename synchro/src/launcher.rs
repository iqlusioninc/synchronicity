//! Launcher - starts a node with the given configuration an executor

use crate::{error::Error, node::Node, transaction::NewVerifier};
use std::{
    convert::TryInto,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Instant,
};

// Libra types

use consensus::consensus_provider::{make_consensus_provider, ConsensusProvider};
use executor::Executor;
use grpc_helpers::ServerHandle;
use grpcio::EnvBuilder;
use libra_config::config::{NodeConfig, RoleType};
use libra_crypto::ed25519::Ed25519PublicKey;
use libra_mempool::{core_mempool::CoreMempool, mempool_service::MempoolService, MempoolRuntime};
use libra_types::account_address::AccountAddress as PeerId;
use log::debug;
use network::{
    validator_network::{
        network_builder::{NetworkBuilder, TransportType},
        ConsensusNetworkEvents,
        ConsensusNetworkSender,
        LibraNetworkProvider,
        // when you add a new protocol const, you must add this in either
        // .direct_send_protocols or .rpc_protocols vector of network_builder in setup_network()
        ADMISSION_CONTROL_RPC_PROTOCOL,
        CONSENSUS_DIRECT_SEND_PROTOCOL,
        CONSENSUS_RPC_PROTOCOL,
        MEMPOOL_DIRECT_SEND_PROTOCOL,
        STATE_SYNCHRONIZER_MSG_PROTOCOL,
    },
    NetworkPublicKeys, ProtocolId,
};
use state_synchronizer::StateSynchronizer;
use storage_client::{StorageRead, StorageReadServiceClient, StorageWriteServiceClient};
use tokio::runtime::Runtime;
use vm_runtime::VMExecutor;

/// Launcher - launches a Synchro node
pub struct Launcher<V: NewVerifier> {
    /// Node configuration
    node_config: NodeConfig,

    /// Peer ID
    peer_id: PeerId,

    /// Node role
    role: RoleType,

    /// Transaction verification provider
    verify_provider: V,
}

impl<V> Launcher<V>
where
    V: NewVerifier,
{
    /// Create a new launcher
    pub fn new(node_config: NodeConfig, verify_provider: V) -> Result<Self, Error> {
        if let Some(net_config) = node_config.networks.get(0) {
            let peer_id = PeerId::from_hex_literal(&net_config.peer_id)?;
            let role = RoleType::from(&net_config.role);

            Ok(Self {
                node_config,
                peer_id,
                role,
                verify_provider,
            })
        } else {
            // TODO(tarcieri): don't panic!
            panic!(
                "unexpected number of network configs: {} (expected 1)",
                node_config.networks.len()
            );
        }
    }

    /// Launch the node
    pub fn launch<E>(mut self) -> Result<Node<E>, Error>
    where
        E: VMExecutor + Send + Sync + 'static,
    {
        let runtime = crate::start_runtime();
        let mut network_provider = self.start_network_provider(&runtime);

        // Note: We need to start network provider before consensus, because the consensus
        // initialization is blocked on state synchronizer to sync to the initial root ledger
        // info, which in turn cannot make progress before network initialization
        // because the NewPeer events which state synchronizer uses to know its
        // peers are delivered by network provider. If we were to start network
        // provider after consensus, we create a cyclic dependency from
        // network provider -> consensus -> state synchronizer -> network provider. This deadlock
        // was observed in GitHub Issue #749. A long term fix might be make
        // consensus initialization async instead of blocking on state synchronizer.
        let mempool = self.start_mempool(network_provider.as_mut());

        let (consensus_network_sender, consensus_network_events) =
            network_provider.add_consensus(vec![
                ProtocolId::from_static(CONSENSUS_RPC_PROTOCOL),
                ProtocolId::from_static(CONSENSUS_DIRECT_SEND_PROTOCOL),
            ]);

        runtime.executor().spawn(network_provider.start());
        debug!("network started for peer_id: {}", &self.peer_id);

        let executor = self.start_executor();
        let consensus = self.start_consensus_provider(
            Arc::clone(&executor),
            consensus_network_sender,
            consensus_network_events,
        )?;

        Ok(Node {
            runtime,
            consensus,
            mempool,
            executor,
        })
    }

    /// Start the network provider
    fn start_network_provider(&mut self, runtime: &Runtime) -> Box<dyn LibraNetworkProvider> {
        // NOTE: this is asserted to exist in `Launcher::new`
        let network_signing_private = self.node_config.networks[0].network_keypairs.take_network_signing_private().expect(
            "failed to move network signing private key out of NodeConfig: key not set or moved already"
        );

        let network_signing_public = Ed25519PublicKey::from(&network_signing_private);

        // NOTE: this is asserted to exist in `Launcher::new`
        let network_config = &self.node_config.networks[0];

        let mut network_builder = NetworkBuilder::new(
            runtime.executor(),
            self.peer_id,
            network_config.listen_address.clone(),
            self.role,
        );

        network_builder
            .permissioned(network_config.is_permissioned)
            .advertised_address(network_config.advertised_address.clone())
            .direct_send_protocols(vec![
                ProtocolId::from_static(CONSENSUS_DIRECT_SEND_PROTOCOL),
                ProtocolId::from_static(MEMPOOL_DIRECT_SEND_PROTOCOL),
                ProtocolId::from_static(STATE_SYNCHRONIZER_MSG_PROTOCOL),
            ])
            .rpc_protocols(vec![
                ProtocolId::from_static(CONSENSUS_RPC_PROTOCOL),
                ProtocolId::from_static(ADMISSION_CONTROL_RPC_PROTOCOL),
            ]);

        let trusted_peers = network_config
            .network_peers
            .peers
            .iter()
            .map(|(peer_id, keys)| {
                (
                    PeerId::from_str(peer_id).unwrap(),
                    NetworkPublicKeys {
                        signing_public_key: keys.network_signing_pubkey.clone(),
                        identity_public_key: keys.network_identity_pubkey.clone(),
                    },
                )
            })
            .collect();

        let seed_peers = network_config
            .seed_peers
            .seed_peers
            .clone()
            .into_iter()
            .map(|(peer_id, addrs)| (peer_id.try_into().expect("invalid PeerId"), addrs))
            .collect();

        network_builder
            .transport(TransportType::TcpNoise(Some(
                network_config
                    .network_keypairs
                    .get_network_identity_keypair(),
            )))
            .connectivity_check_interval_ms(network_config.connectivity_check_interval_ms)
            .seed_peers(seed_peers)
            .trusted_peers(trusted_peers)
            .signing_keys((network_signing_private, network_signing_public))
            .discovery_interval_ms(network_config.discovery_interval_ms);

        let (listen_addr, network_provider) = network_builder.build();
        debug!("listen addr: {:?}", listen_addr);

        network_provider
    }

    /// Start the mempool for this node
    fn start_mempool(&self, network_provider: &mut dyn LibraNetworkProvider) -> MempoolRuntime {
        let (network_sender, network_events) = network_provider
            .add_mempool(vec![ProtocolId::from_static(MEMPOOL_DIRECT_SEND_PROTOCOL)]);

        // Initialize and start mempool.
        let instant = Instant::now();

        let config = &self.node_config;
        let mempool = Arc::new(Mutex::new(CoreMempool::new(&config)));
        let cq_count = 2; // TODO: customize count based on CPUs?

        // setup grpc server
        let env = Arc::new(
            EnvBuilder::new()
                .name_prefix("grpc-mempool-")
                .cq_count(cq_count)
                .build(),
        );

        let handle = MempoolService {
            core_mempool: Arc::clone(&mempool),
        };

        let service = libra_mempool::proto::mempool::create_mempool(handle);
        let grpc_server = grpcio::ServerBuilder::new(env)
            .register_service(service)
            .bind(
                config.mempool.address.clone(),
                config.mempool.mempool_service_port,
            )
            .build()
            .expect("[mempool] unable to create grpc server");

        // setup shared mempool
        let storage_client: Arc<dyn StorageRead> = Arc::new(StorageReadServiceClient::new(
            Arc::new(EnvBuilder::new().name_prefix("grpc-mem-sto-").build()),
            "localhost",
            config.storage.port,
        ));

        let verifier = Arc::new(
            self.verify_provider
                .new_verifier(Arc::clone(&storage_client)),
        );

        // TODO(tarcieri): mempool subscribers?
        let subscribers = vec![];

        // TODO(tarcieri): timer?
        let timer = None;

        let shared_mempool = libra_mempool::shared_mempool::start_shared_mempool(
            config,
            mempool,
            network_sender,
            network_events,
            storage_client,
            verifier,
            subscribers,
            timer,
        );

        debug!("mempool started in {} ms", instant.elapsed().as_millis());

        MempoolRuntime {
            grpc_server: ServerHandle::setup(grpc_server),
            shared_mempool,
        }
    }

    /// Start the consensus provider
    fn start_consensus_provider<E>(
        &mut self,
        executor: Arc<Executor<E>>,
        consensus_network_sender: ConsensusNetworkSender,
        consensus_network_events: ConsensusNetworkEvents,
    ) -> Result<Box<dyn ConsensusProvider>, Error>
    where
        E: VMExecutor + Send + Sync + 'static,
    {
        // Initialize and start consensus.
        let instant = Instant::now();

        // TODO(tarcieri): populate these?
        let state_sync_network_handles = vec![];

        let state_synchronizer = StateSynchronizer::bootstrap(
            state_sync_network_handles,
            Arc::clone(&executor),
            &self.node_config,
        );

        let mut consensus_provider = make_consensus_provider(
            &mut self.node_config,
            consensus_network_sender,
            consensus_network_events,
            executor,
            state_synchronizer.create_client(),
        );

        consensus_provider.start()?;
        debug!("consensus started in {} ms", instant.elapsed().as_millis());

        Ok(consensus_provider)
    }

    /// Start the executor for this node
    fn start_executor<E>(&self) -> Arc<Executor<E>>
    where
        E: VMExecutor + Send + Sync + 'static,
    {
        let client_env = Arc::new(EnvBuilder::new().name_prefix("grpc-exe-sto-").build());

        let storage_read_client = Arc::new(StorageReadServiceClient::new(
            Arc::clone(&client_env),
            &self.node_config.storage.address,
            self.node_config.storage.port,
        ));

        let storage_write_client = Arc::new(StorageWriteServiceClient::new(
            Arc::clone(&client_env),
            &self.node_config.storage.address,
            self.node_config.storage.port,
            self.node_config.storage.grpc_max_receive_len,
        ));

        Arc::new(Executor::new(
            Arc::clone(&storage_read_client) as Arc<dyn StorageRead>,
            storage_write_client,
            &self.node_config,
        ))
    }
}
