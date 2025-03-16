use steel::*;

use crate::utils;
use crate::types::Hash;
use crate::cvm::{
    CodeVmAccount,
    VirtualDurableNonce, 
    VirtualTimelockAccount
};

/// Helper function to get token addresses for a virtual timelock account
fn get_token_address(
    vm: &CodeVmAccount,
    vta: &VirtualTimelockAccount,
) -> Pubkey {
    let timelock_address = vta.get_timelock_address(
        &vm.get_mint(),
        &vm.get_authority(),
        vm.get_lock_duration(),
    );
    vta.get_token_address(&timelock_address)
}

/// Generic function to create a compact message with variable components
fn create_compact_message(
    message_type: &[u8],
    components: &[&[u8]],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let mut message_parts = Vec::with_capacity(components.len() + 3);
    message_parts.push(message_type);
    message_parts.extend_from_slice(components);
    message_parts.push(vdn.address.as_ref());
    message_parts.push(vdn.value.as_ref());
    
    utils::hashv(&message_parts)
}

/// Creates a compact message for a direct vote operation
pub fn compact_vote_message(
    voter_token_address: &Pubkey,
    option_token_address: &Pubkey,
    amount: u64,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let amount_bytes = amount.to_le_bytes();
    
    create_compact_message(
        b"direct_vote",
        &[
            voter_token_address.as_ref(),
            option_token_address.as_ref(),
            amount_bytes.as_ref(),
            poll_id,
        ],
        vdn
    )
}

/// Creates a compact message for a poll initialisation
pub fn compact_poll_init_message(
    creator_timelock_address: &Pubkey,
    poll_id: &[u8],
    options_count: u8,
    vdn: &VirtualDurableNonce,
) -> Hash {
    let options_count_bytes = [options_count];
    
    create_compact_message(
        b"poll_init",
        &[
            creator_timelock_address.as_ref(),
            poll_id,
            &options_count_bytes,
        ],
        vdn
    )
}

/// Creates a compact message for a poll initialisation with expiry timestamp
pub fn compact_poll_init_message_with_expiry(
    creator_timelock_address: &Pubkey,
    poll_id: &[u8],
    options_count: u8,
    expiry_timestamp: i64,
    vdn: &VirtualDurableNonce,
) -> Hash {
    let options_count_bytes = [options_count];
    let expiry_timestamp_bytes = expiry_timestamp.to_le_bytes();
    
    create_compact_message(
        b"poll_init_with_expiry",
        &[
            creator_timelock_address.as_ref(),
            poll_id,
            &options_count_bytes,
            &expiry_timestamp_bytes,
        ],
        vdn
    )
}

/// Creates a compact message for poll completion and result finalisation
pub fn compact_poll_complete_message(
    creator_timelock_address: &Pubkey,
    winning_timelock_address: &Pubkey,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    create_compact_message(
        b"poll_complete",
        &[
            creator_timelock_address.as_ref(),
            winning_timelock_address.as_ref(),
            poll_id,
        ],
        vdn
    )
}

/// Creates a direct vote message for a specific poll option
pub fn create_vote_message(
    vm: &CodeVmAccount,
    voter_vta: &VirtualTimelockAccount,
    option_vta: &VirtualTimelockAccount,
    amount: u64,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let voter_token_address = get_token_address(vm, voter_vta);
    let option_token_address = get_token_address(vm, option_vta);

    compact_vote_message(
        &voter_token_address,
        &option_token_address,
        amount,
        poll_id,
        vdn,
    )
}

/// Creates a poll initialisation message
pub fn create_poll_init_message(
    vm: &CodeVmAccount,
    creator_vta: &VirtualTimelockAccount,
    options_count: u8,
    external_poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let creator_timelock_address = get_token_address(vm, creator_vta);
    
    compact_poll_init_message(
        &creator_timelock_address,
        external_poll_id,
        options_count,
        vdn,
    )
}

/// Creates a poll initialisation message with expiry timestamp
pub fn create_poll_init_message_with_expiry(
    vm: &CodeVmAccount,
    creator_vta: &VirtualTimelockAccount,
    options_count: u8,
    external_poll_id: &[u8],
    expiry_timestamp: i64,
    vdn: &VirtualDurableNonce,
) -> Hash {
    let creator_timelock_address = get_token_address(vm, creator_vta);
    
    compact_poll_init_message_with_expiry(
        &creator_timelock_address,
        external_poll_id,
        options_count,
        expiry_timestamp,
        vdn,
    )
}

/// Creates a poll completion message
pub fn create_poll_complete_message(
    vm: &CodeVmAccount,
    creator_vta: &VirtualTimelockAccount,
    winning_vta: &VirtualTimelockAccount,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let creator_timelock_address = get_token_address(vm, creator_vta);
    let winning_timelock_address = get_token_address(vm, winning_vta);
    
    compact_poll_complete_message(
        &creator_timelock_address,
        &winning_timelock_address,
        poll_id,
        vdn,
    )
}

/// Creates a direct transfer message for voting
pub fn create_direct_vote_transfer(
    vm: &CodeVmAccount,
    voter_vta: &VirtualTimelockAccount,
    option_vta: &VirtualTimelockAccount,
    amount: u64,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let voter_token_address = get_token_address(vm, voter_vta);
    let option_token_address = get_token_address(vm, option_vta);
    
    create_compact_message(
        b"direct_vote_transfer",
        &[
            voter_token_address.as_ref(),
            option_token_address.as_ref(),
            amount.to_le_bytes().as_ref(),
            poll_id,
        ],
        vdn
    )
}

/// Create a refund message for returning votes if poll is cancelled
pub fn create_vote_refund_message(
    vm: &CodeVmAccount,
    option_vta: &VirtualTimelockAccount,
    voter_vta: &VirtualTimelockAccount,
    amount: u64,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let option_token_address = get_token_address(vm, option_vta);
    let voter_token_address = get_token_address(vm, voter_vta);
    
    create_compact_message(
        b"vote_refund",
        &[
            option_token_address.as_ref(),
            voter_token_address.as_ref(),
            amount.to_le_bytes().as_ref(),
            poll_id,
        ],
        vdn
    )
}

/// Create a message for distributing rewards to winning option voters
pub fn create_reward_distribution_message(
    vm: &CodeVmAccount,
    treasury_vta: &VirtualTimelockAccount,
    voter_vta: &VirtualTimelockAccount,
    amount: u64,
    poll_id: &[u8],
    vdn: &VirtualDurableNonce,
) -> Hash {
    let treasury_token_address = get_token_address(vm, treasury_vta);
    let voter_token_address = get_token_address(vm, voter_vta);
    
    create_compact_message(
        b"reward_distribution",
        &[
            treasury_token_address.as_ref(),
            voter_token_address.as_ref(),
            amount.to_le_bytes().as_ref(),
            poll_id,
        ],
        vdn
    )
}