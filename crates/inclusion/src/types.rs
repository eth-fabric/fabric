use alloy::consensus::{
    Signed, Transaction, TxEip1559, TxEip2930, TxEip4844, TxEip7702, TxEnvelope, TxLegacy,
    transaction::SignerRecoverable,
};
use alloy::primitives::{B256, Bytes};
use alloy::rlp::Decodable;
use alloy::sol_types::SolValue;
use eyre::{Result, WrapErr, bail};
use serde::{Deserialize, Serialize};

use commitments::types::SignedCommitment;
use constraints::types::Constraint;

/// Fee payload for an inclusion preconf request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeePayload {
    pub request_hash: B256,
    pub price_gwei: u64,
}

/// A signed commitment and its paired constraint for a specific slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCommitmentAndConstraint {
    pub commitment: SignedCommitment,
    pub constraint: Constraint,
}

/// Payload for commitments/constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionPayload {
    pub slot: u64,
    pub signed_tx: Bytes,
}

impl InclusionPayload {
    /// ABI-encodes the InclusionPayload struct for signing commitments
    pub fn abi_encode(&self) -> Result<Bytes> {
        alloy::sol! {
            struct SolInclusionPayload {
                uint64 slot;
                bytes signed_tx;
            }
        }

        Ok(Bytes::from(SolInclusionPayload::abi_encode(
            &SolInclusionPayload {
                slot: self.slot,
                signed_tx: self.signed_tx.clone(),
            },
        )))
    }

    /// ABI-decodes an InclusionPayload from bytes
    pub fn abi_decode(data: &Bytes) -> Result<Self> {
        alloy::sol! {
            struct SolInclusionPayload {
                uint64 slot;
                bytes signed_tx;
            }
        }

        let decoded =
            SolInclusionPayload::abi_decode(data).wrap_err("Failed to decode InclusionPayload")?;

        Ok(InclusionPayload {
            slot: decoded.slot,
            signed_tx: decoded.signed_tx,
        })
    }

    /// Decodes an RLP-encoded signed transaction into a TxEnvelope
    pub fn decode_transaction(&self) -> Result<TxEnvelope> {
        let tx_envelope = TxEnvelope::decode(&mut self.signed_tx.as_ref())
            .wrap_err("Failed to decode transaction from RLP bytes")?;
        Ok(tx_envelope)
    }

    pub fn verify_signature(&self) -> Result<()> {
        let tx_envelope = self.decode_transaction()?;
        tx_envelope
            .recover_signer()
            .wrap_err("Failed to recover signer - invalid signature")?;
        Ok(())
    }

    /// Returns the transaction hash by trying each supported transaction type
    /// This hash includes the signature of the transaction so is not the same
    /// as the hash of the unsigned transaction / the hash for signing commitments
    pub fn tx_hash(&self) -> Result<B256> {
        if let Ok(tx) = Signed::<TxEip1559>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(B256::from(tx.hash().as_ref()));
        }

        if let Ok(tx) = Signed::<TxLegacy>::rlp_decode(&mut self.signed_tx.as_ref()) {
            return Ok(B256::from(tx.hash().as_ref()));
        }

        if let Ok(tx) = Signed::<TxEip2930>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(B256::from(tx.hash().as_ref()));
        }

        if let Ok(tx) = Signed::<TxEip4844>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(B256::from(tx.hash().as_ref()));
        }

        if let Ok(tx) = Signed::<TxEip7702>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(B256::from(tx.hash().as_ref()));
        }

        bail!("Failed to decode signed transaction as any known type")
    }

    /// Returns the gas used by the transaction by trying each supported transaction type
    pub fn gas(&self) -> Result<u64> {
        if let Ok(tx) = Signed::<TxEip1559>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(tx.gas_limit());
        }

        if let Ok(tx) = Signed::<TxLegacy>::rlp_decode(&mut self.signed_tx.as_ref()) {
            return Ok(tx.gas_limit());
        }

        if let Ok(tx) = Signed::<TxEip2930>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(tx.gas_limit());
        }

        if let Ok(tx) = Signed::<TxEip4844>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(tx.gas_limit());
        }

        if let Ok(tx) = Signed::<TxEip7702>::eip2718_decode(&mut self.signed_tx.as_ref()) {
            return Ok(tx.gas_limit());
        }

        bail!("Failed to decode signed transaction as any known type")
    }

    /// Useful for testing
    pub fn random() -> Self {
        use alloy::consensus::{SignableTransaction, Signed, TxEip1559, TxEnvelope};
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
        InclusionPayload {
            slot: 12345,
            signed_tx: Bytes::from(encoded_tx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::consensus::TxType;
    use alloy::primitives::keccak256;

    #[test]
    fn test_abi_encode_inclusion_payload() -> Result<()> {
        let payload = InclusionPayload {
            slot: 12345,
            signed_tx: Bytes::new(),
        };

        let encoded = payload.abi_encode()?;

        // Verify the encoded data is not empty
        assert!(!encoded.is_empty());

        assert_eq!(
            keccak256(&encoded).to_string(),
            "0x0ccee42883ae08bce4f8725891b2e9cc65a260876b55b23d4b7d94d60fad8211"
        );
        Ok(())
    }

    #[test]
    fn test_abi_decode_inclusion_payload() -> Result<()> {
        let original_payload = InclusionPayload {
            slot: 67890,
            signed_tx: Bytes::from(vec![0x05, 0x06, 0x07, 0x08, 0x09]),
        };

        // Encode first
        let encoded = original_payload.abi_encode()?;

        // Then decode
        let decoded = InclusionPayload::abi_decode(&encoded)?;

        // Verify they match
        assert_eq!(decoded.slot, original_payload.slot);
        assert_eq!(decoded.signed_tx, original_payload.signed_tx);

        println!("Original: {:?}", original_payload);
        println!("Decoded: {:?}", decoded);
        Ok(())
    }

    #[test]
    fn test_tx_helpers() -> Result<()> {
        let payload = InclusionPayload::random();
        payload.tx_hash().unwrap();
        payload.gas().unwrap();

        let decoded_tx = payload.decode_transaction().unwrap();
        assert_eq!(decoded_tx.tx_type(), TxType::Eip1559);

        payload.verify_signature().unwrap();
        Ok(())
    }
}
