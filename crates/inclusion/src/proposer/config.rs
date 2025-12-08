use serde::Deserialize;

/// Configuration for the proposer service
#[derive(Debug, Clone, Deserialize)]
pub struct ProposerConfig {
    /// Path to the RocksDB database for storing delegations (for equivocation prevention)
    pub db_path: String,

    /// Gateway delegate BLS public key
    pub gateway_public_key: String,

    /// Gateway committer EOA address
    pub gateway_address: String,

    /// Address of the Relay server (constraints API)
    pub relay_host: String,

    /// Port of the Relay server (constraints API)
    pub relay_port: u16,

    /// API key for the Relay server (constraints API)
    pub relay_api_key: Option<String>,

    /// Beacon API URL for fetching proposer duties
    pub beacon_api_url: String,

    /// How often to poll for proposer duties (in seconds)
    pub lookahead_check_interval_seconds: u64,

    /// Module signing ID for this proposer instance
    pub module_signing_id: String,
}
