//! RPC specification and server trait for the Commitments service.
//!
//! This module defines:
//! - The wire-level RPC methods and their parameter / response types
//! - The `CommitmentsRpc` spec trait (using jsonrpsee's #[rpc(server)] macro)
//! - A reference handler struct `DefaultCommitmentsRpc<T>` that uses `CommitmentsServerState<T>`
//!
//! Implementations in other crates can:
//! - Reuse the `CommitmentsRpc` trait and param/response types
//! - Implement `CommitmentsRpcServer` for their own handler struct and state

use alloy::primitives::B256;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use crate::types::{CommitmentRequest, FeeInfo, SignedCommitment, SlotInfoResponse};

/// JSON RPC spec for the Commitments service.
/// Implementations are free to choose any internal state or dependencies.
/// jsonrpsee auto-generates a CommitmentsRpcServer when into_rpc() is called on this trait.
#[rpc(server, client)]
pub trait CommitmentsRpc {
    /// Request a commitment.
    #[method(name = "commitmentRequest")]
    async fn commitment_request(&self, request: CommitmentRequest) -> RpcResult<SignedCommitment>;

    /// Query a previously created commitment result.
    #[method(name = "commitmentResult")]
    async fn commitment_result(&self, request_hash: B256) -> RpcResult<SignedCommitment>;

    /// Query slots information.
    #[method(name = "slots")]
    async fn slots(&self) -> RpcResult<SlotInfoResponse>;

    /// Query current fee information.
    #[method(name = "fee")]
    async fn fee(&self, request: CommitmentRequest) -> RpcResult<FeeInfo>;
}
