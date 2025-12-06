use alloy::{
    network::Ethereum,
    primitives::B256,
    providers::{DynProvider, Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use commit_boost::prelude::{
    BlsPublicKey, Chain, StartCommitModuleConfig, commit::client::SignerClient,
};

use common::storage::DatabaseContext;
use constraints::client::HttpConstraintsClient;

use crate::gateway::config::GatewayConfig;

/// Server state that provides access to shared resources for gateway operations
#[derive(Clone)]
pub struct GatewayState {
    /// Storage
    pub db: DatabaseContext,
    /// Signer client for calling the signer API
    pub signer_client: SignerClient,
    /// Constraints client for sending constraints to the relay
    pub constraints_client: HttpConstraintsClient,
    /// Execution client for pricing
    pub execution_client: DynProvider<Ethereum>,
    /// Gateway public key for signing constraints
    pub gateway_public_key: BlsPublicKey,
    /// Constraints receivers whitelist
    pub constraints_receivers: Vec<BlsPublicKey>,
    /// Module signing ID for inclusion preconfs
    pub module_signing_id: B256,
    /// Chain ID
    pub chain: Chain,
    /// How often to check for new delegations
    pub delegation_check_interval_seconds: u64,
}

impl GatewayState {
    pub fn new(db: DatabaseContext, config: StartCommitModuleConfig<GatewayConfig>) -> Self {
        // Create constraints client
        let constraints_client = HttpConstraintsClient::new(
            config.extra.relay_addr.to_string(),
            config.extra.relay_api_key.clone(),
        );

        // Create execution client
        let execution_client_url = Url::parse(config.extra.execution_client.as_str())
            .expect("Failed to parse execution client URL from config");
        let execution_client = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http(execution_client_url)
            .erased();

        // Parse config fields into their respective types
        let signer_client = config.signer_client.clone();

        let gateway_public_key =
            BlsPublicKey::deserialize(config.extra.gateway_public_key.as_bytes())
                .expect("Failed to deserialize gateway public key from config");

        let constraints_receivers = config
            .extra
            .constraints_receivers
            .iter()
            .map(|receiver| {
                BlsPublicKey::deserialize(receiver.as_bytes())
                    .expect("Failed to deserialize constraints receiver from config")
            })
            .collect::<Vec<_>>();

        let chain = config.chain;
        let module_signing_id = B256::from_slice(config.extra.module_signing_id.as_bytes());
        let delegation_check_interval_seconds = config.extra.delegation_check_interval_seconds;
        Self {
            db,
            signer_client,
            constraints_client,
            execution_client,
            gateway_public_key,
            constraints_receivers,
            chain,
            module_signing_id,
            delegation_check_interval_seconds,
        }
    }
}
