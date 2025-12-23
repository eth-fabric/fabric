use commit_boost::prelude::Chain;
use reqwest::{Client, Url};

use common::storage::DatabaseContext;
use constraints::{proxy::ProxyState, types::ConstraintCapabilities};
use lookahead::{
	beacon_client::{BeaconApiClient, ReqwestClient},
	types::BeaconApiConfig,
};

use crate::relay::{config::RelayConfig, services::proxy::LegacyRelayClient};

/// Server state that provides access to shared resources for gateway operations
#[derive(Clone)]
pub struct RelayState {
	/// Host of constraints server
	pub host: String,
	/// Port of constraints server
	pub port: u16,
	/// Storage
	pub db: DatabaseContext,
	/// Beacon client for fetching proposer duties
	pub beacon_client: BeaconApiClient<ReqwestClient>,
	/// Client to call downstream relay
	pub downstream_relay_client: LegacyRelayClient,
	/// Chain ID
	pub chain: Chain,
	/// How often to update the lookahead window
	pub lookahead_update_interval: u64,
	/// Supported constraint types
	pub constraint_capabilities: ConstraintCapabilities,
}

impl ProxyState for RelayState {
	fn server_url(&self) -> &str {
		&self.downstream_relay_client.base_url
	}

	fn http_client(&self) -> &Client {
		&self.downstream_relay_client.client
	}
}

impl RelayState {
	pub fn new(db: DatabaseContext, config: RelayConfig) -> Self {
		let chain = config.chain;
		let host = config.host;
		let port = config.port;

		// Create beacon client
		let beacon_client = BeaconApiClient::with_default_client(BeaconApiConfig {
			primary_endpoint: Url::parse(
				format!("http://{}:{}", config.beacon_api_host, config.beacon_api_port).as_str(),
			)
			.unwrap(),
			fallback_endpoints: vec![],
			request_timeout_secs: 30,
			genesis_time: chain.genesis_time_sec(),
		})
		.expect("Failed to create beacon client");

		// Create downstream relay client
		let downstream_relay_client =
			LegacyRelayClient::new(format!("http://{}:{}", config.downstream_relay_host, config.downstream_relay_port))
				.expect("Failed to create downstream relay client");

		let lookahead_update_interval = config.lookahead_update_interval;
		let constraint_capabilities = ConstraintCapabilities { constraint_types: config.constraint_capabilities };
		Self {
			db,
			host,
			port,
			beacon_client,
			chain,
			lookahead_update_interval,
			downstream_relay_client,
			constraint_capabilities,
		}
	}
}
