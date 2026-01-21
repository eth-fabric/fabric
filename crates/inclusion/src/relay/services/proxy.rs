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
        let url = format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            LEGACY_SUBMIT_BLOCK.trim_start_matches('/')
        );

        info!("Submitting block to downstream relay: {}", url);

        // Build downstream relay request
        let mut req = self.client.post(&url);

        // IMPORTANT:
        // Since we are mutating the inbound request (stripping proof data) before forwarding.
        // We MUST NOT forward payload-specific headers such as Content-Length.
        //
        // Safer approach: allowlist only the headers we actually need.
        const ALLOWLIST: [&str; 3] = [
            "authorization",
            "user-agent",
            "x-request-id",
        ];

        for (name, value) in headers.iter() {
            let name_str = name.as_str();

            if !ALLOWLIST.iter().any(|h| h.eq_ignore_ascii_case(name_str)) {
                continue;
            }

            // HeaderValue -> &str conversion may fail for non-UTF8; skip those safely.
            if let Ok(val) = value.to_str() {
                req = req.header(name_str, val);
            }
        }

        // Do NOT set Content-Type manually; reqwest sets it for you when using .json().
        // Do NOT forward Content-Length; reqwest computes it for the outbound body.
        //
        // Send block request to downstream relay
        let response = req.json(&block).send().await?;

        if response.status().is_success() {
            // Drain the body to encourage clean connection reuse under load, even if you ignore it.
            let _ = response.bytes().await;
            Ok(())
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            Err(eyre!(
                "Failed to submit block to downstream relay: status={}, body={}",
                status,
                body
            ))
        }
    }
}
