use std::sync::Arc;

use alloy::primitives::keccak256;
use async_trait::async_trait;
use axum::http::HeaderMap;
use constraints::{
	api::ConstraintsApi,
	proxy::ProxyState,
	types::{
		AuthorizationContext, ConstraintCapabilities, ConstraintsResponse, DelegationsResponse, SignedConstraints,
		SignedDelegation, SubmitBlockRequestWithProofs,
	},
};
use eyre::{Result, eyre};
use lookahead::utils::current_slot;
use reqwest::Client;
use signing::signer::verify_bls;
use tracing::{debug, info};

use crate::relay::{
	state::RelayState,
	utils::{
		handle_proof_validation, validate_constraints_message, validate_delegation_message, validate_is_gateway,
		validate_is_proposer, verify_constraints_signature, verify_delegation_signature,
	},
};
use crate::storage::{DelegationsDbExt, InclusionDbExt};

#[derive(Clone)]
pub struct RelayServer {
	state: Arc<RelayState>,
}

impl RelayServer {
	pub fn new(state: Arc<RelayState>) -> Self {
		Self { state }
	}
}

impl AsRef<RelayState> for RelayServer {
	fn as_ref(&self) -> &RelayState {
		&self.state
	}
}

impl ProxyState for RelayServer {
	fn server_url(&self) -> &str {
		&self.state.downstream_relay_client.base_url
	}

	fn http_client(&self) -> &Client {
		&self.state.downstream_relay_client.client
	}
}

#[async_trait]
impl ConstraintsApi for RelayServer {
	/// POST /constraints
	async fn post_constraints(&self, signed_constraints: SignedConstraints) -> Result<()> {
		debug!("validate_constraints_message()");
		// Validate constraints message structure
		validate_constraints_message(&signed_constraints.message, &self.state.chain)?;

		debug!("verify_constraints_signature()");
		// Verify BLS signature using the delegate public key from the message
		verify_constraints_signature(&signed_constraints, &self.state.chain)?;

		debug!("validate_is_gateway()");
		// Verify a delegation exists and is for the correct gateway
		validate_is_gateway(&signed_constraints.message.delegate, signed_constraints.message.slot, &self.state.db)?;

		debug!("store_signed_constraints()");
		// Store signed constraints in database
		self.state.db.store_signed_constraints(&signed_constraints)?;

		info!(
			"Received signed constraints for slot {} from {}",
			signed_constraints.message.slot, signed_constraints.message.delegate
		);

		Ok(())
	}

	/// GET /constraints
	/// Returns all signed constraints for a slot
	/// If the slot has passed, returns all signed constraints for the slot without authentication
	/// If the slot has not passed, verifies the authentication headers against the receivers list
	async fn get_constraints(&self, slot: u64, auth: AuthorizationContext) -> Result<ConstraintsResponse> {
		// Get current slot to check if target slot has passed
		let current_slot = current_slot(&self.state.chain);

		// If we're at slot_target + 1 or beyond, bypass authentication
		if current_slot > slot {
			// Simply fetch and return all constraints for this slot
			let signed_constraints = self.state.db.get_signed_constraints(slot)?;
			match signed_constraints {
				Some(signed_constraints) => {
					return Ok(ConstraintsResponse { constraints: vec![signed_constraints] });
				}
				None => {
					return Ok(ConstraintsResponse { constraints: vec![] });
				}
			}
		}

		// Get signed constraints from database
		let signed_constraints = match self.state.db.get_signed_constraints(slot)? {
			Some(signed_constraints) => signed_constraints,
			None => return Ok(ConstraintsResponse { constraints: vec![] }),
		};

		// Bypass authentication if the receivers list is empty
		if signed_constraints.message.receivers.is_empty() {
			debug!("get_constraints(): No receivers list found for slot {}, bypassing authentication", slot);
			return Ok(ConstraintsResponse { constraints: vec![signed_constraints] });
		}

		// Slot has not passed yet and receivers list is not empty -> enforce authentication
		// All headers must be present
		let public_key = auth.public_key.ok_or(eyre!("Missing public key from header"))?;
		let signature = auth.signature.ok_or(eyre!("Missing signature from header"))?;
		let signing_id = auth.signing_id.ok_or(eyre!("Missing signing id from header"))?;
		let nonce = auth.nonce.ok_or(eyre!("Missing nonce from header"))?;

		// Compute slot hash for signature verification
		let slot_hash = keccak256(&slot.to_be_bytes());

		debug!("verifying slot signature");
		// Verify caller's signature against the slot hash using standardized commit-boost verification
		verify_bls(self.state.chain, &public_key, &slot_hash, &signature, &signing_id, nonce)?;

		debug!("verifying receiver list");
		// Verify the caller is part of the receivers list
		if !signed_constraints.message.receivers.contains(&public_key) {
			return Err(eyre!("Caller is not part of the receivers list for slot {}", slot));
		}

		info!("returning signed constraints for slot {}", slot);
		Ok(ConstraintsResponse { constraints: vec![signed_constraints] })
	}

	/// POST /delegation
	async fn post_delegation(&self, signed_delegation: SignedDelegation) -> Result<()> {
		debug!("validate_delegation_message()");
		// Validate delegation message is for a future slot
		validate_delegation_message(&signed_delegation.message, &self.state.chain)?;

		debug!("verify_delegation_signature()");
		// Verify delegation was signed by proposer
		verify_delegation_signature(&signed_delegation, &self.state.chain)?;

		debug!("validate_is_proposer()");
		// Validate proposer is scheduled for this slot
		validate_is_proposer(&signed_delegation.message.proposer, signed_delegation.message.slot, &self.state.db)?;

		debug!("checking for existing delegation");
		// Check for existing delegation to prevent equivocation
		if self.state.db.is_delegated(signed_delegation.message.slot)? {
			return Err(eyre!("Delegation already exists for slot {}", signed_delegation.message.slot));
		}

		debug!("storing delegation in database");
		// Store delegation in database
		self.state.db.store_delegation(&signed_delegation)?;

		info!(
			"Delegation posted for slot {}, key={:?}",
			signed_delegation.message.slot, signed_delegation.message.proposer
		);

		Ok(())
	}

	/// GET /delegations/{slot}
	async fn get_delegations(&self, slot: u64) -> Result<DelegationsResponse> {
		match self.state.db.get_delegation(slot)? {
			Some(delegation) => {
				return Ok(DelegationsResponse { delegations: vec![delegation] });
			}
			None => {
				return Ok(DelegationsResponse { delegations: vec![] });
			}
		}
	}

	/// POST /blocks_with_proofs
	async fn post_blocks_with_proofs(
		&self,
		block_request: SubmitBlockRequestWithProofs,
		headers: HeaderMap,
	) -> Result<()> {
		info!("post_blocks_with_proofs()");
		// Get the slot
		let slot = block_request.slot();

		debug!("fetching signed constraints from database");
		// Fetch constraints from database for the slot
		let signed_constraints = self
			.state
			.db
			.get_signed_constraints(slot)?
			.ok_or(eyre!("No signed constraints found for slot {}", slot))?;

		debug!("validating proofs");
		// Validate the proofs
		handle_proof_validation(&block_request, signed_constraints)?;

		// Make the legacy submit block request to the downnstream relay
		let block = block_request.into_block_request();
		self.state.downstream_relay_client.submit_block(block, headers).await?;

		Ok(())
	}

	/// GET /capabilities
	async fn get_capabilities(&self) -> Result<ConstraintCapabilities> {
		Ok(self.state.constraint_capabilities.clone())
	}

	/// GET /health
	async fn health_check(&self) -> Result<()> {
		Ok(())
	}
}
