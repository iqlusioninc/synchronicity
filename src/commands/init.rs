//! `init` subcommand

use crate::prelude::*;
use abscissa_core::{Command, Options, Runnable};
use std::{fs, path::PathBuf, process::exit};
use synchro::config::{self, peer_info, KeySeed};

/// Derivation component used when computing seed
pub const DERIVATION_COMPONENT: &[u8] = b"synchronicity";

/// `init` subcommand
#[derive(Command, Debug, Options)]
pub struct InitCmd {
    /// Listen address to bind to (default 0.0.0.0)
    #[options(short = "l", long = "listen", help = "listen on this IP address")]
    listen_address: Option<String>,

    /// Path to the base directory
    #[options(free)]
    base_dir: Vec<PathBuf>,
}

impl Runnable for InitCmd {
    /// Initialize application configuration.
    fn run(&self) {
        if self.base_dir.len() != 1 {
            status_err!("usage: synchronicity init /path/to/syncronicity/base/dir");
            exit(1);
        }

        // TODO(tarcieri): support for reusing a previously generated `KeySeed`
        let key_seed = KeySeed::generate();

        let base_dir = self.base_dir[0].as_path();
        fs::create_dir_all(base_dir).unwrap_or_else(|e| {
            status_err!("couldn't create base dir: {}", e);
            exit(1);
        });

        let mut builder = config::Builder::new(key_seed);
        builder.with_output_dir(base_dir);

        if let Some(listen_addr) = &self.listen_address {
            builder.with_listen_address(listen_addr);
        }

        // Generate private keys as well as consensus and network configs
        let (private_keys, consensus_peers_config, network_peers_config) =
            builder.generate_keys_and_configs(DERIVATION_COMPONENT, 0);

        let peer_id = network_peers_config.peers.keys().next().unwrap().to_owned();

        builder.generate_peer_info(&peer_id, consensus_peers_config, network_peers_config);
        status_ok!(
            "Generated",
            "{}",
            base_dir.join(peer_info::DEFAULT_FILENAME).display()
        );

        let (_account, (consensus_private_key, network_private_keys)) =
            private_keys.into_iter().next().unwrap();

        let consensus_config = builder.generate_consensus_config(consensus_private_key);
        status_ok!(
            "Generated",
            "{}",
            base_dir
                .join(&consensus_config.consensus_keypair_file)
                .display()
        );

        let network_config = builder.generate_network_config(&peer_id, network_private_keys);
        status_ok!(
            "Generated",
            "{}",
            base_dir
                .join(&network_config.network_keypairs_file)
                .display()
        );

        builder.generate_node_config(consensus_config, network_config);
        status_ok!(
            "Generated",
            "{}",
            base_dir.join("node.config.toml").display()
        );
    }
}
