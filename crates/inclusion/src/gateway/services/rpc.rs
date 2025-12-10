use alloy::primitives::B256;
use async_trait::async_trait;
use commitments::server::CommitmentsServerInfo;
use jsonrpsee::core::RpcResult;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use commitments::rpc::CommitmentsRpcServer;
use commitments::types::{
    CommitmentRequest, FeeInfo, Offering, SignedCommitment, SlotInfo, SlotInfoResponse,
};
use lookahead::utils::current_slot;

use crate::constants::{INCLUSION_COMMITMENT_TYPE, LOOKAHEAD_WINDOW_SIZE};
use crate::gateway::state::GatewayState;
use crate::gateway::utils;
use crate::storage::{DelegationsDbExt, InclusionDbExt};

#[derive(Clone)]
pub struct GatewayRpc {
    pub state: Arc<GatewayState>,
}

impl GatewayRpc {
    pub fn new(state: Arc<GatewayState>) -> Self {
        Self { state }
    }
}

impl CommitmentsServerInfo for GatewayRpc {
    fn server_addr(&self) -> SocketAddr {
        self.state.rpc_addr
    }
    fn metrics_addr(&self) -> SocketAddr {
        self.state.metrics_addr
    }
}

/// Implementation of the CommitmentsRpcServer for inclusion preconfs
#[async_trait]
impl CommitmentsRpcServer for GatewayRpc {
    async fn commitment_request(&self, request: CommitmentRequest) -> RpcResult<SignedCommitment> {
        // Parse the inclusion payload
        let inclusion_payload = utils::validate_commitment_request(&request).map_err(|e| {
            jsonrpsee::types::error::ErrorObject::owned(
                -32602, // Invalid params
                "Invalid commitment request",
                Some(format!("{}", e)),
            )
        })?;
        debug!(
            "Validated inclusion payload for slot {}",
            inclusion_payload.slot
        );

        // Get the *singular* valid signed delegation for the slot
        // Error if none exists for this gateway
        let signed_delegation = self
            .state
            .db
            .get_delegation(inclusion_payload.slot)
            .map_err(|e| {
                jsonrpsee::types::error::ErrorObject::owned(
                    -32602, // Invalid params
                    "No delegation for slot",
                    Some(format!("{}", e)),
                )
            })?
            .ok_or(jsonrpsee::types::error::ErrorObject::owned(
                -32602, // Invalid params
                "No delegation for slot",
                Some(format!(
                    "No delegation found for slot {}",
                    inclusion_payload.slot
                )),
            ))?;
        debug!(
            "Found signed delegation for slot {}",
            inclusion_payload.slot
        );

        // Sign the commitment using ECDSA key for "committer" address
        let signed_commitment = utils::create_signed_commitment(
            &request,
            &mut self.state.signer_client.clone(),
            signed_delegation.message.committer,
            &self.state.module_signing_id,
            self.state.chain,
        )
        .await
        .map_err(|e| {
            jsonrpsee::types::error::ErrorObject::owned(
                -32603, // Internal error
                "Failed to create signed commitment",
                Some(format!("{}", e)),
            )
        })?;
        debug!(
            "Created signed commitment for slot {}",
            inclusion_payload.slot
        );

        // Create the corresponding constraint
        let constraint =
            utils::create_constraint_from_commitment_request(&request, inclusion_payload.slot)
                .map_err(|e| {
                    jsonrpsee::types::error::ErrorObject::owned(
                        -32603, // Internal error
                        "Failed to create constraint",
                        Some(format!("{}", e)),
                    )
                })?;
        debug!("Created constraint for slot {}", inclusion_payload.slot);

        // Store the commitment and constraint atomically
        self.state
            .db
            .store_signed_commitment_and_constraint(
                inclusion_payload.slot,
                &signed_commitment.commitment.request_hash,
                &signed_commitment,
                &constraint,
            )
            .map_err(|e| {
                jsonrpsee::types::error::ErrorObject::owned(
                    -32603, // Internal error
                    "Failed to store commitment and constraint",
                    Some(format!("{}", e)),
                )
            })?;
        debug!(
            "Stored commitment and constraint for slot {}",
            inclusion_payload.slot
        );

        info!(
            "Signed commitment, slot {}, request hash {:?}",
            inclusion_payload.slot, signed_commitment.commitment.request_hash
        );

        // Return the signed commitment
        Ok(signed_commitment)
    }

    /// Query a previously created SignedCommitment
    async fn commitment_result(&self, request_hash: B256) -> RpcResult<SignedCommitment> {
        match self.state.db.get_signed_commitment(&request_hash) {
            Ok(Some(signed_commitment)) => Ok(signed_commitment.commitment),
            Ok(None) => {
                Err(jsonrpsee::types::error::ErrorObject::owned(
                    -32602, // Invalid params
                    "Commitment not found",
                    Some(format!(
                        "No commitment found for request hash: {}",
                        request_hash
                    )),
                ))
            }
            Err(e) => {
                Err(jsonrpsee::types::error::ErrorObject::owned(
                    -32603, // Internal error
                    "Failed to get commitment and constraint",
                    Some(format!("{}", e)),
                ))
            }
        }
    }

    /// Query slots information.
    async fn slots(&self) -> RpcResult<SlotInfoResponse> {
        // Get current slot
        let current_slot = current_slot(&self.state.chain);
        debug!("Current slot: {}", current_slot);

        // Query slots this gateway is delegated to
        let delegated_slots = self
            .state
            .db
            .get_delegations_in_range(current_slot, current_slot + LOOKAHEAD_WINDOW_SIZE)
            .map_err(|e| {
                jsonrpsee::types::error::ErrorObject::owned(
                    -32603, // Internal error
                    "Failed to get delegated slots",
                    Some(format!("{}", e)),
                )
            })?;

        // Build slot info for each delegated slot
        let mut slots = Vec::new();

        // Create offering with chain ID and commitment type
        let offering = Offering {
            chain_id: self.state.chain.id().to::<u64>(),
            commitment_types: vec![INCLUSION_COMMITMENT_TYPE],
        };

        for (slot, _) in delegated_slots {
            slots.push(SlotInfo {
                slot,
                offerings: vec![offering.clone()],
            });
        }

        Ok(SlotInfoResponse { slots })
    }

    /// Query current fee information.
    async fn fee(&self, request: CommitmentRequest) -> RpcResult<FeeInfo> {
        let fee_info = utils::calculate_fee_info(&request, &self.state.execution_client)
            .await
            .map_err(|e| {
                jsonrpsee::types::error::ErrorObject::owned(
                    -32603, // Internal error
                    "Failed to calculate fee info",
                    Some(format!("{}", e)),
                )
            })?;
        Ok(fee_info)
    }
}
