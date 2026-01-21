use commit_boost::prelude::Chain;

use crate::constants::{SLOT_DURATION_MS, SLOT_DURATION_SECONDS, SLOTS_PER_EPOCH};

/// Converts a slot number to its corresponding epoch.
///
/// # Examples
///
pub fn slot_to_epoch(slot: u64) -> u64 {
	slot / SLOTS_PER_EPOCH
}

/// Compute the first slot index of the given epoch.
///
/// # Examples
///
pub fn epoch_to_first_slot(epoch: u64) -> u64 {
	epoch * SLOTS_PER_EPOCH
}

/// Compute the last slot index of a given epoch.
///
/// # Examples
///
pub fn epoch_to_last_slot(epoch: u64) -> u64 {
	(epoch + 1) * SLOTS_PER_EPOCH - 1
}

/// Estimate the current beacon slot from the chain genesis time.
///
/// Returns the slot index computed from the difference between the current system time and `genesis_time`.
/// If the current system time is before `genesis_time`, this returns `0`.
///
/// # Examples
///
pub fn current_slot_estimate(genesis_time: u64) -> u64 {
	let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

	if now < genesis_time {
		return 0;
	}

	(now - genesis_time) / SLOT_DURATION_SECONDS
}

/// Compute the number of milliseconds from the current system time until the start of a given slot.
///
/// The returned value is negative if the slot has already started.
///
/// # Parameters
///
/// - `genesis_time`: Unix epoch seconds when the chain genesis occurred.
/// - `target_slot`: Slot number whose start time is being queried.
///
/// # Returns
///
/// `i64` number of milliseconds until the start of `target_slot`; negative if the slot start time is in the past.
///
/// # Examples
///
pub fn time_until_slot_ms(genesis_time: u64, target_slot: u64) -> i64 {
	let slot_start_time_ms = (genesis_time * 1000) + (target_slot * SLOT_DURATION_MS);
	let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

	slot_start_time_ms as i64 - now_ms as i64
}

pub fn current_slot(chain: &Chain) -> u64 {
	current_slot_estimate(chain.genesis_time_sec())
}

pub fn time_until_next_slot_ms(chain: &Chain) -> i64 {
	let genesis_time = chain.genesis_time_sec();
	time_until_slot_ms(genesis_time, current_slot_estimate(genesis_time) + 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_epoch_calculations() {
		assert_eq!(slot_to_epoch(0), 0);
		assert_eq!(slot_to_epoch(31), 0);
		assert_eq!(slot_to_epoch(32), 1);
		assert_eq!(slot_to_epoch(63), 1);
		assert_eq!(slot_to_epoch(64), 2);

		assert_eq!(epoch_to_first_slot(0), 0);
		assert_eq!(epoch_to_first_slot(1), 32);
		assert_eq!(epoch_to_first_slot(2), 64);

		assert_eq!(epoch_to_last_slot(0), 31);
		assert_eq!(epoch_to_last_slot(1), 63);
		assert_eq!(epoch_to_last_slot(2), 95);
	}

	#[test]
	fn test_time_until_slot_ms_future_slot() {
		// Use current time as genesis, so slot 1 should be ~12 seconds in the future
		// Note: genesis_time is in seconds, but function returns milliseconds
		// There's up to 999ms discrepancy due to sub-second timing
		let now_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

		let time_until = time_until_slot_ms(now_secs, 1);

		// Slot 1 starts at genesis + 12 seconds = 12000ms from genesis
		// Allow up to 1000ms tolerance for sub-second timing discrepancy
		assert!(time_until > 11_000, "Expected ~12000ms, got {}", time_until);
		assert!(time_until <= 12_000, "Expected ~12000ms, got {}", time_until);
	}

	#[test]
	fn test_time_until_slot_ms_past_slot() {
		// Use a genesis time 1 minute in the past
		let now_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		let genesis_time = now_secs - 60; // 60 seconds ago

		// Slot 0 started at genesis (60 seconds ago)
		let time_until = time_until_slot_ms(genesis_time, 0);

		// Should be approximately -60000ms (negative because it's in the past)
		// Allow up to 1000ms tolerance for sub-second timing discrepancy
		assert!(time_until < -59_000, "Expected ~-60000ms, got {}", time_until);
		assert!(time_until >= -61_000, "Expected ~-60000ms, got {}", time_until);
	}

	#[test]
	fn test_time_until_slot_ms_returns_milliseconds() {
		// Verify that the function returns milliseconds, not seconds
		let now_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

		// Slot 10 starts at genesis + (10 * 12) = 120 seconds = 120000ms
		let time_until = time_until_slot_ms(now_secs, 10);

		// Should be around 120000ms, not 120
		assert!(time_until > 100_000, "Expected milliseconds, got {}", time_until);
	}

	#[test]
	fn test_time_until_slot_ms_slot_boundary() {
		// Test at slot boundary: if we're exactly at slot N, time until slot N should be ~0
		// Note: there's up to 999ms discrepancy because genesis_time is in seconds
		let now_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

		// At genesis (slot 0), time_until_slot_ms(genesis, 0) should be close to 0
		let time_until = time_until_slot_ms(now_secs, 0);

		// Should be between -1000ms and 0ms (negative due to sub-second elapsed time)
		assert!(time_until <= 0, "Expected <= 0ms at slot boundary, got {}", time_until);
		assert!(time_until > -1000, "Expected > -1000ms at slot boundary, got {}", time_until);
	}

	#[test]
	fn test_time_until_slot_ms_calculation_accuracy() {
		// Test the mathematical correctness of the slot time calculation
		// by comparing two consecutive slots
		let genesis_time: u64 = 1_700_000_000; // Fixed genesis time

		let time_until_slot_5 = time_until_slot_ms(genesis_time, 5);
		let time_until_slot_6 = time_until_slot_ms(genesis_time, 6);

		// The difference between consecutive slots should be exactly 12000ms (SLOT_DURATION_MS)
		let difference = time_until_slot_6 - time_until_slot_5;
		assert_eq!(difference, SLOT_DURATION_MS as i64, "Slot duration should be 12000ms");
	}
}
