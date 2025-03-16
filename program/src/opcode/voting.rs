use code_vm_api::prelude::*;
use steel::*;
use solana_program::program_error::ProgramError;

use crate::ExecContext;

#[derive(Debug)]
pub struct VoteOp {
    pub signature: [u8; 64],
    pub amount: u64,
    pub poll_id: [u8; 32],
}

impl VoteOp {
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < 104 { // 64 + 8 + 32
            return Err(ProgramError::InvalidInstructionData);
        }

        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[0..64]);

        let amount = u64::from_le_bytes([
            data[64], data[65], data[66], data[67],
            data[68], data[69], data[70], data[71],
        ]);

        let mut poll_id = [0u8; 32];
        poll_id.copy_from_slice(&data[72..104]);

        Ok(VoteOp {
            signature,
            amount,
            poll_id,
        })
    }

    pub fn to_struct(self) -> Result<Self, ProgramError> {
        Ok(self)
    }
}

#[derive(Debug)]
pub struct PollInitOp {
    pub signature: [u8; 64],
    pub options_count: u8,
    pub poll_id: [u8; 32],
    pub expiry_timestamp: Option<i64>,
}

impl PollInitOp {
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < 97 { // 64 + 1 + 32
            return Err(ProgramError::InvalidInstructionData);
        }

        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[0..64]);

        let options_count = data[64];

        let mut poll_id = [0u8; 32];
        poll_id.copy_from_slice(&data[65..97]);

        let expiry_timestamp = if data.len() >= 105 {
            let timestamp_bytes = [
                data[97], data[98], data[99], data[100],
                data[101], data[102], data[103], data[104],
            ];
            Some(i64::from_le_bytes(timestamp_bytes))
        } else {
            None
        };

        Ok(PollInitOp {
            signature,
            options_count,
            poll_id,
            expiry_timestamp,
        })
    }
}

#[derive(Debug)]
pub struct PollCompleteOp {
    pub signature: [u8; 64],
    pub poll_id: [u8; 32],
}

impl PollCompleteOp {
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < 96 { // 64 + 32
            return Err(ProgramError::InvalidInstructionData);
        }

        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[0..64]);

        let mut poll_id = [0u8; 32];
        poll_id.copy_from_slice(&data[64..96]);

        Ok(PollCompleteOp {
            signature,
            poll_id,
        })
    }
}

// Helper function to create a message hash for vote verification
fn create_vote_message(
    vm: &CodeVmAccount,
    voter_vta: &VirtualTimelockAccount,
    option_vta: &VirtualTimelockAccount,
    amount: u64,
    poll_id: &[u8; 32],
    vdn: &VirtualDurableNonce,
) -> [u8; 32] {
    // Looking at other opcodes, they use hashv with byte slices
    let amount_bytes = amount.to_string().into_bytes();
    let nonce_bytes = vdn.value.to_string().into_bytes();
    let poh_bytes = vm.get_current_poh().to_string().into_bytes();
    
    hashv(&[
        voter_vta.owner.as_ref(),
        option_vta.owner.as_ref(),
        &amount_bytes,
        poll_id,
        &nonce_bytes,
        &poh_bytes,
    ]).to_bytes()
}

// Helper function to create a message hash for poll initialisation
fn create_poll_init_message(
    vm: &CodeVmAccount,
    creator_vta: &VirtualTimelockAccount,
    options_count: u8,
    poll_id: &[u8; 32],
    expiry_timestamp: Option<i64>,
    vdn: &VirtualDurableNonce,
) -> [u8; 32] {
    let options_count_bytes = options_count.to_string().into_bytes();
    let nonce_bytes = vdn.value.to_string().into_bytes();
    let poh_bytes = vm.get_current_poh().to_string().into_bytes();
    
    let mut message_parts = vec![
        creator_vta.owner.as_ref(),
        &options_count_bytes,
        poll_id,
        &nonce_bytes,
        &poh_bytes,
    ];
    
    // Create timestamp_bytes outside the if block so it lives long enough
    let timestamp_bytes;
    if let Some(timestamp) = expiry_timestamp {
        timestamp_bytes = timestamp.to_string().into_bytes();
        message_parts.push(&timestamp_bytes);
    }
    
    hashv(&message_parts).to_bytes()
}

// Helper function to create a message hash for poll completion
fn create_poll_complete_message(
    vm: &CodeVmAccount,
    creator_vta: &VirtualTimelockAccount,
    poll_id: &[u8; 32],
    vdn: &VirtualDurableNonce,
) -> [u8; 32] {
    let nonce_bytes = vdn.value.to_string().into_bytes();
    let poh_bytes = vm.get_current_poh().to_string().into_bytes();
    
    hashv(&[
        creator_vta.owner.as_ref(),
        poll_id,
        &nonce_bytes,
        &poh_bytes,
    ]).to_bytes()
}

pub fn process_vote(
    ctx: &ExecContext,
    data: &ExecIxData,
) -> ProgramResult {
    let vm = load_vm(ctx.vm_info)?;
    let args = VoteOp::try_from_bytes(&data.data)?.to_struct()?;

    let mem_indicies = &data.mem_indicies;
    let mem_banks = &data.mem_banks;
    
    // We need 3 accounts: nonce, voter, and option
    let num_accounts = 3;

    check_condition(
        mem_indicies.len() == num_accounts,
        "invalid number of memory indicies",
    )?;

    check_condition(
        mem_banks.len() == num_accounts,
        "invalid number of memory banks",
    )?;

    let nonce_index = mem_indicies[0];
    let nonce_mem = mem_banks[0];

    let voter_index = mem_indicies[1];
    let voter_mem = mem_banks[1];
    
    let option_index = mem_indicies[2];
    let option_mem = mem_banks[2];

    let vm_mem = ctx.get_banks();

    check_condition(
        vm_mem[nonce_mem as usize].is_some(),
        "the nonce memory account must be provided",
    )?;

    check_condition(
        vm_mem[voter_mem as usize].is_some(),
        "the voter memory account must be provided",
    )?;
    
    check_condition(
        vm_mem[option_mem as usize].is_some(),
        "the option memory account must be provided",
    )?;

    let nonce_mem_info = vm_mem[nonce_mem as usize].unwrap();
    let voter_mem_info = vm_mem[voter_mem as usize].unwrap();
    let option_mem_info = vm_mem[option_mem as usize].unwrap();

    let va = try_read(&nonce_mem_info, nonce_index)?;
    let mut vdn = va.into_inner_nonce().unwrap();

    let va = try_read(&voter_mem_info, voter_index)?;
    let mut voter_vta = va.into_inner_timelock().unwrap();

    let va = try_read(&option_mem_info, option_index)?;
    let mut option_vta = va.into_inner_timelock().unwrap();

    // Check if voter has sufficient funds
    if voter_vta.balance < args.amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer tokens from voter to option account
    voter_vta.balance = voter_vta.balance
        .checked_sub(args.amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    option_vta.balance = option_vta.balance
        .checked_add(args.amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Create the message hash for signature verification
    let hash = create_vote_message(
        &vm,
        &voter_vta,
        &option_vta,
        args.amount,
        &args.poll_id,
        &vdn,
    );

    // Verify the signature
    sig_verify(
        voter_vta.owner.as_ref(),
        args.signature.as_ref(),
        hash.as_ref(),
    )?;

    // Update the nonce
    vdn.value = vm.get_current_poh();

    // Write back the updated accounts
    try_write(
        voter_mem_info,
        voter_index,
        &VirtualAccount::Timelock(voter_vta)
    )?;

    try_write(
        option_mem_info,
        option_index,
        &VirtualAccount::Timelock(option_vta)
    )?;

    try_write(
        nonce_mem_info,
        nonce_index,
        &VirtualAccount::Nonce(vdn)
    )?;

    Ok(())
}

/*
    This instruction is used to initialise a new poll with a set number of options.
    The signature of the creator account is required to authorise the poll creation.

    Instruction data:

    0. signature: [u8;64]      - The signature authorising the poll creation.
    1. options_count: [u8]     - The number of options in the poll.
    2. poll_id: [u8;32]        - The unique identifier for the poll.
    3. expiry_timestamp: [i64] - Optional expiry timestamp for the poll.
*/
pub fn process_poll_init(
    ctx: &ExecContext,
    data: &ExecIxData,
) -> ProgramResult {
    let vm = load_vm(ctx.vm_info)?;
    let args = PollInitOp::try_from_bytes(&data.data)?;

    let mem_indicies = &data.mem_indicies;
    let mem_banks = &data.mem_banks;
    
    // We need at least 2 accounts: nonce and creator
    // Plus one account for each option
    let min_accounts = 2;
    let total_accounts = min_accounts + args.options_count as usize;

    check_condition(
        mem_indicies.len() == total_accounts,
        "invalid number of memory indicies",
    )?;

    check_condition(
        mem_banks.len() == total_accounts,
        "invalid number of memory banks",
    )?;

    let nonce_index = mem_indicies[0];
    let nonce_mem = mem_banks[0];

    let creator_index = mem_indicies[1];
    let creator_mem = mem_banks[1];

    let vm_mem = ctx.get_banks();

    check_condition(
        vm_mem[nonce_mem as usize].is_some(),
        "the nonce memory account must be provided",
    )?;

    check_condition(
        vm_mem[creator_mem as usize].is_some(),
        "the creator memory account must be provided",
    )?;

    let nonce_mem_info = vm_mem[nonce_mem as usize].unwrap();
    let creator_mem_info = vm_mem[creator_mem as usize].unwrap();

    let va = try_read(&nonce_mem_info, nonce_index)?;
    let mut vdn = va.into_inner_nonce().unwrap();

    let va = try_read(&creator_mem_info, creator_index)?;
    let creator_vta = va.into_inner_timelock().unwrap();

    // Create the message hash for signature verification
    let hash = create_poll_init_message(
        &vm,
        &creator_vta,
        args.options_count,
        &args.poll_id,
        args.expiry_timestamp,
        &vdn,
    );

    // Verify the signature
    sig_verify(
        creator_vta.owner.as_ref(),
        args.signature.as_ref(),
        hash.as_ref(),
    )?;

    // Initialise each option account with zero balance
    for i in 0..args.options_count as usize {
        let option_index = mem_indicies[min_accounts + i];
        let option_mem = mem_banks[min_accounts + i];

        check_condition(
            vm_mem[option_mem as usize].is_some(),
            "an option memory account must be provided",
        )?;

        let option_mem_info = vm_mem[option_mem as usize].unwrap();
        
        let va = try_read(&option_mem_info, option_index)?;
        let mut option_vta = va.into_inner_timelock().unwrap();
        
        // Ensure option account has zero balance
        option_vta.balance = 0;
        
        // Write back the option account
        try_write(
            option_mem_info,
            option_index,
            &VirtualAccount::Timelock(option_vta)
        )?;
    }

    // Update the nonce
    vdn.value = vm.get_current_poh();

    // Write back the updated nonce
    try_write(
        nonce_mem_info,
        nonce_index,
        &VirtualAccount::Nonce(vdn)
    )?;

    Ok(())
}

pub fn process_poll_complete(
    ctx: &ExecContext,
    data: &ExecIxData,
) -> ProgramResult {
    let vm = load_vm(ctx.vm_info)?;
    let args = PollCompleteOp::try_from_bytes(&data.data)?;

    let mem_indicies = &data.mem_indicies;
    let mem_banks = &data.mem_banks;
    
    // We need at least 3 accounts: nonce, creator, and winning option
    let min_accounts = 3;

    check_condition(
        mem_indicies.len() >= min_accounts,
        "invalid number of memory indicies",
    )?;

    check_condition(
        mem_banks.len() >= min_accounts,
        "invalid number of memory banks",
    )?;

    let nonce_index = mem_indicies[0];
    let nonce_mem = mem_banks[0];

    let creator_index = mem_indicies[1];
    let creator_mem = mem_banks[1];
    
    let winner_index = mem_indicies[2];
    let winner_mem = mem_banks[2];

    let vm_mem = ctx.get_banks();

    check_condition(
        vm_mem[nonce_mem as usize].is_some(),
        "the nonce memory account must be provided",
    )?;

    check_condition(
        vm_mem[creator_mem as usize].is_some(),
        "the creator memory account must be provided",
    )?;
    
    check_condition(
        vm_mem[winner_mem as usize].is_some(),
        "the winning option memory account must be provided",
    )?;

    let nonce_mem_info = vm_mem[nonce_mem as usize].unwrap();
    let creator_mem_info = vm_mem[creator_mem as usize].unwrap();
    let winner_mem_info = vm_mem[winner_mem as usize].unwrap();

    let va = try_read(&nonce_mem_info, nonce_index)?;
    let mut vdn = va.into_inner_nonce().unwrap();

    let va = try_read(&creator_mem_info, creator_index)?;
    let creator_vta = va.into_inner_timelock().unwrap();
    
    let va = try_read(&winner_mem_info, winner_index)?;
    let mut winner_vta = va.into_inner_timelock().unwrap();

    // Create the message hash for signature verification
    let hash = create_poll_complete_message(
        &vm,
        &creator_vta,
        &args.poll_id,
        &vdn,
    );

    // Verify the signature
    sig_verify(
        creator_vta.owner.as_ref(),
        args.signature.as_ref(),
        hash.as_ref(),
    )?;

    // Process other option accounts if provided
    let highest_balance = winner_vta.balance;
    let mut total_votes = highest_balance;
    
    // Check if there are additional option accounts to compare
    for i in min_accounts..mem_indicies.len() {
        let option_index = mem_indicies[i];
        let option_mem = mem_banks[i];
        
        check_condition(
            vm_mem[option_mem as usize].is_some(),
            "an option memory account must be provided",
        )?;
        
        let option_mem_info = vm_mem[option_mem as usize].unwrap();
        
        let va = try_read(&option_mem_info, option_index)?;
        let option_vta = va.into_inner_timelock().unwrap();
        
        // Add to total votes
        total_votes = total_votes
            .checked_add(option_vta.balance)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        // Verify this isn't a higher balance than our winner
        check_condition(
            option_vta.balance <= highest_balance,
            "provided winner is not the actual winner",
        )?;
        
        // Reset option account balance to zero
        let mut updated_option_vta = option_vta;
        updated_option_vta.balance = 0;
        
        // Write back the updated option account
        try_write(
            option_mem_info,
            option_index,
            &VirtualAccount::Timelock(updated_option_vta)
        )?;
    }
    
    // Reset winner account balance to zero
    winner_vta.balance = 0;
    
    // Write back the updated winner account
    try_write(
        winner_mem_info,
        winner_index,
        &VirtualAccount::Timelock(winner_vta)
    )?;

    // Update the nonce
    vdn.value = vm.get_current_poh();

    // Write back the updated nonce
    try_write(
        nonce_mem_info,
        nonce_index,
        &VirtualAccount::Nonce(vdn)
    )?;

    Ok(())
}

// Dispatch function to route to the appropriate handler based on opcode
pub fn process_voting_op(
    ctx: &ExecContext,
    data: &ExecIxData,
) -> ProgramResult {
    match data.opcode {
        // Define opcodes for voting operations
        0x01 => process_vote(ctx, data),
        0x02 => process_poll_init(ctx, data),
        0x03 => process_poll_complete(ctx, data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
