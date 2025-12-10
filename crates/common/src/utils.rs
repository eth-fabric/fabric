use alloy::rpc::types::beacon::BlsPublicKey;
use alloy::{hex, primitives::Address};
use eyre::Result;
use tokio::{
	select,
	signal::unix::{SignalKind, signal},
};

pub fn decode_pubkey(public_key: &str) -> Result<BlsPublicKey> {
	let public_key = hex::decode(public_key)?;
	let public_key_bytes =
		public_key.try_into().map_err(|e| eyre::eyre!("Failed to convert BLS public key to bytes: {:?}", e))?;
	Ok(BlsPublicKey::new(public_key_bytes))
}

pub fn decode_address(address: &str) -> Result<Address> {
	let address = hex::decode(address)?;
	let address_bytes = address.try_into().map_err(|e| eyre::eyre!("Failed to convert address to bytes: {:?}", e))?;
	Ok(Address::new(address_bytes))
}

#[cfg(unix)]
pub async fn wait_for_signal() -> Result<()> {
	let mut sigint = signal(SignalKind::interrupt())?;
	let mut sigterm = signal(SignalKind::terminate())?;

	select! {
		_ = sigint.recv() => {}
		_ = sigterm.recv() => {}
	}

	Ok(())
}

#[cfg(windows)]
pub async fn wait_for_signal() -> eyre::Result<()> {
	signal::ctrl_c().await?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decode_pubkey_valid() {
		// 48-byte BLS public key (96 hex chars)
		let pubkey_hex =
			"a0b0c0d0e0f0a1b1c1d1e1f1a2b2c2d2e2f2a3b3c3d3e3f3a4b4c4d4e4f4a5b5c5d5e5f5a6b6c6d6e6f6a7b7c7d7e7f7";
		let result = decode_pubkey(pubkey_hex);
		assert!(result.is_ok());
	}

	#[test]
	fn test_decode_pubkey_invalid_hex() {
		let invalid_hex = "zzzz";
		let result = decode_pubkey(invalid_hex);
		assert!(result.is_err());
	}

	#[test]
	fn test_decode_pubkey_wrong_length() {
		// Too short (only 32 bytes / 64 hex chars)
		let short_hex = "a0b0c0d0e0f0a1b1c1d1e1f1a2b2c2d2e2f2a3b3c3d3e3f3a4b4c4d4e4f4a5b5";
		let result = decode_pubkey(short_hex);
		assert!(result.is_err());
	}

	#[test]
	fn test_decode_address_valid() {
		// 20-byte Ethereum address (40 hex chars)
		let address_hex = "742d35cc6634c0532925a3b844bc454e4438f44e";
		let result = decode_address(address_hex);
		assert!(result.is_ok());
	}

	#[test]
	fn test_decode_address_invalid_hex() {
		let invalid_hex = "zzzz";
		let result = decode_address(invalid_hex);
		assert!(result.is_err());
	}

	#[test]
	fn test_decode_address_wrong_length() {
		// Too short (only 10 bytes / 20 hex chars)
		let short_hex = "742d35cc6634c0532925";
		let result = decode_address(short_hex);
		assert!(result.is_err());
	}
}
