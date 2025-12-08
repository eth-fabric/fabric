use axum::http::HeaderMap;
use eyre::{Result, eyre};
use reqwest::Client;

use constraints::{routes::LEGACY_SUBMIT_BLOCK, types::SubmitBlockRequestWithProofs};

#[derive(Clone)]
pub struct LegacyRelayClient {
    pub client: Client,
    pub base_url: String,
}

impl LegacyRelayClient {
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self { client, base_url })
    }

    pub async fn submit_block(
        &self,
        block_with_proofs: SubmitBlockRequestWithProofs,
        headers: HeaderMap,
    ) -> Result<()> {
        let url = format!("{}/{}", self.base_url, LEGACY_SUBMIT_BLOCK);

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

        // Convert block with proofs to block request without proofs
        let block = block_with_proofs.into_block_request();

        // Send block request without proofs to downstream relay
        let response = req.json(&block).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(eyre!(
                "Failed to submit block to downstream relay: {}",
                response.status()
            ))
        }
    }
}
