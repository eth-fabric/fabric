use serde::{Deserialize, Serialize};

/// Gateway configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Address of the Commitments RPC server
    pub rpc_host: String,

    /// Port of the Commitments RPC server
    pub rpc_port: u16,

    /// Host of the metrics server
    pub metrics_host: String,

    /// Port of the metrics server
    pub metrics_port: u16,

    /// Path to the rocksdb database file location
    pub db_path: String,

    /// Host of the Relay server (constraints API)
    pub relay_host: String,

    /// Port of the Relay server (constraints API)
    pub relay_port: u16,

    /// API key for the Relay server (constraints API)
    pub relay_api_key: Option<String>,

    /// Host of the Execution client
    pub execution_client_host: String,

    /// Port of the Execution client
    pub execution_client_port: u16,

    /// Constraints receivers
    pub constraints_receivers: Vec<String>,

    /// Module signing ID for this gateway instance
    pub module_signing_id: String,

    // Commitments-specific configuration
    pub log_level: String,

    /// How often to check for new delegations
    pub delegation_check_interval_seconds: u64,

    /// Gateway public key for signing constraints
    pub gateway_public_key: String,
}
