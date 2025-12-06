use alloy::primitives::B256;
use commit_boost::prelude::{Chain, StartCommitModuleConfig};

use common::storage::DatabaseContext;
use constraints::types::ConstraintCapabilities;
use lookahead::{
    beacon_client::{BeaconApiClient, ReqwestClient},
    types::BeaconApiConfig,
};

use crate::relay::config::RelayConfig;

/// Server state that provides access to shared resources for gateway operations
#[derive(Clone)]
pub struct RelayState {
    /// Storage
    pub db: DatabaseContext,
    /// Beacon client for fetching proposer duties
    pub beacon_client: BeaconApiClient<ReqwestClient>,
    /// Module signing ID for inclusion preconfs
    pub module_signing_id: B256,
    /// Chain ID
    pub chain: Chain,
    /// How often to update the lookahead window
    pub lookahead_update_interval: u64,
    /// Optional downstream relay URL for proxying unhandled requests
    pub downstream_relay_url: String,
    /// Supported constraint types
    pub constraint_capabilities: ConstraintCapabilities,
}

impl RelayState {
    pub fn new(db: DatabaseContext, config: StartCommitModuleConfig<RelayConfig>) -> Self {
        // Create beacon client
        let beacon_client = BeaconApiClient::with_default_client(BeaconApiConfig {
            primary_endpoint: config.extra.beacon_api_url.to_string(),
            fallback_endpoints: vec![],
            request_timeout_secs: 30,
            genesis_time: config.chain.genesis_time_sec(),
        })
        .expect("Failed to create beacon client");
        let chain = config.chain;
        let module_signing_id = B256::from_slice(config.extra.module_signing_id.as_bytes());
        let lookahead_update_interval = config.extra.lookahead_update_interval;
        let downstream_relay_url = config.extra.downstream_relay_url;
        let constraint_capabilities = ConstraintCapabilities {
            constraint_types: config.extra.constraint_capabilities,
        };
        Self {
            db,
            beacon_client,
            chain,
            module_signing_id,
            lookahead_update_interval,
            downstream_relay_url,
            constraint_capabilities,
        }
    }
}
