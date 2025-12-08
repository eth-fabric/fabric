use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use eyre::{Result, eyre};
use reqwest::Client;
use tracing::{error, info};

use constraints::{routes::LEGACY_SUBMIT_BLOCK, types::SubmitBlockRequestWithProofs};

use crate::relay::state::RelayState;

#[derive(Clone)]
pub struct LegacyRelayClient {
    client: Client,
    base_url: String,
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

/// Proxy handler for forwarding unmatched requests to downstream relay
async fn proxy_handler(
    State(state): State<RelayState>,
    req: Request,
) -> Result<Response, StatusCode> {
    info!("proxying request to downstream relay");

    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path();
    let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();

    // Build downstream relay URL
    let downstream_full_url = format!(
        "{}{}{}",
        state.downstream_relay_client.base_url, path, query
    );
    info!("Proxying {} {} to {}", method, path, downstream_full_url);

    // Extract headers and body
    let headers = req.headers().clone();
    let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read request body: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Build downstream request
    let mut req = state
        .downstream_relay_client
        .client
        .request(method.clone(), &downstream_full_url);

    // Forward headers (excluding host and connection-related headers)
    for (key, value) in headers.iter() {
        let key_str = key.as_str();
        if key_str != "host" && key_str != "connection" && !key_str.starts_with("x-forwarded") {
            if let Ok(val) = value.to_str() {
                req = req.header(key_str, val);
            }
        }
    }

    // Add body if present
    if !body_bytes.is_empty() {
        req = req.body(body_bytes.to_vec());
    }

    // Send request to
    let response = match req.send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to proxy request to downstream relay: {}", e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    // Build response
    let status = response.status();
    let mut response_builder = Response::builder().status(status);

    // Copy response headers
    for (key, value) in response.headers() {
        response_builder = response_builder.header(key, value);
    }

    // Get response body
    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read downstream relay response body: {}", e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    // Build final response
    match response_builder.body(Body::from(body_bytes)) {
        Ok(response) => {
            info!("Proxy response: {} for {} {}", status, method, path);
            Ok(response)
        }
        Err(e) => {
            error!("Failed to build response: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
