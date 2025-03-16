use code_vm_api::prelude::*;
use steel::*;
use solana_program::account_info::next_account_info;
use solana_program::program_error::ProgramError;

use crate::opcode::process_voting_op;
use crate::ExecContext;

pub fn process_voting(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Parse accounts - following the pattern from other instruction handlers
    let account_iter = &mut accounts.iter();
    
    // Required accounts
    let vm_info = next_account_info(account_iter)?;
    let vm_authority_info = next_account_info(account_iter)?;
    let external_address_info = next_account_info(account_iter)?;
    
    // Additional required accounts based on ExecContext definition
    let omnibus_info = next_account_info(account_iter)?;
    let relay_info = next_account_info(account_iter)?;
    let relay_vault_info = next_account_info(account_iter)?;
    let token_program_info = next_account_info(account_iter)?;
    
    // Memory banks - get the remaining accounts
    let mem_a_info = next_account_info(account_iter)?;
    let mem_b_info = next_account_info(account_iter)?;
    let mem_c_info = next_account_info(account_iter)?;
    let mem_d_info = next_account_info(account_iter)?;
    
    // Create the ExecContext with the correct fields
    let ctx = ExecContext {
        vm_info,
        vm_authority_info,
        external_address_info: Some(external_address_info),
        omnibus_info: Some(omnibus_info),
        relay_info: Some(relay_info),
        relay_vault_info: Some(relay_vault_info),
        token_program_info: Some(token_program_info),
        mem_a_info: Some(mem_a_info),
        mem_b_info: Some(mem_b_info),
        mem_c_info: Some(mem_c_info),
        mem_d_info: Some(mem_d_info),
    };
    
    // Parse the instruction data
    // The first byte is typically the opcode
    if data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let opcode = data[0];
    let instruction_data = &data[1..];
    
    // Parse memory indices and banks from the instruction data
    let (mem_indicies, rest) = parse_mem_indices(instruction_data)?;
    let (mem_banks, rest) = parse_mem_banks(rest)?;
    
    // Create the ExecIxData with all required fields
    let exec_data = ExecIxData {
        opcode,
        data: rest.to_vec(),
        mem_indicies,
        mem_banks,
    };
    
    // Call the voting opcode processor
    process_voting_op(&ctx, &exec_data)
}

// Helper functions to parse the instruction data
fn parse_mem_indices(data: &[u8]) -> Result<(Vec<u16>, &[u8]), ProgramError> {
    if data.len() < 2 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let count = data[0] as usize;
    let required_len = 1 + (count * 2); // 1 byte for count + 2 bytes per index
    
    if data.len() < required_len {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let mut indices = Vec::with_capacity(count);
    for i in 0..count {
        let start = 1 + (i * 2);
        let index = u16::from_le_bytes([data[start], data[start + 1]]);
        indices.push(index);
    }
    
    Ok((indices, &data[required_len..]))
}

fn parse_mem_banks(data: &[u8]) -> Result<(Vec<u8>, &[u8]), ProgramError> {
    if data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let count = data[0] as usize;
    let required_len = 1 + count; // 1 byte for count + 1 byte per bank
    
    if data.len() < required_len {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let mut banks = Vec::with_capacity(count);
    for i in 0..count {
        banks.push(data[1 + i]);
    }
    
    Ok((banks, &data[required_len..]))
}