//! `init` subcommand

use crate::{config::CONFIG_FILE, prelude::*};
use abscissa_core::{Command, Options, Runnable};
use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};
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
        let base_dir = self.prepare_base_dir();

        // TODO(tarcieri): support for reusing a previously generated `KeySeed`
        let key_seed = KeySeed::generate();

        self.generate_synchronicity_toml(&base_dir);
        self.generate_libra_configs(&base_dir, key_seed);
    }
}

impl InitCmd {
    /// Ensure the base directory exists
    pub fn prepare_base_dir(&self) -> PathBuf {
        if self.base_dir.len() != 1 {
            status_err!("usage: synchronicity init /path/to/syncronicity/base/dir");
            exit(1);
        }

        let base_dir = self.base_dir[0].canonicalize().unwrap_or_else(|e| {
            status_err!("couldn't canonicalize base dir: {}", e);
            exit(1);
        });

        // Create the scratch dir, which will ensure the parent also exists
        fs::create_dir_all(base_dir.join("scratch")).unwrap_or_else(|e| {
            status_err!("couldn't create base dir: {}", e);
            exit(1);
        });

        base_dir
    }

    /// Generate Synchronicity-specific config file (i.e. `synchronicity.toml`)
    pub fn generate_synchronicity_toml(&self, base_dir: &Path) {
        // TODO(tarcieri): better templating
        let config_data = format!(
            "# serendipity.toml: configuration file for Serendipity\n\
             node_config = \"{}\"\n\
             scratch_dir = \"{}\"\n\
             ",
            base_dir.join("node.config.toml").display(),
            base_dir.join("scratch/").display()
        );

        let config_path = base_dir.join(CONFIG_FILE);

        fs::write(&config_path, config_data).unwrap_or_else(|e| {
            status_err!("couldn't write {}: {}", config_path.display(), e);
            exit(1);
        });

        status_ok!("Generated", "{}", config_path.display());
    }

    /// Generate configuration files specific to Libra
    pub fn generate_libra_configs(&self, base_dir: &Path, key_seed: KeySeed) {
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
