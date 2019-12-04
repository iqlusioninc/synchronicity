//! Synchronicity Subcommands

mod init;
mod start;
mod version;

use self::{init::InitCmd, start::StartCmd, version::VersionCmd};
use crate::config::SynchronicityConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

/// Synchronicity Configuration Filename
pub const CONFIG_FILE: &str = "synchronicity.toml";

/// Synchronicity Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum SynchronicityCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),

    /// The `init` subcommand
    #[options(help = "initialize application home/config")]
    Init(InitCmd),

    /// The `start` subcommand
    #[options(help = "start the application")]
    Start(StartCmd),
}

impl Configurable<SynchronicityConfig> for SynchronicityCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        let filename = PathBuf::from(CONFIG_FILE);

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }
}
