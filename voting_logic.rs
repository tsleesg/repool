use steel::*;

use crate::utils;
use crate::cvm::{
    CodeVmAccount,
    VirtualTimelockAccount
};

/// Structure to represent poll metadata
pub struct PollMetadata {
    pub creator: Pubkey,
    pub options_count: u8,
    pub creation_timestamp: i64,
    pub expiry_timestamp: i64,
    pub poll_id: Vec<u8>,
}

/// Calculate the winning option based on vote amounts
pub fn calculate_winning_option(
    options: &[VirtualTimelockAccount],
    vote_amounts: &[u64],
    vote_options: &[usize],
) -> usize {
    // Initialise vote totals for each option
    let mut option_totals = vec![0u64; options.len()];
    
    // Sum up votes for each option
    for (i, &option_idx) in vote_options.iter().enumerate() {
        if option_idx < options.len() {
            option_totals[option_idx] += vote_amounts[i];
        }
    }
    
    // Find the option with the highest vote total
    let mut max_votes = 0;
    let mut winning_idx = 0;
    
    for (i, &total) in option_totals.iter().enumerate() {
        if total > max_votes {
            max_votes = total;
            winning_idx = i;
        }
    }
    
    winning_idx
}

/// Create a new poll with expiry timestamp
pub fn create_poll_metadata(
    creator_vta: &VirtualTimelockAccount,
    options_count: u8,
    external_data: &[u8],
    poll_duration_seconds: i64,
    vm: &CodeVmAccount,
) -> Result<PollMetadata, &'static str> {
    // Get the current timestamp using the same approach as in unlock.rs
    let current_timestamp = Clock::get().map_err(|_| "Failed to get clock")?.unix_timestamp;
    
    // Calculate poll expiry timestamp
    let expiry_timestamp = current_timestamp + poll_duration_seconds;
    
    // Ensure the poll expires before the lock duration ends
    if !is_poll_lock_compatible(expiry_timestamp, vm)? {
        return Err("Poll expiry must be before lock duration ends");
    }
    
    // Generate a unique poll ID
    let poll_id = generate_poll_id(creator_vta, external_data);
    
    Ok(PollMetadata {
        creator: creator_vta.owner,
        options_count,
        creation_timestamp: current_timestamp,
        expiry_timestamp,
        poll_id,
    })
}

/// Check if a poll is still active
pub fn is_poll_active(poll_metadata: &PollMetadata) -> Result<bool, &'static str> {
    let current_timestamp = Clock::get().map_err(|_| "Failed to get clock")?.unix_timestamp;
    Ok(current_timestamp < poll_metadata.expiry_timestamp)
}

/// Verify that a vote meets the conditions for a valid vote
pub fn verify_vote_validity(
    voter_vta: &VirtualTimelockAccount,
    amount: u64,
    option_idx: usize,
    options: &[VirtualTimelockAccount],
) -> bool {
    // Check that the voter has sufficient balance
    if voter_vta.balance < amount {
        return false;
    }
    
    // Check that the option index is valid
    if option_idx >= options.len() {
        return false;
    }
    
    true
}

/// Verify that a vote meets the conditions for a valid vote, including lock duration constraints
pub fn verify_vote_validity_with_expiry(
    voter_vta: &VirtualTimelockAccount,
    amount: u64,
    option_idx: usize,
    options: &[VirtualTimelockAccount],
    poll_expiry_timestamp: i64,
    vm: &CodeVmAccount,
) -> Result<bool, &'static str> {
    // Check basic validity first
    if !verify_vote_validity(voter_vta, amount, option_idx, options) {
        return Ok(false);
    }
    
    // Check that the poll expires before the lock duration ends
    Ok(is_poll_lock_compatible(poll_expiry_timestamp, vm)?)
}

/// Generate a unique poll ID based on creator and external data
pub fn generate_poll_id(
    creator_vta: &VirtualTimelockAccount,
    external_data: &[u8],
) -> Vec<u8> {
    let message = &[
        b"poll_id",
        creator_vta.owner.as_ref(),
        external_data,
    ];
    
    utils::hashv(message).to_bytes().to_vec()
}

/// Check if a poll's lock duration is compatible with the VM's lock duration
pub fn is_poll_lock_compatible(
    poll_expiry_timestamp: i64,
    vm: &CodeVmAccount,
) -> Result<bool, &'static str> {
    // Get the current timestamp
    let current_timestamp = Clock::get().map_err(|_| "Failed to get clock")?.unix_timestamp;
    
    // Get the lock duration from the VM (in days)
    let lock_duration = vm.get_lock_duration();
    
    // Convert lock duration (in days) to seconds
    let lock_duration_seconds = (lock_duration as i64) * 24 * 60 * 60;
    
    // Calculate when the lock would expire if started now
    let lock_expiry_timestamp = current_timestamp + lock_duration_seconds;
    
    // Check that the poll expires before the lock duration ends
    Ok(poll_expiry_timestamp < lock_expiry_timestamp)
}

/// Calculate the maximum poll duration that would be compatible with the VM's lock duration
pub fn calculate_max_poll_duration(
    vm: &CodeVmAccount,
) -> Result<i64, &'static str> {
    // Get the lock duration from the VM (in days)
    let lock_duration = vm.get_lock_duration();
    
    // Convert lock duration (in days) to seconds
    // Subtract a safety margin (e.g., 1 hour) to ensure compatibility
    let safety_margin = 60 * 60; // 1 hour in seconds
    let max_poll_duration = (lock_duration as i64) * 24 * 60 * 60 - safety_margin;
    
    Ok(max_poll_duration)
}