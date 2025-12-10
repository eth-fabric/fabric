use std::net::{IpAddr, SocketAddr};

use alloy::{
    hex,
    network::Ethereum,
    primitives::B256,
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::beacon::BlsPublicKey,
    transports::http::reqwest::Url,
};
use commit_boost::prelude::{Chain, StartCommitModuleConfig, commit::client::SignerClient};

use common::{storage::DatabaseContext, utils::decode_pubkey};
use constraints::client::HttpConstraintsClient;

use crate::gateway::config::GatewayConfig;

/// Server state that provides access to shared resources for gateway operations
#[derive(Clone)]
pub struct GatewayState {
    /// Address of the Commitments RPC server
    pub rpc_addr: SocketAddr,
    /// Path to the rocksdb database file location
    pub metrics_addr: SocketAddr,
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
            config
                .extra
                .relay_host
                .parse::<IpAddr>()
                .expect("Failed to parse relay host"),
            config.extra.relay_port,
            config.extra.relay_api_key.clone(),
        );

        let rpc_addr = format!("{}:{}", config.extra.rpc_host, config.extra.rpc_port)
            .parse::<SocketAddr>()
            .expect("Failed to parse RPC address");

        let metrics_addr = format!(
            "{}:{}",
            config.extra.metrics_host, config.extra.metrics_port
        )
        .parse::<SocketAddr>()
        .expect("Failed to parse metrics address");

        // Create execution client
        let execution_client_addr = format!(
            "{}:{}",
            config.extra.execution_client_host, config.extra.execution_client_port
        )
        .parse::<SocketAddr>()
        .expect("Failed to parse execution client address");
        let execution_client_url =
            Url::parse(format!("http://{}", execution_client_addr.to_string()).as_str())
                .expect("Failed to parse execution client URL from config");
        let execution_client = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http(execution_client_url)
            .erased();

        // Parse config fields into their respective types
        let signer_client = config.signer_client.clone();

        let gateway_public_key = decode_pubkey(config.extra.gateway_public_key.as_str())
            .expect("Failed to decode gateway public key");

        let constraints_receivers = config
            .extra
            .constraints_receivers
            .iter()
            .map(|receiver| {
                decode_pubkey(receiver.as_str()).expect("Failed to decode constraints receiver")
            })
            .collect::<Vec<_>>();

        let chain = config.chain;
        let module_signing_id = B256::from_slice(
            &hex::decode(config.extra.module_signing_id.as_str())
                .expect("Failed to decode module signing id"),
        );
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
            rpc_addr,
            metrics_addr,
        }
    }
}
