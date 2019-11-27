//! Synchronicity Abscissa Application

use crate::{commands::SynchronicityCmd, config::SynchronicityConfig};
use abscissa_core::{
    application, config, logging, Application, EntryPoint, FrameworkError, StandardPaths,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APPLICATION: application::Lock<SynchronicityApp> = application::Lock::default();
}

pub fn app_reader() -> application::lock::Reader<SynchronicityApp> {
    APPLICATION.read()
}

pub fn app_writer() -> application::lock::Writer<SynchronicityApp> {
    APPLICATION.write()
}

pub fn app_config() -> config::Reader<SynchronicityApp> {
    config::Reader::new(&APPLICATION)
}

/// Synchronicity Application
#[derive(Debug)]
pub struct SynchronicityApp {
    /// Application configuration.
    config: Option<SynchronicityConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
impl Default for SynchronicityApp {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for SynchronicityApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint<SynchronicityCmd>;

    /// Application configuration.
    type Cfg = SynchronicityConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> &SynchronicityConfig {
        self.config.as_ref().expect("config not loaded")
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut application::State<Self> {
        &mut self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// Get logging configuration from command-line options
    fn logging_config(&self, command: &EntryPoint<SynchronicityCmd>) -> logging::Config {
        if command.verbose {
            logging::Config::verbose()
        } else {
            logging::Config::default()
        }
    }
}
