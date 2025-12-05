use alloy::primitives::B256;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use std::sync::Arc;

use super::state::GatewayState;
use commitments::rpc::CommitmentsRpcServer;
use commitments::types::{CommitmentRequest, FeeInfo, SignedCommitment, SlotInfoResponse};

#[derive(Clone)]
pub struct GatewayRpc {
    pub state: Arc<GatewayState>,
}

impl GatewayRpc {
    pub fn new(state: Arc<GatewayState>) -> Self {
        Self { state }
    }
}

/// Implementation of the CommitmentsRpcServer for inclusion preconfs
#[async_trait]
impl CommitmentsRpcServer for GatewayRpc {
    async fn commitment_request(&self, request: CommitmentRequest) -> RpcResult<SignedCommitment> {
        todo!()
    }

    /// Query a previously created commitment result.
    async fn commitment_result(&self, request_hash: B256) -> RpcResult<SignedCommitment> {
        todo!()
    }

    /// Query slots information.
    async fn slots(&self) -> RpcResult<SlotInfoResponse> {
        todo!()
    }

    /// Query current fee information.
    async fn fee(&self, request: CommitmentRequest) -> RpcResult<FeeInfo> {
        todo!()
    }
}
