use async_trait::async_trait;
use eyre::{Result, eyre};
use reqwest::{Client, Url};
use std::time::Duration;

use crate::metrics::client_http_metrics;
use crate::routes;
use crate::types::{
	ConstraintCapabilities, ConstraintsResponse, DelegationsResponse, SignedConstraints, SignedDelegation,
	SubmitBlockRequestWithProofs,
};

/// Trait for a Constraints REST client (mockable for testing).
///
/// This mirrors the server-side `ConstraintsApi` but uses references
/// for POST bodies to avoid unnecessary clones.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ConstraintsClient: Send + Sync {
	/// GET /capabilities
	async fn get_capabilities(&self) -> Result<ConstraintCapabilities>;

	/// POST /constraints
	async fn post_constraints(&self, signed_constraints: &SignedConstraints) -> Result<()>;

	/// GET /constraints/{slot}
	async fn get_constraints(&self, slot: u64) -> Result<Vec<SignedConstraints>>;

	/// POST /delegation
	async fn post_delegation(&self, signed_delegation: &SignedDelegation) -> Result<()>;

	/// GET /delegations/{slot}
	async fn get_delegations(&self, slot: u64) -> Result<Vec<SignedDelegation>>;

	/// POST /blocks_with_proofs
	async fn post_blocks_with_proofs(&self, blocks_with_proofs: &SubmitBlockRequestWithProofs) -> Result<()>;

	/// GET /health
	async fn health_check(&self) -> Result<bool>;
}

/// HTTP implementation of the Constraints client.
#[derive(Clone)]
pub struct HttpConstraintsClient {
	pub client: Client,
	pub base_url: Url,
	pub api_key: Option<String>,
}

impl HttpConstraintsClient {
	/// Create a new constraints client.
	pub fn new(host: String, port: u16, api_key: Option<String>) -> Self {
		let client = Client::builder().timeout(Duration::from_secs(30)).build().expect("Failed to create HTTP client");

		let base_url = Url::parse(format!("http://{}:{}", host, port).as_str()).expect("Failed to parse base URL");

		Self { client, base_url, api_key }
	}

	fn auth_header(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
		if let Some(api_key) = &self.api_key { req.header("Authorization", format!("Bearer {api_key}")) } else { req }
	}

	fn full_url(&self, endpoint: &str) -> String {
		// Strip leading slash from endpoint if present
		let endpoint = endpoint.trim_start_matches('/');
		format!("{}{}", self.base_url, endpoint)
	}
}

#[async_trait]
impl ConstraintsClient for HttpConstraintsClient {
	async fn get_capabilities(&self) -> Result<ConstraintCapabilities> {
		const ENDPOINT: &str = routes::CAPABILITIES;
		const METHOD: &str = "GET";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let url = self.full_url(ENDPOINT);

		let mut req = self.client.get(&url);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			let caps: ConstraintCapabilities = resp.json().await?;
			Ok(caps)
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to get capabilities (status {status}): {text}"))
		}
	}

	async fn post_constraints(&self, signed_constraints: &SignedConstraints) -> Result<()> {
		const ENDPOINT: &str = routes::CONSTRAINTS;
		const METHOD: &str = "POST";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let url = self.full_url(ENDPOINT);

		let mut req = self.client.post(&url).json(signed_constraints);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			Ok(())
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to post constraints (status {status}): {text}"))
		}
	}

	async fn get_constraints(&self, slot: u64) -> Result<Vec<SignedConstraints>> {
		const ENDPOINT: &str = routes::CONSTRAINTS_SLOT;
		const METHOD: &str = "GET";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let path = ENDPOINT.replace("{slot}", &slot.to_string());
		let url = self.full_url(&path);

		let mut req = self.client.get(&url);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			let result: ConstraintsResponse = resp.json().await?;
			Ok(result.constraints)
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to get constraints for slot {slot} (status {status}): {text}"))
		}
	}

	async fn post_delegation(&self, signed_delegation: &SignedDelegation) -> Result<()> {
		const ENDPOINT: &str = routes::DELEGATION;
		const METHOD: &str = "POST";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let url = self.full_url(ENDPOINT);

		let mut req = self.client.post(&url).json(signed_delegation);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			Ok(())
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to post delegation (status {status}): {text}"))
		}
	}

	async fn get_delegations(&self, slot: u64) -> Result<Vec<SignedDelegation>> {
		const ENDPOINT: &str = routes::DELEGATIONS_SLOT;
		const METHOD: &str = "GET";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let path = ENDPOINT.replace("{slot}", &slot.to_string());
		let url = self.full_url(&path);

		let mut req = self.client.get(&url);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			let result: DelegationsResponse = resp.json().await?;
			Ok(result.delegations)
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to get delegations for slot {slot} (status {status}): {text}"))
		}
	}

	async fn post_blocks_with_proofs(&self, blocks_with_proofs: &SubmitBlockRequestWithProofs) -> Result<()> {
		const ENDPOINT: &str = routes::BLOCKS_WITH_PROOFS;
		const METHOD: &str = "POST";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let url = self.full_url(ENDPOINT);

		let mut req = self.client.post(&url).json(blocks_with_proofs);
		req = self.auth_header(req);

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		if status.is_success() {
			Ok(())
		} else {
			let text = resp.text().await.unwrap_or_default();
			Err(eyre!("Failed to post blocks_with_proofs (status {status}): {text}"))
		}
	}

	async fn health_check(&self) -> Result<bool> {
		const ENDPOINT: &str = routes::HEALTH;
		const METHOD: &str = "GET";

		let metrics = client_http_metrics();
		let start = metrics.start(ENDPOINT, METHOD);

		let url = self.full_url(ENDPOINT);

		let req = self.client.get(&url).timeout(Duration::from_secs(5));

		let resp = match req.send().await {
			Ok(r) => r,
			Err(e) => {
				metrics.finish_label(ENDPOINT, METHOD, "error", start);
				return Err(e.into());
			}
		};

		let status = resp.status();
		metrics.finish_status(ENDPOINT, METHOD, status.as_u16(), start);

		Ok(status.is_success())
	}
}
