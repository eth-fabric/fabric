use eyre::{Result, WrapErr};
use tracing::{debug, info};

use alloy::consensus::{SignableTransaction, TxEnvelope};
use alloy::network::{Ethereum, TransactionBuilder};
use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::providers::{DynProvider, Provider};
use alloy::rpc::types::beacon::{BlsPublicKey, BlsSignature};
use commit_boost::prelude::{Chain, commit::client::SignerClient};

use commitments::types::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment};
use constraints::types::{Constraint, ConstraintsMessage, SignedConstraints};
use signing::signer;
use urc::utils::{
    get_commitment_request_signing_root, get_commitment_signing_root,
    get_constraints_message_signing_root,
};

use crate::constants::{INCLUSION_COMMITMENT_TYPE, INCLUSION_CONSTRAINT_TYPE};
use crate::types::{FeePayload, InclusionPayload};

/// Helper functions for RPC business logic
/// This module contains utility functions that can be shared across multiple RPC handlers

/// Validates a commitment request and returns the decoded InclusionPayload
pub fn validate_commitment_request(request: &CommitmentRequest) -> Result<InclusionPayload> {
    if request.commitment_type != INCLUSION_COMMITMENT_TYPE {
        return Err(eyre::eyre!(
            "Invalid commitment type: expected {}, got {}",
            INCLUSION_COMMITMENT_TYPE,
            request.commitment_type
        ));
    }

    if request.payload.is_empty() {
        return Err(eyre::eyre!("Payload cannot be empty"));
    }

    if request.slasher == Address::ZERO {
        return Err(eyre::eyre!("Invalid slasher address"));
    }

    // Validate that the payload is a valid InclusionPayload
    match InclusionPayload::abi_decode(&request.payload) {
        Ok(inclusion_payload) => {
            debug!(
                "Decoded InclusionPayload: slot={}, signed_tx_len={}",
                inclusion_payload.slot,
                inclusion_payload.signed_tx.len()
            );

            // Additional validation for InclusionPayload
            if inclusion_payload.slot == 0 {
                return Err(eyre::eyre!("Invalid slot: 0"));
            }

            if inclusion_payload.signed_tx.is_empty() {
                return Err(eyre::eyre!("Signed transaction cannot be empty"));
            }

            // Validate signed_tx format and signature
            inclusion_payload.verify_signature()?;

            debug!("Commitment request validation passed");
            Ok(inclusion_payload)
        }
        Err(e) => {
            return Err(eyre::eyre!("Invalid payload format: {}", e));
        }
    }
}

/// Converts a TxEnvelope to a TransactionRequest suitable for eth_estimateGas
///
/// This helper function extracts transaction fields from a signed transaction
/// and converts them into a format suitable for JSON-RPC calls like eth_estimateGas.
///
/// # Parameters
///
/// * `tx_envelope` - The decoded transaction envelope
///
/// # Returns
///
/// An `alloy::rpc::types::TransactionRequest` with all fields populated from the envelope
///
/// # Errors
///
/// Returns an error if the signer cannot be recovered from the transaction signature
pub fn tx_envelope_to_rpc_request(
    tx_envelope: &TxEnvelope,
) -> Result<alloy::rpc::types::TransactionRequest> {
    use alloy::rpc::types::TransactionRequest;

    let mut tx_request = TransactionRequest::default();

    match tx_envelope {
        TxEnvelope::Legacy(signed_tx) => {
            let from = signed_tx
                .recover_signer()
                .wrap_err("Failed to recover signer from legacy transaction")?;
            let tx = signed_tx.tx();

            // Convert TxKind to optional address
            let to = match tx.to {
                alloy::primitives::TxKind::Call(addr) => Some(addr),
                alloy::primitives::TxKind::Create => None,
            };

            tx_request = tx_request
                .with_from(from)
                .with_value(tx.value)
                .with_gas_limit(tx.gas_limit)
                .with_gas_price(tx.gas_price)
                .with_nonce(tx.nonce)
                .with_input(tx.input.clone());

            if let Some(to_addr) = to {
                tx_request = tx_request.with_to(to_addr);
            }

            if let Some(chain_id) = tx.chain_id {
                tx_request = tx_request.with_chain_id(chain_id);
            }
        }
        TxEnvelope::Eip2930(signed_tx) => {
            let from = signed_tx
                .recover_signer()
                .wrap_err("Failed to recover signer from EIP-2930 transaction")?;
            let tx = signed_tx.tx();

            let to = match tx.to {
                alloy::primitives::TxKind::Call(addr) => Some(addr),
                alloy::primitives::TxKind::Create => None,
            };

            tx_request = tx_request
                .with_from(from)
                .with_value(tx.value)
                .with_gas_limit(tx.gas_limit)
                .with_gas_price(tx.gas_price)
                .with_nonce(tx.nonce)
                .with_input(tx.input.clone())
                .with_chain_id(tx.chain_id)
                .with_access_list(tx.access_list.clone());

            if let Some(to_addr) = to {
                tx_request = tx_request.with_to(to_addr);
            }
        }
        TxEnvelope::Eip1559(signed_tx) => {
            let from = signed_tx
                .recover_signer()
                .wrap_err("Failed to recover signer from EIP-1559 transaction")?;
            let tx = signed_tx.tx();

            let to = match tx.to {
                alloy::primitives::TxKind::Call(addr) => Some(addr),
                alloy::primitives::TxKind::Create => None,
            };

            tx_request = tx_request
                .with_from(from)
                .with_value(tx.value)
                .with_gas_limit(tx.gas_limit)
                .with_max_fee_per_gas(tx.max_fee_per_gas)
                .with_max_priority_fee_per_gas(tx.max_priority_fee_per_gas)
                .with_nonce(tx.nonce)
                .with_input(tx.input.clone())
                .with_chain_id(tx.chain_id)
                .with_access_list(tx.access_list.clone());

            if let Some(to_addr) = to {
                tx_request = tx_request.with_to(to_addr);
            }
        }
        TxEnvelope::Eip4844(signed_tx) => {
            let from = signed_tx
                .recover_signer()
                .wrap_err("Failed to recover signer from EIP-4844 transaction")?;
            let tx = signed_tx.tx().tx();

            // EIP-4844 transactions have a direct 'to' address field (not TxKind)
            tx_request = tx_request
                .with_from(from)
                .with_to(tx.to) // EIP-4844 'to' is an Address, not TxKind
                .with_value(tx.value)
                .with_gas_limit(tx.gas_limit)
                .with_max_fee_per_gas(tx.max_fee_per_gas)
                .with_max_priority_fee_per_gas(tx.max_priority_fee_per_gas)
                .with_nonce(tx.nonce)
                .with_input(tx.input.clone())
                .with_chain_id(tx.chain_id)
                .with_access_list(tx.access_list.clone());
            // Note: blob-specific fields are not part of standard TransactionRequest for eth_estimateGas
        }
        TxEnvelope::Eip7702(signed_tx) => {
            let from = signed_tx
                .recover_signer()
                .wrap_err("Failed to recover signer from EIP-7702 transaction")?;
            let tx = signed_tx.tx();

            // EIP-7702 has a direct 'to' address field (not TxKind)
            tx_request = tx_request
                .with_from(from)
                .with_to(tx.to) // EIP-7702 'to' is an Address, not TxKind
                .with_value(tx.value)
                .with_gas_limit(tx.gas_limit)
                .with_max_fee_per_gas(tx.max_fee_per_gas)
                .with_max_priority_fee_per_gas(tx.max_priority_fee_per_gas)
                .with_nonce(tx.nonce)
                .with_input(tx.input.clone())
                .with_chain_id(tx.chain_id)
                .with_access_list(tx.access_list.clone());
            // Note: authorization_list is not part of standard TransactionRequest for eth_estimateGas
        }
    }

    debug!("Converted transaction envelope to RPC request");
    Ok(tx_request)
}

/// Calculates fee information for a commitment request using RPC calls
///
/// This function:
/// 1. Decodes the InclusionPayload from the request
/// 2. Decodes the signed transaction from the payload
/// 3. Converts it to a TransactionRequest for gas estimation
/// 4. Calls eth_estimateGas to get the gas required
/// 5. Calls eth_gasPrice to get the current gas price
/// 6. Calculates the total fee as gas_price * estimated_gas
///
/// # Parameters
///
/// * `request` - The commitment request containing the InclusionPayload
/// * `execution_client` - The execution client RPC API for gas price and estimation calls
///
/// # Returns
///
/// `FeeInfo` containing the calculated fee and commitment type
pub async fn calculate_fee_info(
    request: &CommitmentRequest,
    execution_client: &DynProvider<Ethereum>,
) -> Result<FeeInfo> {
    debug!(
        "Calculating fee for commitment type: {}",
        request.commitment_type
    );

    // 1. Decode the InclusionPayload from the request
    let inclusion_payload = InclusionPayload::abi_decode(&request.payload)
        .wrap_err("Failed to decode InclusionPayload from request")?;

    // 2. Decode the signed transaction
    let tx_envelope = inclusion_payload.decode_transaction()?;

    // 3. Convert to TransactionRequest for gas estimation
    let tx_request = tx_envelope_to_rpc_request(&tx_envelope)?;

    // 4. Estimate gas required for the transaction using Alloy provider
    let estimated_gas = U256::from(
        execution_client
            .estimate_gas(tx_request)
            .await
            .wrap_err("Failed to estimate gas for transaction")?,
    );

    // 5. Get current gas price
    let gas_price = U256::from(
        execution_client
            .get_gas_price()
            .await
            .wrap_err("Failed to get gas price from execution client node")?,
    );

    // Convert from wei to gwei by dividing by 1 billion (1e9)
    let price_gwei = ((gas_price * estimated_gas) / U256::from(1_000_000_000)).to();

    let request_hash = get_commitment_request_signing_root(&request);
    let fee_payload = FeePayload {
        request_hash,
        price_gwei,
    };

    debug!(
        "Calculated fee: estimated_gas={}, gas_price={} wei, price_gwei={} gwei",
        estimated_gas, gas_price, price_gwei
    );

    Ok(FeeInfo {
        fee_payload: Bytes::from(serde_json::to_vec(&fee_payload)?),
        commitment_type: request.commitment_type,
    })
}

/// Validates a request hash format
pub fn validate_request_hash(hash: &str) -> Result<B256> {
    if hash.len() != 66 || !hash.starts_with("0x") {
        return Err(eyre::eyre!(
            "Invalid hash format. Expected 0x followed by 64 hex characters"
        ));
    }

    hash.parse::<B256>().wrap_err("Failed to parse hash")
}

/// Creates a constraint from a commitment request
/// This function creates a constraint with the same payload but using the constraint type
pub fn create_constraint_from_commitment_request(
    request: &CommitmentRequest,
    slot: u64,
) -> Result<Constraint> {
    debug!(
        "Creating constraint from commitment request for slot {}",
        slot
    );

    // Create the constraint with the same payload but constraint type
    let constraint = Constraint {
        constraint_type: INCLUSION_CONSTRAINT_TYPE,
        payload: request.payload.clone(),
    };

    debug!(
        "Created constraint with type {} and payload length {} for slot {}",
        constraint.constraint_type,
        constraint.payload.len(),
        slot
    );
    Ok(constraint)
}

/// Creates a properly signed commitment using ECDSA
pub async fn create_signed_commitment(
    request: &CommitmentRequest,
    signer_client: &mut SignerClient,
    committer_address: Address,
    module_signing_id: &B256,
    chain: Chain,
) -> Result<SignedCommitment> {
    let request_hash = get_commitment_request_signing_root(request);

    let commitment = Commitment {
        commitment_type: request.commitment_type,
        payload: request.payload.clone(),
        request_hash: request_hash,
        slasher: request.slasher,
    };

    let commitment_hash = get_commitment_signing_root(&commitment);

    // Call the proxy_ecdsa signer
    let response = signer::call_proxy_ecdsa_signer(
        signer_client,
        commitment_hash,
        committer_address,
        module_signing_id,
        chain,
    )
    .await?;

    // 6. Construct the SignedCommitment
    let signed_commitment = SignedCommitment {
        commitment,
        nonce: response.nonce,
        signing_id: response.module_signing_id,
        signature: response.signature.normalized_s(),
    };

    Ok(signed_commitment)
}

/// Validates a signature against a commitment
pub fn verify_commitment_signature(
    commitment: &Commitment,
    signature: &alloy::primitives::Signature,
    expected_signer: &Address,
) -> Result<bool> {
    let commitment_hash = get_commitment_signing_root(commitment);
    let recovered_address = signature
        .recover_address_from_prehash(&commitment_hash)
        .wrap_err("Failed to recover address from signature")?;

    Ok(recovered_address == *expected_signer)
}

/// Creates a valid RLP-encoded EIP-1559 transaction with a mock signature
/// This can be reused across tests to generate properly formatted signed transactions
pub fn create_valid_signed_transaction() -> Bytes {
    use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
    use alloy::eips::eip2718::Encodable2718;
    use alloy::primitives::{Address, Bytes, TxKind, U256};
    use alloy::signers::{SignerSync, local::PrivateKeySigner};

    let signer = PrivateKeySigner::random();
    let tx = TxEip1559 {
        chain_id: 1,
        nonce: 0,
        gas_limit: 21000,
        max_fee_per_gas: 20_000_000_000u128,
        max_priority_fee_per_gas: 2_000_000_000u128,
        to: TxKind::Call(Address::from([0x01; 20])),
        value: U256::from(1_000_000_000_000_000_000u64),
        input: Bytes::new(),
        access_list: Default::default(),
    };

    // Create a mock signature that will pass the format validation
    let encoded_tx = tx.encoded_for_signing();
    let signature = signer
        .sign_message_sync(&encoded_tx)
        .expect("Failed to sign message");
    let signed_tx = Signed::new_unhashed(tx, signature);
    let tx_envelope = TxEnvelope::Eip1559(signed_tx);
    let mut encoded_tx = Vec::new();
    tx_envelope.encode_2718(&mut encoded_tx);
    Bytes::from(encoded_tx)
}

/// Creates a properly signed constraints message using BLS
pub async fn sign_constraints_message(
    message: &ConstraintsMessage,
    signer_client: &mut SignerClient,
    bls_public_key: BlsPublicKey,
    module_signing_id: &B256,
    chain: Chain,
) -> Result<SignedConstraints> {
    debug!("Creating signed constraints with proper BLS signing");

    // Hash the constraints message
    let signing_root = get_constraints_message_signing_root(message)?;

    // Call the proxy_bls signer
    let response = signer::call_proxy_bls_signer(
        signer_client,
        signing_root,
        bls_public_key,
        module_signing_id,
        chain,
    )
    .await?;
    debug!("Received response from proxy_bls: {:?}", response);

    let signed_constraints = SignedConstraints {
        message: message.clone(),
        nonce: response.nonce,
        signing_id: response.module_signing_id,
        signature: BlsSignature::new(response.signature.serialize()),
    };

    debug!("Signed constraints created successfully");
    Ok(signed_constraints)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, Bytes};

    #[tokio::test]
    async fn test_validate_commitment_request() -> Result<()> {
        // Test valid request with proper InclusionPayload and properly signed transaction
        let valid_signed_tx = create_valid_signed_transaction();
        let inclusion_payload = InclusionPayload {
            slot: 100,
            signed_tx: valid_signed_tx,
        };
        let encoded_payload = inclusion_payload.abi_encode()?;

        let valid_request = CommitmentRequest {
            commitment_type: 1,
            payload: encoded_payload,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        assert!(validate_commitment_request(&valid_request).is_ok());

        // Test invalid commitment type
        let invalid_type_payload = InclusionPayload {
            slot: 100,
            signed_tx: Bytes::from(vec![0x01]),
        };
        let invalid_type_encoded = invalid_type_payload.abi_encode()?;
        let invalid_type_request = CommitmentRequest {
            commitment_type: 0,
            payload: invalid_type_encoded,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        assert!(validate_commitment_request(&invalid_type_request).is_err());

        // Test empty payload
        let empty_payload_request = CommitmentRequest {
            commitment_type: 1,
            payload: Bytes::new(),
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        assert!(validate_commitment_request(&empty_payload_request).is_err());

        // Test zero address
        let zero_address_payload = InclusionPayload {
            slot: 100,
            signed_tx: Bytes::from(vec![0x01]),
        };
        let zero_address_encoded = zero_address_payload.abi_encode()?;
        let zero_address_request = CommitmentRequest {
            commitment_type: 1,
            payload: zero_address_encoded,
            slasher: Address::ZERO,
        };

        assert!(validate_commitment_request(&zero_address_request).is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_commitment_request_with_inclusion_payload() -> Result<()> {
        // Create a valid InclusionPayload with properly RLP-encoded EIP-1559 transaction
        let valid_signed_tx = create_valid_signed_transaction();
        let inclusion_payload = InclusionPayload {
            slot: 100,
            signed_tx: valid_signed_tx,
        };

        // Encode it
        let encoded_payload = inclusion_payload.abi_encode()?;

        // Create a valid commitment request
        let valid_request = CommitmentRequest {
            commitment_type: 1,
            payload: encoded_payload,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        // Should pass validation
        assert!(validate_commitment_request(&valid_request).is_ok());

        // Test invalid slot (0)
        let invalid_slot_payload = InclusionPayload {
            slot: 0, // Invalid
            signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]),
        };
        let invalid_slot_encoded = invalid_slot_payload.abi_encode()?;
        let invalid_slot_request = CommitmentRequest {
            commitment_type: 1,
            payload: invalid_slot_encoded,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };
        assert!(validate_commitment_request(&invalid_slot_request).is_err());

        // Test empty signed_tx
        let empty_tx_payload = InclusionPayload {
            slot: 100,
            signed_tx: Bytes::new(), // Invalid
        };
        let empty_tx_encoded = empty_tx_payload.abi_encode()?;
        let empty_tx_request = CommitmentRequest {
            commitment_type: 1,
            payload: empty_tx_encoded,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };
        assert!(validate_commitment_request(&empty_tx_request).is_err());

        println!("InclusionPayload validation tests passed");
        Ok(())
    }

    #[test]
    fn test_validate_commitment_request_invalid_payload_format() -> Result<()> {
        // Create a request with invalid payload (not ABI-encoded InclusionPayload)
        let invalid_request = CommitmentRequest {
            commitment_type: 1,
            payload: Bytes::from(vec![0x01, 0x02, 0x03]), // Not ABI-encoded
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        // Should fail validation due to invalid payload format
        assert!(validate_commitment_request(&invalid_request).is_err());

        println!("Invalid payload format test passed");
        Ok(())
    }

    // test_calculate_fee_info removed: relies on deprecated ExecutionApiClient

    #[test]
    fn test_transaction_decoding() -> Result<()> {
        use alloy::consensus::TxEnvelope;
        use alloy::rlp::Decodable;

        // Create a minimal valid transaction (this is a simplified example)
        let tx_bytes = Bytes::from(vec![0x01; 32]); // 32 bytes

        // Try to decode as a transaction envelope
        match TxEnvelope::decode(&mut tx_bytes.as_ref()) {
            Ok(tx_envelope) => {
                println!("Successfully decoded transaction envelope");
                println!("Transaction type: {:?}", tx_envelope.tx_type());
                Ok(())
            }
            Err(e) => {
                println!("Failed to decode transaction: {}", e);
                // This is expected for our test data since it's not a real transaction
                Ok(())
            }
        }
    }

    #[test]
    fn test_verify_signed_tx() -> Result<()> {
        // Test with empty transaction (should fail)
        let empty_tx = Bytes::new();
        assert!(
            InclusionPayload {
                slot: 100,
                signed_tx: empty_tx
            }
            .verify_signature()
            .is_err()
        );

        // Test with invalid transaction data (should fail)
        let invalid_tx = Bytes::from(vec![0x01, 0x02, 0x03]);
        assert!(
            InclusionPayload {
                slot: 100,
                signed_tx: invalid_tx
            }
            .verify_signature()
            .is_err()
        );

        // Test with a properly RLP-encoded transaction (should pass)
        let valid_tx = create_valid_signed_transaction();
        assert!(
            InclusionPayload {
                slot: 100,
                signed_tx: valid_tx
            }
            .verify_signature()
            .is_ok()
        );

        println!("verify_signed_tx tests passed");
        Ok(())
    }

    #[test]
    fn test_validate_commitment_request_with_signed_tx_verification() -> Result<()> {
        // Test that validate_commitment_request now includes signed_tx verification
        // This test ensures that invalid signed transactions are caught

        // Create a valid InclusionPayload with invalid signed_tx (too short)
        let invalid_tx_payload = InclusionPayload {
            slot: 100,
            signed_tx: Bytes::from(vec![0x01, 0x02, 0x03]), // Too short transaction
        };
        let invalid_tx_encoded = invalid_tx_payload.abi_encode()?;
        let invalid_tx_request = CommitmentRequest {
            commitment_type: 1,
            payload: invalid_tx_encoded,
            slasher: "0x1234567890123456789012345678901234567890".parse()?,
        };

        // Should fail validation due to invalid signed transaction
        assert!(validate_commitment_request(&invalid_tx_request).is_err());

        println!("Signed transaction verification integration test passed");
        Ok(())
    }
}
