use alloy::primitives::B256;
use eyre::{Result, WrapErr};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

use crate::methods::{
    COMMITMENT_REQUEST_METHOD, COMMITMENT_RESULT_METHOD, FEE_METHOD, SLOTS_METHOD,
};
use crate::metrics::client_http_metrics;
use crate::rpc::CommitmentsRpcClient;
use crate::types::{CommitmentRequest, FeeInfo, SignedCommitment, SlotInfoResponse};

/// Thin wrapper around `HttpClient` that exposes typed methods for the Commitments RPC API.
#[derive(Clone)]
pub struct CommitmentsHttpClient {
    inner: HttpClient,
}

impl CommitmentsHttpClient {
    /// Create a new HTTP client pointed at the given base URL.
    ///
    /// Example:
    /// ```ignore
    /// let client = CommitmentsHttpClient::new("http://127.0.0.1:8545")?;
    /// ```
    pub fn new<S: AsRef<str>>(url: S) -> Result<Self> {
        let inner = HttpClientBuilder::default()
            .build(url.as_ref())
            .wrap_err_with(|| format!("failed to build HttpClient for url {}", url.as_ref()))?;

        Ok(Self { inner })
    }

    /// Expose inner if needed
    pub fn inner(&self) -> &HttpClient {
        &self.inner
    }

    pub async fn commitment_request(&self, request: CommitmentRequest) -> Result<SignedCommitment> {
        const ROLE: &str = "client";
        const METHOD: &str = COMMITMENT_REQUEST_METHOD;

        let metrics = client_http_metrics();
        let start = metrics.start(ROLE, METHOD);

        // Make the actual RPC call
        let result = CommitmentsRpcClient::commitment_request(&self.inner, request).await;

        match result {
            Ok(resp) => {
                metrics.finish_label(ROLE, METHOD, "ok", start);
                Ok(resp.into())
            }
            Err(e) => {
                // map your Error -> RpcError, etc
                metrics.finish_label(ROLE, METHOD, format!("error: {e:?}").as_str(), start);
                Err(e.into())
            }
        }
    }

    pub async fn commitment_result(&self, request_hash: B256) -> Result<SignedCommitment> {
        const ROLE: &str = "client";
        const METHOD: &str = COMMITMENT_RESULT_METHOD;

        let metrics = client_http_metrics();
        let start = metrics.start(ROLE, METHOD);

        let result = CommitmentsRpcClient::commitment_result(&self.inner, request_hash).await;

        match result {
            Ok(resp) => {
                metrics.finish_label(ROLE, METHOD, "ok", start);
                Ok(resp.into())
            }
            Err(e) => {
                metrics.finish_label(ROLE, METHOD, format!("error: {e:?}").as_str(), start);
                Err(e.into())
            }
        }
    }

    pub async fn slots(&self) -> Result<SlotInfoResponse> {
        const ROLE: &str = "client";
        const METHOD: &str = SLOTS_METHOD;

        let metrics = client_http_metrics();
        let start = metrics.start(ROLE, METHOD);

        let result = CommitmentsRpcClient::slots(&self.inner).await;

        match result {
            Ok(resp) => {
                metrics.finish_label(ROLE, METHOD, "ok", start);
                Ok(resp)
            }
            Err(e) => {
                metrics.finish_label(ROLE, METHOD, format!("error: {e:?}").as_str(), start);
                Err(e.into())
            }
        }
    }

    pub async fn fee(&self, request: CommitmentRequest) -> Result<FeeInfo> {
        const ROLE: &str = "client";
        const METHOD: &str = FEE_METHOD;

        let metrics = client_http_metrics();
        let start = metrics.start(ROLE, METHOD);

        let result = CommitmentsRpcClient::fee(&self.inner, request).await;

        match result {
            Ok(resp) => {
                metrics.finish_label(ROLE, METHOD, "ok", start);
                Ok(resp)
            }
            Err(e) => {
                metrics.finish_label(ROLE, METHOD, format!("error: {e:?}").as_str(), start);
                Err(e.into())
            }
        }
    }
}
