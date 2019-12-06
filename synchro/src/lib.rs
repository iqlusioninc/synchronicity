//! Synchro

// Modules
pub mod config;
pub mod crypto;
pub mod error;
pub mod launcher;
pub mod node;
pub mod transaction;

// Crate re-exports
pub use grpcio;

// Libra re-exports
pub use consensus;
pub use executor;
pub use futures;
pub use libra_mempool as mempool;
pub use libra_state_view as state_view;
pub use libra_types as types;
pub use network;
pub use state_synchronizer;
pub use storage_client;
pub use vm_runtime;
pub use vm_validator;

// Other re-exports
pub use tokio;

pub use self::{launcher::Launcher, node::Node};

/// Helper to initialize a Tokio runtime
pub fn start_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new()
        .name_prefix("synchro-")
        .build()
        .unwrap_or_else(|e| panic!("couldn't initialize Tokio runtime: {}", e))
}
