//! `start` subcommand - launch Synchronicity node

// Parts adapted from upstream Libra's `main_node.rs`:
// <https://github.com/libra/libra/blob/master/libra-node/src/main_node.rs>
//
// Copyright (c) The Libra Core Contributors

use crate::{executor::SynchronicityExecutor, prelude::*, verifier::VerifyProvider};
use abscissa_core::{Command, Options, Runnable};
use synchro::{config::NodeConfig, Launcher, Node};

/// `start` subcommand
#[derive(Command, Debug, Options)]
pub struct StartCmd {}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let verify_provider = VerifyProvider::new();
        let launcher = Launcher::new(self.load_node_config(), verify_provider).unwrap();
        let _node: Node<SynchronicityExecutor> = launcher.launch().unwrap();
    }
}

impl StartCmd {
    /// Load the `NodeConfig`
    fn load_node_config(&self) -> NodeConfig {
        let cfg = app_config();
        cfg.load_node_config()
    }
}
