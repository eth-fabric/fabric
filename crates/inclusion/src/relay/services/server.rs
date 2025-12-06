use std::sync::Arc;

use async_trait::async_trait;
use constraints::{
    api::ConstraintsApi,
    types::{
        ConstraintCapabilities, ConstraintsResponse, DelegationsResponse, SignedConstraints,
        SignedDelegation, SubmitBlockRequestWithProofs,
    },
};
use eyre::Result;

use crate::relay::state::RelayState;

#[derive(Clone)]
pub struct RelayServer {
    state: Arc<RelayState>,
}

impl RelayServer {
    pub fn new(state: Arc<RelayState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl ConstraintsApi for RelayServer {
    /// GET /capabilities
    async fn get_capabilities(&self) -> Result<ConstraintCapabilities> {
        todo!()
    }

    /// POST /constraints
    async fn post_constraints(&self, signed_constraints: SignedConstraints) -> Result<()> {
        todo!()
    }

    /// GET /constraints
    async fn get_constraints(&self, slot: u64) -> Result<ConstraintsResponse> {
        todo!()
    }

    /// POST /delegation
    async fn post_delegation(&self, signed_delegation: SignedDelegation) -> Result<()> {
        todo!()
    }

    /// GET /delegations/{slot}
    async fn get_delegations(&self, slot: u64) -> Result<DelegationsResponse> {
        todo!()
    }

    /// POST /blocks_with_proofs
    async fn post_blocks_with_proofs(
        &self,
        blocks_with_proofs: SubmitBlockRequestWithProofs,
    ) -> Result<()> {
        todo!()
    }

    /// GET /health
    async fn health_check(&self) -> Result<bool> {
        todo!()
    }
}
