use steel::*;
use borsh::{BorshDeserialize, BorshSerialize};
use super::Hash;

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, PartialEq, Debug)]
pub struct Ratio {
    pub kin_parts: u64,
    pub token_parts: u64,
    pub ratio_hash: Hash,
}

impl Ratio {
    pub fn new(kin_parts: u64, token_parts: u64) -> Self {
        let ratio_hash = Hash::new(&[kin_parts.to_le_bytes(), token_parts.to_le_bytes()].concat());
        Self {
            kin_parts,
            token_parts,
            ratio_hash,
        }
    }

    pub fn verify(&self, kin_amount: u64, token_amount: u64) -> bool {
        (kin_amount * self.token_parts) == (token_amount * self.kin_parts)
    }
}

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, PartialEq, Debug)]
pub struct DualDepositIx {
    pub account_index: u16,
    pub kin_amount: u64,
    pub token_amount: u64,
    pub lock_duration: u8,
    pub bump: u8,
    pub kin_bump: u8,
    pub token_bump: u8,
    pub unlock_bump: u8,
    pub withdraw_bump: u8,
}

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub struct DualTokenAccount {
    pub instance: Vec<u8>,
    pub owner: Pubkey,
    pub kin_balance: u64,
    pub second_token_balance: u64,
    pub ratio: Ratio,
    pub lock_duration: u8,
    pub unlock_time: i64,
    pub instance_hash: Hash,
    pub bump: u8,
    pub token_bump: u8,
    pub unlock_bump: u8,
    pub withdraw_bump: u8,
}

impl DualTokenAccount {
    pub const LEN: usize = 32 + 32 + 8 + 8 + std::mem::size_of::<Ratio>() + 1 + 8 + 32 + 4;
    
    pub fn pack(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        let mut writer = std::io::Cursor::new(dst);
        self.serialize(&mut writer).map_err(|_| ProgramError::InvalidAccountData)
    }    
    
    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
    }
}