//! `start` subcommand

use abscissa_core::{Command, Options, Runnable};

/// `start` subcommand
#[derive(Command, Debug, Options)]
pub struct StartCmd {}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {}
}
