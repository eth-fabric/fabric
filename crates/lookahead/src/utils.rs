use commit_boost::prelude::Chain;

use crate::constants::{SLOT_DURATION_SECONDS, SLOTS_PER_EPOCH};

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
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if now < genesis_time {
        return 0;
    }

    (now - genesis_time) / SLOT_DURATION_SECONDS
}

/// Compute the number of seconds from the current system time until the start of a given slot.
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
/// `i64` number of seconds until the start of `target_slot`; negative if the slot start time is in the past.
///
/// # Examples
///
pub fn time_until_slot(genesis_time: u64, target_slot: u64) -> i64 {
    let slot_start_time = genesis_time + (target_slot * SLOT_DURATION_SECONDS);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    slot_start_time as i64 - now as i64
}

pub fn current_slot(chain: &Chain) -> u64 {
    current_slot_estimate(chain.genesis_time_sec())
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
}
