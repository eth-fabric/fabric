use alloy::primitives::B256;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use commit_boost::prelude::BlsPublicKey;

/// Gateway configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Address of the Commitments RPC server
    pub rpc_addr: SocketAddr,

    /// Path to the rocksdb database file location
    pub db_path: String,

    /// Address of the Relay server (constraints API)
    pub relay_addr: SocketAddr,
    pub relay_api_key: Option<String>,

    /// Execution client configuration
    pub execution_client: SocketAddr,

    /// Constraints receivers
    pub constraints_receivers: Vec<BlsPublicKey>,
    pub delegate_public_key: BlsPublicKey,

    /// Module signing ID for this gateway instance
    pub module_signing_id: B256,

    // Commitments-specific configuration
    pub log_level: String,
    pub enable_method_tracing: bool,
    pub traced_methods: Vec<String>,

    /// How often to check for new delegations
    pub delegation_check_interval_seconds: u64,
}
