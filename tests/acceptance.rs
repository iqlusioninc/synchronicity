//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use abscissa_core::testing::prelude::*;
use std::{fs, path::Path};
use synchro::config::{ConsensusConfig, NetworkConfig, NodeConfig, PeerInfo, PersistableConfig};
use synchronicity::config::SynchronicityConfig;
use tempfile::tempdir;

#[test]
fn config_generator() {
    let tmp_dir = tempdir().unwrap();
    let dir = tmp_dir.path().canonicalize().unwrap();

    // Run `synchronicity init {dir}`
    run_synchronicity_init(&dir);

    // Ensure `synchronicity.toml` is valid
    let config = SynchronicityConfig::load_config(dir.join("synchronicity.toml"));
    assert_eq!(&config.node_config, &dir.join("node.config.toml"));
    assert_eq!(&config.scratch_dir, &dir.join("scratch"));

    // Make sure the scratch directory exists
    assert!(fs::metadata(&config.scratch_dir).unwrap().is_dir());

    // Make sure the generated Libra `node.config.toml` is valid
    let node_config = NodeConfig::load_config(&config.node_config);
    assert_eq!(
        dir.join(&node_config.consensus.consensus_keypair_file),
        dir.join("consensus_keypair.config.toml")
    );
    assert_eq!(
        dir.join(&node_config.networks[0].network_keypairs_file),
        dir.join("network_keypairs.config.toml")
    );

    // Make sure the generated Libra config files load
    ConsensusConfig::load_config(dir.join("consensus_keypair.config.toml"));
    NetworkConfig::load_config(dir.join("network_keypairs.config.toml"));
    PeerInfo::load_config(dir.join("peer_info.toml"));
}

/// Run `synchronicity init`
fn run_synchronicity_init(output_dir: &Path) {
    let mut runner = CmdRunner::default();
    let cmd = runner.arg("init").arg(output_dir).capture_stdout().run();

    cmd.wait().unwrap().expect_success();
}
