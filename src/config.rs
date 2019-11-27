//! Synchronicity Configuration

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

/// Synchronicity Configuration
#[derive(Clone, Config, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SynchronicityConfig {}
