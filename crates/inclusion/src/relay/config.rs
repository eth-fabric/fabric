use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Address of the Relay server (constraints API)
    pub relay_addr: String,

    /// Path to the rocksdb database file location
    pub db_path: String,

    /// Module signing ID for this relay instance
    pub module_signing_id: String,

    /// Supported constraint types
    pub constraint_capabilities: Vec<u64>,

    /// Beacon API URL for fetching lookahead window
    pub beacon_api_url: String,

    /// How often to update the lookahead window
    pub lookahead_update_interval: u64,

    /// Optional downstream relay URL for proxying unhandled requests
    pub downstream_relay_url: String,
}
