//! Synchronicity Subcommands

mod start;
mod version;

use self::{start::StartCmd, version::VersionCmd};
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

    /// The `start` subcommand
    #[options(help = "start the application")]
    Start(StartCmd),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
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
