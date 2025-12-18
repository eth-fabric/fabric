use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use axum::http::HeaderMap;
use eyre::{Result, eyre};
use reqwest::Client;

use constraints::routes::LEGACY_SUBMIT_BLOCK;
use tracing::info;

#[derive(Clone)]
pub struct LegacyRelayClient {
	pub client: Client,
	pub base_url: String,
}

impl LegacyRelayClient {
	pub fn new(base_url: String) -> Result<Self> {
		let client = Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
		let base_url = base_url.trim_end_matches('/').to_string();
		Ok(Self { client, base_url })
	}

	pub async fn submit_block(&self, block: AlloySubmitBlockRequest, headers: HeaderMap) -> Result<()> {
		let url = format!("{}/{}", self.base_url.trim_end_matches('/'), LEGACY_SUBMIT_BLOCK.trim_start_matches('/'));

		info!("Submitting block to downstream relay: {}", url);

		// Build downstream relay request
		let mut req = self.client.post(&url);

		// Forward relevant headers
		for (key, value) in headers.iter() {
			let key_str = key.as_str();
			if key_str != "host" && key_str != "connection" && !key_str.starts_with("x-forwarded") {
				if let Ok(val) = value.to_str() {
					req = req.header(key_str, val);
				}
			}
		}

		// Set content type
		req = req.header("Content-Type", "application/json");

		// Send block request to downstream relay
		let response = req.json(&block).send().await?;
		if response.status().is_success() {
			Ok(())
		} else {
			Err(eyre!("Failed to submit block to downstream relay: {}", response.status()))
		}
	}
}
