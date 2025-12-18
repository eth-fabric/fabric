use alloy::consensus::TxEnvelope;
use alloy::rlp::Decodable;
use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use eyre::{Result, eyre};

pub fn extract_transactions(block: &AlloySubmitBlockRequest) -> Result<Vec<TxEnvelope>> {
	// Extract transaction bytes from the appropriate variant
	let tx_bytes_list = match &block {
		AlloySubmitBlockRequest::Electra(request) => {
			&request.execution_payload.payload_inner.payload_inner.transactions
		}
		AlloySubmitBlockRequest::Fulu(request) => &request.execution_payload.payload_inner.payload_inner.transactions,
		AlloySubmitBlockRequest::Deneb(request) => &request.execution_payload.payload_inner.payload_inner.transactions,
		AlloySubmitBlockRequest::Capella(request) => &request.execution_payload.payload_inner.transactions,
	};

	// Decode transactions
	let mut transactions = Vec::new();

	for tx_bytes in tx_bytes_list {
		let tx =
			TxEnvelope::decode(&mut tx_bytes.as_ref()).map_err(|e| eyre!("Failed to decode transaction: {}", e))?;
		transactions.push(tx);
	}

	if transactions.is_empty() {
		return Err(eyre!("No transactions in execution payload"));
	}

	Ok(transactions)
}
