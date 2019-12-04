//! Synchronicity Configuration

use abscissa_core::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Synchronicity Configuration Filename
pub const CONFIG_FILE: &str = "synchronicity.toml";

/// Synchronicity Configuration
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SynchronicityConfig {
    /// Node config directory
    pub node_config: PathBuf,

    /// Scratch directory
    pub scratch_dir: PathBuf,
}
