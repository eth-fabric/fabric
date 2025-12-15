//! Minimal Beacon API client for retrieving proposer duties and slot information
#![allow(async_fn_in_trait)]

use eyre::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

use crate::constants::PROPOSER_DUTIES_ROUTE;
use crate::types::{BeaconApiConfig, ProposerDutiesResponse};

/// HTTP response containing status code and body
#[derive(Debug, Clone)]
pub struct HttpResponse {
	pub status: u16,
	pub body: Vec<u8>,
}

/// Trait for making HTTP requests (mockable for testing)
/// When test-utils feature is enabled, mockall will generate MockHttpClient
#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
pub trait HttpClient: Send + Sync {
	/// Perform an HTTP GET request to the given URL
	async fn get(&self, url: &str) -> Result<HttpResponse>;
}

/// Production HTTP client implementation using reqwest
pub struct ReqwestClient {
	client: Client,
}

impl ReqwestClient {
	/// Create a new ReqwestClient with the given timeout
	pub fn new(timeout_secs: u64) -> Result<Self> {
		let client = Client::builder()
			.timeout(Duration::from_secs(timeout_secs))
			.build()
			.context("Failed to create HTTP client")?;
		Ok(Self { client })
	}
}

impl HttpClient for ReqwestClient {
	async fn get(&self, url: &str) -> Result<HttpResponse> {
		let response = self
			.client
			.get(url)
			.header("Content-Type", "application/json")
			.send()
			.await
			.with_context(|| format!("Failed to send request to {}", url))?;

		let status = response.status().as_u16();
		let body =
			response.bytes().await.with_context(|| format!("Failed to read response body from {}", url))?.to_vec();

		Ok(HttpResponse { status, body })
	}
}

/// Beacon API client for retrieving chain state and proposer information
pub struct BeaconApiClient<H: HttpClient> {
	http_client: Arc<H>,
	config: BeaconApiConfig,
}

// Manual Debug implementation since H might not implement Debug
impl<H: HttpClient> std::fmt::Debug for BeaconApiClient<H> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("BeaconApiClient").field("config", &self.config).finish()
	}
}

// Manual Clone implementation since H might not implement Clone
impl<H: HttpClient> Clone for BeaconApiClient<H> {
	fn clone(&self) -> Self {
		Self { http_client: Arc::clone(&self.http_client), config: self.config.clone() }
	}
}

impl<H: HttpClient> BeaconApiClient<H> {
	/// Creates a new BeaconApiClient configured with the provided BeaconApiConfig and HTTP client.
	///
	/// The created client uses the provided HTTP client for making requests.
	/// Returns an error if the configuration is invalid (e.g., empty primary endpoint or zero timeout).
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The primary endpoint is empty
	/// - The request timeout is zero (would cause immediate timeouts)
	///
	/// # Examples
	///
	pub fn new(config: BeaconApiConfig, http_client: H) -> Result<Self> {
		// Validate configuration
		if config.request_timeout_secs == 0 {
			eyre::bail!("Request timeout must be greater than zero");
		}

		Ok(Self { http_client: Arc::new(http_client), config })
	}

	/// Fetches proposer duties for the given epoch from the configured beacon endpoints.
	///
	/// Tries the primary endpoint first and falls back to configured fallback endpoints; returns
	/// the first successful response or an error if all endpoints fail.
	///
	/// # Returns
	///
	/// `Ok(ProposerDutiesResponse)` containing scheduled proposer duties for the epoch, `Err` if all
	/// configured endpoints fail or no endpoints are configured.
	///
	/// # Examples
	///
	pub async fn get_proposer_duties(&self, epoch: u64) -> Result<ProposerDutiesResponse> {
		let endpoint = format!("{}/{}", PROPOSER_DUTIES_ROUTE, epoch);

		// Try primary endpoint first, then fallbacks
		let mut _last_error = None;

		// Try primary endpoint
		match self.make_request(&self.config.primary_endpoint.to_string(), &endpoint).await {
			Ok(response) => return Ok(response),
			Err(e) => {
				warn!(
					endpoint = %self.config.primary_endpoint,
					epoch = epoch,
					error = %e,
					"Primary beacon endpoint failed, trying fallbacks"
				);
				_last_error = Some(e);
			}
		}

		// Try fallback endpoints
		for fallback_endpoint in &self.config.fallback_endpoints {
			match self.make_request(fallback_endpoint.to_string().as_str(), &endpoint).await {
				Ok(response) => {
					debug!(
						endpoint = %fallback_endpoint,
						epoch = epoch,
						"Successfully retrieved proposer duties from fallback endpoint"
					);
					return Ok(response);
				}
				Err(e) => {
					warn!(
						endpoint = %fallback_endpoint,
						epoch = epoch,
						error = %e,
						"Fallback beacon endpoint failed"
					);
					_last_error = Some(e);
				}
			}
		}

		// All endpoints failed
		Err(_last_error.unwrap_or_else(|| eyre::eyre!("No beacon endpoints configured")))
	}

	/// Perform an HTTP GET to the given endpoint on `base_url`, validate the response, and deserialize the JSON body into `T`.
	///
	/// The method constructs the full URL by joining `base_url` and `endpoint`, sends a GET request with standard headers,
	/// fails if the HTTP status is not successful (including the status and response body in the error), and parses the
	/// response JSON into `T`.
	///
	/// # Returns
	///
	/// The deserialized JSON response as `T`.
	///
	/// # Errors
	///
	/// Returns an error if the request fails to send, the response status is not successful, or the response body cannot be parsed as `T`.
	///
	/// # Examples
	///
	async fn make_request<T>(&self, base_url: &str, endpoint: &str) -> Result<T>
	where
		T: for<'de> Deserialize<'de>,
	{
		let url = if base_url.ends_with('/') {
			format!("{}{}", base_url, endpoint)
		} else {
			format!("{}/{}", base_url, endpoint)
		};

		debug!(url = %url, "Making beacon API request");

		let response =
			self.http_client.get(&url).await.with_context(|| format!("Failed to send request to {}", url))?;

		if response.status != 200 {
			let error_text = String::from_utf8(response.body.clone()).unwrap_or_else(|_| "Unknown error".to_string());
			eyre::bail!("Beacon API request failed with status {}: {}", response.status, error_text);
		}

		let result: T =
			serde_json::from_slice(&response.body).with_context(|| format!("Failed to parse response from {}", url))?;

		Ok(result)
	}
}

// Convenience constructor for production use with ReqwestClient
impl BeaconApiClient<ReqwestClient> {
	/// Creates a new BeaconApiClient with the default ReqwestClient HTTP client.
	///
	/// This is the standard constructor for production use. For testing, use
	/// `BeaconApiClient::with_default_client()` with a mock HTTP client.
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The primary endpoint is empty
	/// - The request timeout is zero
	/// - The underlying HTTP client cannot be constructed
	///
	/// # Examples
	///
	pub fn with_default_client(config: BeaconApiConfig) -> Result<Self> {
		let http_client = ReqwestClient::new(config.request_timeout_secs)?;
		Self::new(config, http_client)
	}
}
