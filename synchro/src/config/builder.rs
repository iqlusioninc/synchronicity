//! Libra configuration builder.
//!
//! Parts adapted from upstream Libra's `SwarmConfigBuilder`:
//! <https://github.com/libra/libra/blob/master/config/config-builder/src/swarm_config.rs>

use super::{
    key_seed::KeySeed,
    keys::{ConsensusKeyPair, NetworkKeyPairs},
    peer_info::{self, PeerInfo},
    trusted_peers::{
        self, ConsensusPeersConfig, ConsensusPrivateKey, NetworkPeersConfig, NetworkPrivateKeys,
    },
    ConsensusConfig, NetworkConfig, NodeConfig, PersistableConfig, RoleType,
};
use crate::types::account_address::AccountAddress;
use parity_multiaddr::Multiaddr;
use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

/// Default address to listen on
pub const DEFAULT_LISTEN_ADDRESS: &str = "0.0.0.0";

/// Default port to listen on
pub const DEFAULT_PORT: u16 = 6180;

/// Libra configuration builder
pub struct Builder {
    /// Seed to use when generating keys (default random)
    key_seed: KeySeed,

    /// Output directory
    output_dir: PathBuf,

    /// Address to listen on
    listen_address: Multiaddr,

    /// Address to advertise to the network
    advertised_address: Multiaddr,

    /// Node `RoleType` (either `Validator` or `FullNode`)
    role: RoleType,

    /// Is this network permissioned?
    is_permissioned: bool,

    /// Seed address to include in `peer_info.toml`
    seed_address: Option<Multiaddr>,
}

impl Builder {
    /// Create a new config builder
    pub fn new(key_seed: KeySeed) -> Self {
        let listen_address =
            parse_multiaddr_from_ipv4(DEFAULT_LISTEN_ADDRESS, DEFAULT_PORT).unwrap();

        let advertised_address = listen_address.clone();

        Self {
            output_dir: PathBuf::from("."),
            listen_address,
            advertised_address,
            key_seed,
            role: RoleType::Validator,
            is_permissioned: true,
            seed_address: None,
        }
    }

    /// Set output directory
    pub fn with_output_dir(&mut self, output_dir: impl AsRef<Path>) -> &mut Self {
        self.output_dir = output_dir.as_ref().to_path_buf();
        self
    }

    /// Set listen address
    pub fn with_listen_address(&mut self, listen_address: impl Display) -> &mut Self {
        self.listen_address = parse_multiaddr_from_ipv4(listen_address, DEFAULT_PORT).unwrap();
        self
    }

    /// Set advertised address
    pub fn with_advertised_address(&mut self, advertised_address: impl Display) -> &mut Self {
        self.advertised_address =
            parse_multiaddr_from_ipv4(advertised_address, DEFAULT_PORT).unwrap();
        self
    }

    /// Configure whether or not the network is permissioned
    pub fn with_is_permissioned(&mut self, is_permissioned: bool) -> &mut Self {
        // TODO(tarcieri): support permissionless networks
        assert!(
            !is_permissioned,
            "support for `is_permissioned: false` unimplemented"
        );
        self
    }

    /// Configure a seed IP address for this node to include in `peer_info.toml`
    pub fn with_seed_address(&mut self, seed_address: impl Display) -> &mut Self {
        self.seed_address = Some(parse_multiaddr_from_ipv4(seed_address, DEFAULT_PORT).unwrap());
        self
    }

    /// Generate keys and initial configuration settings
    pub fn generate_keys_and_configs(
        &self,
        domain: &[u8],
        version: u32,
    ) -> (
        HashMap<AccountAddress, (ConsensusPrivateKey, NetworkPrivateKeys)>,
        ConsensusPeersConfig,
        NetworkPeersConfig,
    ) {
        let seed = self.key_seed.derive_seed(domain, version);
        trusted_peers::ConfigHelpers::gen_validator_nodes(1, Some(seed))
    }

    /// Generate `ConsensusConfig` and write `consensus_keypair.config.toml`
    pub fn generate_consensus_config(&self, private_key: ConsensusPrivateKey) -> ConsensusConfig {
        let consensus_config = ConsensusConfig::default();

        let consensus_keypair = ConsensusKeyPair::load(Some(private_key.consensus_private_key));
        let consensus_keypair_file = self
            .output_dir
            .join(&consensus_config.consensus_keypair_file);

        consensus_keypair.save_config(&consensus_keypair_file);
        consensus_config
    }

    /// Generate `NetworkConfig` and write `network_keypairs.config.toml`
    pub fn generate_network_config(
        &self,
        peer_id: &str,
        private_keys: NetworkPrivateKeys,
    ) -> NetworkConfig {
        let network_keypairs = NetworkKeyPairs::load(
            private_keys.network_signing_private_key,
            private_keys.network_identity_private_key,
        );

        let mut network_config = NetworkConfig::default();
        network_config.peer_id = peer_id.to_owned();

        network_config.role = match self.role {
            RoleType::Validator => "validator",
            RoleType::FullNode => "full_node",
        }
        .to_owned();

        network_config.listen_address = self.listen_address.clone();
        network_config.advertised_address = self.advertised_address.clone();
        network_config.is_permissioned = self.is_permissioned;

        let network_keypairs_file = self.output_dir.join(&network_config.network_keypairs_file);
        network_keypairs.save_config(&network_keypairs_file);
        network_config
    }

    /// Generate `NodeConfig` and write `node.config.toml`
    pub fn generate_node_config(
        &self,
        consensus_config: ConsensusConfig,
        network_config: NetworkConfig,
    ) -> NodeConfig {
        let node_config = NodeConfig {
            base: Default::default(),
            networks: vec![network_config],
            consensus: consensus_config,
            metrics: Default::default(),
            execution: Default::default(),
            admission_control: Default::default(),
            debug_interface: Default::default(),
            storage: Default::default(),
            mempool: Default::default(),
            state_sync: Default::default(),
            log_collector: Default::default(),
            vm_config: Default::default(),
        };

        let node_config_file = self.output_dir.join("node.config.toml");
        node_config.save_config(&node_config_file);
        node_config
    }

    /// Generate `PeerInfo` and write `peer_info.toml`
    pub fn generate_peer_info(
        &self,
        peer_id: &str,
        consensus_peers: ConsensusPeersConfig,
        network_peers: NetworkPeersConfig,
    ) -> PeerInfo {
        let consensus_info = consensus_peers.peers.into_iter().next().unwrap().1;
        let network_info = network_peers.peers.into_iter().next().unwrap().1;
        let peer_info = PeerInfo::new(
            peer_id,
            self.seed_address.as_ref(),
            consensus_info,
            network_info,
        );

        let peer_info_file = self.output_dir.join(peer_info::DEFAULT_FILENAME);
        peer_info.save_config(&peer_info_file);
        peer_info
    }
}

/// Parse an IP address into a Multiaddr
fn parse_multiaddr_from_ipv4(
    ipv4_addr: impl Display,
    port: u16,
) -> Result<Multiaddr, parity_multiaddr::Error> {
    format!("/ip4/{}/tcp/{}", ipv4_addr, port).parse()
}
