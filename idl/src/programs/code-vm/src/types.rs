use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use sha2::Digest;

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct Hash {
    pub(crate) value: [u8; 32]
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct Signature {
    pub(crate) value: [u8; 64]
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct TokenPool {
    pub vault: Pubkey,
    pub vault_bump: u8,
}



#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct CircularBuffer<const N: usize, const M: usize> {
    pub items: [[u8; M]; N],
    pub offset: u8,
    pub num_items: u8,
    _padding: [u8; 6],
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct MerkleTree<const N: usize> {
    pub root: Hash,
    pub filled_subtrees: [Hash; N],
    pub zero_values: [Hash; N],
    pub next_index: u64,
}

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, PartialEq, Debug)]
pub struct DualTokenAccount {
    pub instance: Vec<u8>,
    pub owner: Pubkey,
    pub kin_balance: u64,
    pub second_token_balance: u64,
    pub ratio: Ratio,
    pub lock_duration: u8,
    pub unlock_time: i64,
    pub instance_hash: Hash,  // Renamed from instance to avoid duplicate field
    pub bump: u8,
    pub token_bump: u8,
    pub unlock_bump: u8,
    pub withdraw_bump: u8,
}

impl DualTokenAccount {
    pub const LEN: usize = 32 + 32 + 8 + 8 + std::mem::size_of::<Ratio>() + 1 + 8 + 32 + 4;
    
    pub fn pack(&self, dst: &mut [u8]) -> Result<(), ProgramError> {
        self.serialize(dst).map_err(|_| ProgramError::InvalidAccountData)
    }
    
    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct Ratio {
    pub kin_parts: u64,
    pub token_parts: u64,
    pub ratio_hash: Hash,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum VirtualAccount {
    Timelock(VirtualTimelockAccount),
    DualToken(DualTokenAccount),
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct VirtualTimelockAccount {
    pub owner: Pubkey,
    pub instance: Hash,
    pub bump: u8,
    pub token_bump: u8,
    pub unlock_bump: u8,
    pub withdraw_bump: u8,
    pub balance: u64,
}

impl VirtualTimelockAccount {
    pub const LEN: usize = 32 + 32 + 1 + 1 + 1 + 1 + 8;
}

// Existing impls remain the same
impl VirtualAccount {
    pub fn into_inner_timelock(self) -> Option<VirtualTimelockAccount> {
        match self {
            VirtualAccount::Timelock(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_inner_dual_token(self) -> Option<DualTokenAccount> {
        match self {
            VirtualAccount::DualToken(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn get_hash(&self) -> Hash {
        match self {
            VirtualAccount::Timelock(inner) => Hash::new(&inner.instance.value),
            VirtualAccount::DualToken(inner) => Hash::new(&inner.instance_hash.value),
        }
    }
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

impl Hash {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut value = [0u8; 32];
        value.copy_from_slice(&result[..]);
        Self { value }
    }
}