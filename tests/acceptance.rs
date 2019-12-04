//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use abscissa_core::testing::prelude::*;
use std::path::Path;
use synchro::config::{ConsensusConfig, NetworkConfig, NodeConfig, PeerInfo, PersistableConfig};
use tempfile::tempdir;

#[test]
fn config_generator() {
    let dir = tempdir().unwrap();

    // Run `synchrnonicity init {dir.path()}`
    run_synchronicity_init(dir.path());

    // Make sure the generated config files load
    ConsensusConfig::load_config(dir.path().join("consensus_keypair.config.toml"));
    NetworkConfig::load_config(dir.path().join("network_keypairs.config.toml"));
    NodeConfig::load_config(dir.path().join("node.config.toml"));
    PeerInfo::load_config(dir.path().join("peer_info.toml"));
}

/// Run `synchronicity init`
fn run_synchronicity_init(output_dir: &Path) {
    let mut runner = CmdRunner::default();
    let cmd = runner.arg("init").arg(output_dir).capture_stdout().run();

    cmd.wait().unwrap().expect_success();
}
