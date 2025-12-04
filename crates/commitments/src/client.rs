use alloy::primitives::B256;
use eyre::{Result, WrapErr};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

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
        CommitmentsRpcClient::commitment_request(&self.inner, request)
            .await
            .map_err(|e| eyre::eyre!("commitmentRequest RPC error: {e:?}"))
    }

    pub async fn commitment_result(&self, request_hash: B256) -> Result<SignedCommitment> {
        CommitmentsRpcClient::commitment_result(&self.inner, request_hash)
            .await
            .map_err(|e| eyre::eyre!("commitmentResult RPC error: {e:?}"))
    }

    pub async fn slots(&self) -> Result<SlotInfoResponse> {
        CommitmentsRpcClient::slots(&self.inner)
            .await
            .map_err(|e| eyre::eyre!("slots RPC error: {e:?}"))
    }

    pub async fn fee(&self, request: CommitmentRequest) -> Result<FeeInfo> {
        CommitmentsRpcClient::fee(&self.inner, request)
            .await
            .map_err(|e| eyre::eyre!("fee RPC error: {e:?}"))
    }
}
