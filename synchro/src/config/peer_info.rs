//! `peer_info.toml` files used to build devnet genesis/configuration

use libra_config::trusted_peers::{ConsensusPeerInfo, NetworkPeerInfo};
use serde::{Deserialize, Serialize};

/// Name of the `PeerInfo` file
pub const DEFAULT_FILENAME: &str = "peer_info.toml";

/// Public keys for a particular network peer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Peer ID
    pub id: String,

    /// Optional description for this peer
    #[serde(default)]
    pub description: String,

    /// Optional web site URL
    #[serde(default)]
    pub website_url: String,

    /// Optional logo URL
    #[serde(default)]
    pub logo_url: String,

    /// Seed IP address to include in `peer_info.toml`
    pub seed_ip: Option<String>,

    /// Consensus peer information
    pub consensus: ConsensusPeerInfo,

    /// Network peer information
    pub network: NetworkPeerInfo,
}

impl PeerInfo {
    /// Create new `PeerInfo` from the given peer ID, consensus and network peer info
    pub fn new(
        peer_id: impl ToString,
        seed_ip: Option<impl ToString>,
        consensus: ConsensusPeerInfo,
        network: NetworkPeerInfo,
    ) -> Self {
        Self {
            id: peer_id.to_string(),
            description: Default::default(),
            website_url: Default::default(),
            logo_url: Default::default(),
            seed_ip: seed_ip.map(|ip| ip.to_string()),
            consensus,
            network,
        }
    }
}
