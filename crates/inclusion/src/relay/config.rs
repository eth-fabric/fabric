use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Chain name
    pub chain: String,

    /// Host of the Relay server (constraints API)
    pub host: String,

    /// Port of the Relay server (constraints API)
    pub port: u16,

    /// Path to the rocksdb database file location
    pub db_path: String,

    /// Supported constraint types
    pub constraint_capabilities: Vec<u64>,

    /// Host of the Beacon API for fetching proposer duties
    pub beacon_api_host: String,

    /// Port of the Beacon API for fetching proposer duties
    pub beacon_api_port: u16,

    /// How often to update the lookahead window
    pub lookahead_update_interval: u64,

    /// Downstream relay URL for proxying unhandled requests
    pub downstream_relay_url: String,
}
