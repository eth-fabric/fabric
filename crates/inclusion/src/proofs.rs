use alloy::consensus::TxEnvelope;
use alloy::primitives::{B256, Bytes, U256};
use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use eth_trie::{EthTrie, MemoryDB, Trie};
use ethereum_types::H256;
use eyre::{Context, Result, eyre};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use constraints::helpers::extract_transactions;
use constraints::types::ConstraintProofs;

/// Merkle inclusion proof for an inclusion payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
	/// Transaction hash
	pub tx_hash: B256,
	/// Index of the transaction in the block
	pub tx_index: usize,
	/// Merkle proof nodes
	pub proof: Vec<Vec<u8>>,
}

impl InclusionProof {
	/// Creates a new InclusionProof
	pub fn new(trie_builder: &mut TransactionTrieBuilder, tx_hash: B256) -> Result<Self> {
		// Find the transaction index
		let tx_index = trie_builder.find_tx_index(&tx_hash)?;

		// Generate the proof
		let proof = trie_builder.get_proof(tx_index)?;

		Ok(InclusionProof { tx_hash, tx_index, proof })
	}

	/// Serializes the InclusionProof to Bytes
	pub fn to_bytes(&self) -> Result<Bytes> {
		let buf = bincode::serialize(self).wrap_err("failed to serialize InclusionProof")?;
		Ok(Bytes::from(buf))
	}

	pub fn from_bytes(bytes: &Bytes) -> Result<Self> {
		let proof: InclusionProof =
			bincode::deserialize(bytes.as_ref()).wrap_err("failed to deserialize InclusionProof")?;
		Ok(proof)
	}
}

/// Builder for transaction Merkle Patricia Trie
pub struct TransactionTrieBuilder {
	trie: EthTrie<MemoryDB>,
	transactions: Vec<B256>,
}

impl TransactionTrieBuilder {
	/// Create a new trie builder
	pub fn new() -> Self {
		let memdb = Arc::new(MemoryDB::new(true));
		let trie = EthTrie::new(memdb);
		Self { trie, transactions: Vec::new() }
	}

	/// Build the transaction trie from a list of signed transactions
	pub fn build(transactions: &[TxEnvelope]) -> Result<Self> {
		let mut builder = Self::new();

		for (idx, tx) in transactions.iter().enumerate() {
			// Key is RLP-encoded index
			let key = alloy::rlp::encode(U256::from(idx));

			// Value is RLP-encoded signed transaction
			let tx_bytes = alloy::rlp::encode(tx);

			builder
				.trie
				.insert(key.as_slice(), &tx_bytes)
				.wrap_err_with(|| format!("Failed to insert transaction at index {idx} into trie"))?;
			builder.transactions.push(*tx.hash());
		}

		Ok(builder)
	}

	/// Proves inclusion of a batch of transactions and returns an encoded ConstraintProofs
	pub fn prove_batch(&mut self, tx_hashes: &[B256]) -> Result<ConstraintProofs> {
		// Finalize the trie by computing root before generating proofs
		// This ensures the trie structure is committed and proofs will be valid
		let _ = self.root()?;

		let payloads: Vec<Bytes> = tx_hashes
			.iter()
			.map(|tx_hash| InclusionProof::new(self, *tx_hash)?.to_bytes())
			.collect::<Result<Vec<_>>>()?;

		let constraint_types = vec![crate::constants::INCLUSION_CONSTRAINT_TYPE; payloads.len()];

		Ok(ConstraintProofs { constraint_types, payloads })
	}

	/// Verifies a batch of inclusion proofs, errors if any proof is invalid
	pub fn verify_batch(&mut self, proofs: &ConstraintProofs) -> Result<()> {
		let transactions_root = self.root()?;
		for (constraint_type, payload) in proofs.constraint_types.iter().zip(proofs.payloads.iter()) {
			if *constraint_type != crate::constants::INCLUSION_CONSTRAINT_TYPE {
				return Err(eyre!("Invalid constraint type {constraint_type}"));
			}

			let inclusion_proof: InclusionProof = InclusionProof::from_bytes(payload)?;
			let tx_bytes = self.verify_proof(inclusion_proof.tx_index, &inclusion_proof.proof, &transactions_root)?;

			// Decode the transaction and verify the hash matches the claimed tx_hash
			let tx: TxEnvelope = alloy::rlp::Decodable::decode(&mut tx_bytes.as_slice())
				.wrap_err("Failed to decode transaction from proof")?;
			if *tx.hash() != inclusion_proof.tx_hash {
				return Err(eyre!(
					"Transaction hash mismatch: proof claims {} but transaction at index {} has hash {}",
					inclusion_proof.tx_hash,
					inclusion_proof.tx_index,
					tx.hash()
				));
			}
		}
		Ok(())
	}

	/// Get the root hash of the trie
	pub fn root(&mut self) -> Result<B256> {
		let root = self.trie.root_hash().wrap_err("Failed to compute trie root hash")?;
		Ok(B256::from_slice(root.as_bytes()))
	}

	/// Generate a proof for a transaction at the given index
	pub fn get_proof(&mut self, tx_index: usize) -> Result<Vec<Vec<u8>>> {
		if tx_index >= self.transactions.len() {
			return Err(eyre!("Transaction not found at index {tx_index}"));
		}

		let key = alloy::rlp::encode(U256::from(tx_index));
		let proof = self
			.trie
			.get_proof(key.as_slice())
			.wrap_err_with(|| format!("Failed to generate proof for transaction at index {tx_index}"))?;
		Ok(proof)
	}

	/// Find the index of a transaction by its hash
	pub fn find_tx_index(&self, tx_hash: &B256) -> Result<usize> {
		self.transactions
			.iter()
			.position(|hash| hash == tx_hash)
			.ok_or_else(|| eyre!("Transaction hash {tx_hash} not found in block"))
	}

	/// Verify a proof for a transaction at the given index
	pub fn verify_proof(&self, tx_index: usize, proof: &[Vec<u8>], root: &B256) -> Result<Vec<u8>> {
		let key = alloy::rlp::encode(U256::from(tx_index));
		let root_hash = H256::from_slice(root.as_slice());

		self.trie
			.verify_proof(root_hash, key.as_slice(), proof.to_vec())
			.wrap_err_with(|| format!("Failed to verify proof for transaction at index {tx_index}"))?
			.ok_or_else(|| eyre!("Invalid proof for transaction at index {tx_index}"))
	}

	/// Get the transaction hash at the given index
	pub fn get_tx_hash(&self, tx_index: usize) -> Result<B256> {
		self.transactions.get(tx_index).copied().ok_or_else(|| eyre!("Transaction not found at index {tx_index}"))
	}
}

impl Default for TransactionTrieBuilder {
	fn default() -> Self {
		Self::new()
	}
}

pub fn prove_constraints(block: &AlloySubmitBlockRequest, tx_hashes: &[B256]) -> Result<ConstraintProofs> {
	if tx_hashes.is_empty() {
		return Ok(ConstraintProofs::default());
	}
	let transactions = extract_transactions(block)?;
	let mut builder = TransactionTrieBuilder::build(&transactions)?;
	let proofs = builder.prove_batch(tx_hashes)?;
	Ok(proofs)
}

pub fn verify_constraints(block: &AlloySubmitBlockRequest, proofs: &ConstraintProofs) -> Result<()> {
	let transactions = extract_transactions(block)?;

	info!(
		"Verifying constraints, transactions: {}, constraint_types: {}, proofs: {}",
		transactions.len(),
		proofs.constraint_types.len(),
		proofs.payloads.len()
	);

	let mut builder = TransactionTrieBuilder::build(&transactions)?;
	builder.verify_batch(proofs)?;
	Ok(())
}
#[cfg(test)]
mod tests {

	use super::*;
	use crate::types::InclusionPayload;

	#[test]
	fn test_inclusion_proof_serialization() {
		let proof = InclusionProof { tx_hash: B256::random(), tx_index: 0, proof: vec![vec![0x01, 0x02, 0x03]] };
		let bytes = proof.to_bytes().unwrap();
		let proof2 = InclusionProof::from_bytes(&bytes).unwrap();
		assert_eq!(proof.tx_hash, proof2.tx_hash);
		assert_eq!(proof.tx_index, proof2.tx_index);
		assert_eq!(proof.proof.len(), proof2.proof.len());
	}

	#[test]
	fn test_build_trie_and_generate_proof() {
		// Create some test transactions
		let payload1 = InclusionPayload::random();
		let tx1 = payload1.decode_transaction().unwrap();
		let payload2 = InclusionPayload::random();
		let tx2 = payload2.decode_transaction().unwrap();
		let transactions = vec![tx1, tx2];

		// // Build trie
		let mut builder = TransactionTrieBuilder::build(&transactions).unwrap();

		// Get root
		let root = builder.root().unwrap();
		assert_ne!(root, B256::ZERO);

		// Generate proof for first transaction
		let proof = builder.get_proof(0).unwrap();
		assert!(!proof.is_empty());

		// Find transaction index
		let tx1_hash = payload1.tx_hash().unwrap();
		let index = builder.find_tx_index(&tx1_hash).unwrap();
		assert_eq!(index, 0);

		// Verify proof
		let verified = builder.verify_proof(0, &proof, &root);
		assert!(verified.is_ok());

		// Generate proof for second transaction
		let proof2 = builder.get_proof(1).unwrap();
		assert!(!proof2.is_empty());

		// Find transaction index
		let tx2_hash = payload2.tx_hash().unwrap();
		let index = builder.find_tx_index(&tx2_hash).unwrap();
		assert_eq!(index, 1);

		// Verify proof
		let verified = builder.verify_proof(1, &proof2, &root);
		assert!(verified.is_ok());
	}

	#[test]
	fn test_prove_batch_and_verify_batch() {
		// Create test transactions
		let payload1 = InclusionPayload::random();
		let tx1 = payload1.decode_transaction().unwrap();
		let payload2 = InclusionPayload::random();
		let tx2 = payload2.decode_transaction().unwrap();
		let payload3 = InclusionPayload::random();
		let tx3 = payload3.decode_transaction().unwrap();
		let transactions = vec![tx1, tx2, tx3];

		let tx_hashes = vec![payload1.tx_hash().unwrap(), payload2.tx_hash().unwrap(), payload3.tx_hash().unwrap()];

		// Build trie and prove
		let mut prover_builder = TransactionTrieBuilder::build(&transactions).unwrap();
		let proofs = prover_builder.prove_batch(&tx_hashes).unwrap();

		assert_eq!(proofs.constraint_types.len(), 3);
		assert_eq!(proofs.payloads.len(), 3);

		// Build a separate trie and verify (simulates verifier rebuilding from block)
		let mut verifier_builder = TransactionTrieBuilder::build(&transactions).unwrap();
		let result = verifier_builder.verify_batch(&proofs);
		assert!(result.is_ok(), "verify_batch failed: {:?}", result.err());
	}
}
