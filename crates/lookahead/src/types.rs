use commit_boost::prelude::BlsPublicKey;
use eyre::Result;
use serde::{Deserialize, Serialize};

/// Configuration for Beacon API integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconApiConfig {
    /// Primary beacon node endpoint (e.g., Alchemy)
    pub primary_endpoint: String,
    /// Fallback beacon node endpoints
    pub fallback_endpoints: Vec<String>,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Beacon chain genesis time (Unix timestamp)
    pub genesis_time: u64,
}

/// Validator duty information from Beacon API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorDuty {
    /// Validator index in beacon state
    pub validator_index: String,
    /// BLS public key of the validator
    pub pubkey: String,
    /// Slot number for the duty
    pub slot: String,
}

/// Helper functions for beacon chain operations
impl ValidatorDuty {
    pub fn parse_pubkey(&self) -> Result<BlsPublicKey> {
        let pubkey_str = self.pubkey.strip_prefix("0x").unwrap_or(&self.pubkey);
        let bytes = hex::decode(pubkey_str)?;

        if bytes.len() != 48 {
            return Err(eyre::eyre!(
                "Invalid BLS public key length: expected 48 bytes, got {}",
                bytes.len()
            ));
        }

        let mut pubkey = [0u8; 48];
        pubkey.copy_from_slice(&bytes);
        BlsPublicKey::deserialize(&pubkey)
            .map_err(|e| eyre::eyre!("Failed to deserialize BLS public key: {:?}", e))
    }

    pub fn parse_slot(&self) -> Result<u64> {
        Ok(self
            .slot
            .parse::<u64>()
            .map_err(|e| eyre::eyre!("Failed to parse slot: {:?}", e))?)
    }

    pub fn parse_validator_index(&self) -> Result<u64> {
        Ok(self
            .validator_index
            .parse::<u64>()
            .map_err(|e| eyre::eyre!("Failed to parse validator index: {:?}", e))?)
    }
}

/// Response from Beacon API for proposer duties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposerDutiesResponse {
    /// Execution optimistic flag
    pub execution_optimistic: bool,
    /// Whether response is finalized
    pub finalized: bool,
    /// Array of proposer duties
    pub data: Vec<ValidatorDuty>,
}

/// Beacon chain state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconState {
    /// Current slot
    pub slot: u64,
    /// Current epoch
    pub epoch: u64,
}

/// Validator status information from Beacon API
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    /// Whether the validator is active (status is active_ongoing, active_exiting, or active_slashed)
    pub is_active: bool,
    /// Whether the validator has been slashed
    pub is_slashed: bool,
    /// Validator index in beacon state
    pub validator_index: u64,
}

/// Response from Beacon API for validator status query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorResponse {
    pub data: ValidatorData,
}

/// Validator data from Beacon API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorData {
    pub index: String,
    pub status: String,
    pub validator: ValidatorDetails,
}

/// Validator details from Beacon API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorDetails {
    pub pubkey: String,
    pub slashed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cb_common::types::BlsSecretKey;

    #[test]
    fn test_pubkey_parsing() {
        // Generate a valid BLS public key
        let secret_key = BlsSecretKey::random();
        let public_key = secret_key.public_key();
        let pubkey_hex = format!("0x{}", hex::encode(public_key.serialize()));

        let duty = ValidatorDuty {
            validator_index: "123".to_string(),
            pubkey: pubkey_hex.clone(),
            slot: "456".to_string(),
        };

        let parsed_pubkey = duty.parse_pubkey().unwrap();
        assert_eq!(parsed_pubkey.serialize().len(), 48);

        // Verify parsing works without 0x prefix too
        let duty_no_prefix = ValidatorDuty {
            validator_index: "123".to_string(),
            pubkey: pubkey_hex.strip_prefix("0x").unwrap().to_string(),
            slot: "456".to_string(),
        };

        let parsed_no_prefix = duty_no_prefix.parse_pubkey().unwrap();
        assert_eq!(parsed_pubkey, parsed_no_prefix);
    }

    #[test]
    fn test_validator_duty_slot_parsing() {
        // Test valid slot parsing
        let duty = ValidatorDuty {
            validator_index: "123".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "12345".to_string(),
        };

        let parsed_slot = duty.parse_slot();
        assert!(parsed_slot.is_ok());
        assert_eq!(parsed_slot.unwrap(), 12345);

        // Test invalid slot parsing
        let invalid_duty = ValidatorDuty {
            validator_index: "123".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "not_a_number".to_string(),
        };

        let result = invalid_duty.parse_slot();
        assert!(result.is_err(), "Should fail to parse invalid slot number");
    }

    #[test]
    fn test_validator_duty_pubkey_parsing_with_prefix() {
        use cb_common::types::BlsSecretKey;

        // Generate a valid BLS public key
        let secret_key = BlsSecretKey::random();
        let public_key = secret_key.public_key();
        let pubkey_hex = format!("0x{}", hex::encode(public_key.serialize()));

        let duty = ValidatorDuty {
            validator_index: "100".to_string(),
            pubkey: pubkey_hex,
            slot: "200".to_string(),
        };

        let result = duty.parse_pubkey();
        assert!(result.is_ok(), "Should parse pubkey with 0x prefix");
        assert_eq!(result.unwrap().serialize().len(), 48);
    }

    #[test]
    fn test_validator_duty_pubkey_parsing_without_prefix() {
        use cb_common::types::BlsSecretKey;

        // Generate a valid BLS public key
        let secret_key = BlsSecretKey::random();
        let public_key = secret_key.public_key();
        let pubkey_hex = hex::encode(public_key.serialize());

        let duty = ValidatorDuty {
            validator_index: "100".to_string(),
            pubkey: pubkey_hex,
            slot: "200".to_string(),
        };

        let result = duty.parse_pubkey();
        assert!(result.is_ok(), "Should parse pubkey without 0x prefix");
        assert_eq!(result.unwrap().serialize().len(), 48);
    }

    #[test]
    fn test_validator_duty_pubkey_parsing_invalid_length() {
        // Too short pubkey
        let duty = ValidatorDuty {
            validator_index: "100".to_string(),
            pubkey: "0x1234".to_string(),
            slot: "200".to_string(),
        };

        let result = duty.parse_pubkey();
        assert!(result.is_err(), "Should reject pubkey with invalid length");
    }

    #[test]
    fn test_validator_duty_pubkey_parsing_invalid_hex() {
        // Invalid hex characters
        let duty = ValidatorDuty {
			validator_index: "100".to_string(),
			pubkey:
				"0xZZZZ567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
					.to_string(),
			slot: "200".to_string(),
		};

        let result = duty.parse_pubkey();
        assert!(
            result.is_err(),
            "Should reject pubkey with invalid hex characters"
        );
    }

    #[test]
    fn test_validator_duty_index_parsing() {
        let duty = ValidatorDuty {
            validator_index: "987654".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "100".to_string(),
        };

        let result = duty.parse_validator_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 987654);
    }

    #[test]
    fn test_validator_duty_index_parsing_invalid() {
        let duty = ValidatorDuty {
            validator_index: "invalid_index".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "100".to_string(),
        };

        let result = duty.parse_validator_index();
        assert!(
            result.is_err(),
            "Should fail to parse invalid validator index"
        );
    }

    #[test]
    fn test_validator_duty_index_parsing_zero() {
        let duty = ValidatorDuty {
            validator_index: "0".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "100".to_string(),
        };

        let result = duty.parse_validator_index();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_slot_number_edge_cases() {
        // Test slot 0
        let duty_zero = ValidatorDuty {
            validator_index: "0".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "0".to_string(),
        };
        assert_eq!(duty_zero.parse_slot().unwrap(), 0);

        // Test very large slot number
        let duty_large = ValidatorDuty {
            validator_index: "0".to_string(),
            pubkey: "0xabcd".to_string(),
            slot: "18446744073709551615".to_string(), // u64::MAX
        };
        assert_eq!(duty_large.parse_slot().unwrap(), u64::MAX);
    }
}
