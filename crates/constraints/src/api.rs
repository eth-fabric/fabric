use crate::types::{
    AuthorizationContext, ConstraintCapabilities, ConstraintsResponse, DelegationsResponse,
    SignedConstraints, SignedDelegation, SubmitBlockRequestWithProofs,
};
use async_trait::async_trait;
use axum::http::HeaderMap;
use eyre::Result;

/// Server side spec for the Constraints REST API.
///
/// Any implementation can use any internal state (DB,
/// RPC clients, etc) as long as it implements this.
#[async_trait]
pub trait ConstraintsApi: Send + Sync + Clone + 'static {
    /// GET /capabilities
    async fn get_capabilities(&self) -> Result<ConstraintCapabilities>;

    /// POST /constraints
    async fn post_constraints(&self, signed_constraints: SignedConstraints) -> Result<()>;

    /// GET /constraints
    async fn get_constraints(
        &self,
        slot: u64,
        auth: AuthorizationContext,
    ) -> Result<ConstraintsResponse>;

    /// POST /delegation
    async fn post_delegation(&self, signed_delegation: SignedDelegation) -> Result<()>;

    /// GET /delegations/{slot}
    async fn get_delegations(&self, slot: u64) -> Result<DelegationsResponse>;

    /// POST /blocks_with_proofs
    async fn post_blocks_with_proofs(
        &self,
        block_request: SubmitBlockRequestWithProofs,
        headers: HeaderMap,
    ) -> Result<()>;

    /// GET /health
    async fn health_check(&self) -> Result<()>;
}
