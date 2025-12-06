use alloy::primitives::{Address, B256};
use commit_boost::prelude::{
    BlsPublicKey, Chain, StartCommitModuleConfig, commit::client::SignerClient,
};

use common::storage::DatabaseContext;
use constraints::client::HttpConstraintsClient;
use lookahead::{
    beacon_client::{BeaconApiClient, ReqwestClient},
    types::BeaconApiConfig,
};

use crate::proposer::config::ProposerConfig;

/// Server state that provides access to shared resources for proposer operations
#[derive(Clone)]
pub struct ProposerState {
    /// Storage
    pub db: DatabaseContext,
    /// Signer client for calling the signer API
    pub signer_client: SignerClient,
    /// Constraints client for sending constraints to the relay
    pub constraints_client: HttpConstraintsClient,
    /// Beacon client for fetching proposer duties
    pub beacon_client: BeaconApiClient<ReqwestClient>,
    /// Gateway delegate BLS public key
    pub gateway_public_key: BlsPublicKey,
    /// Gateway committer EOA address
    pub gateway_address: Address,
    /// Module signing ID for inclusion preconfs
    pub module_signing_id: B256,
    /// Chain ID
    pub chain: Chain,
    /// How often to check for new delegations
    pub lookahead_check_interval_seconds: u64,
}

impl ProposerState {
    pub fn new(db: DatabaseContext, config: StartCommitModuleConfig<ProposerConfig>) -> Self {
        // Create constraints client
        let constraints_client = HttpConstraintsClient::new(
            config.extra.relay_addr.to_string(),
            config.extra.relay_api_key.clone(),
        );

        // Create beacon client
        let beacon_client = BeaconApiClient::with_default_client(BeaconApiConfig {
            primary_endpoint: config.extra.beacon_api_url.to_string(),
            fallback_endpoints: vec![],
            request_timeout_secs: 30,
            genesis_time: config.chain.genesis_time_sec(),
        })
        .expect("Failed to create beacon client");

        let signer_client = config.signer_client.clone();

        let gateway_public_key =
            BlsPublicKey::deserialize(config.extra.gateway_public_key.as_bytes())
                .expect("Failed to deserialize gateway public key from config");
        let gateway_address = Address::try_from(config.extra.gateway_address.as_bytes())
            .expect("Failed to parse gateway address from config");

        let chain = config.chain;
        let module_signing_id = B256::from_slice(config.extra.module_signing_id.as_bytes());
        let lookahead_check_interval_seconds = config.extra.lookahead_check_interval_seconds;
        Self {
            db,
            signer_client,
            constraints_client,
            beacon_client,
            gateway_public_key,
            gateway_address,
            module_signing_id,
            chain,
            lookahead_check_interval_seconds,
        }
    }
}
