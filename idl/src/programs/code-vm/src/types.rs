use anchor_lang::prelude::*;

const NUM_ACCOUNTS: usize = 100;

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

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum VirtualAccount {
    Timelock = 1,
    DurableNonce = 2,
    Relay = 3,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct VirtualTimelockAccount {
    kind: VirtualAccount,

    pub owner: Pubkey,
    pub instance: Hash,

    pub token_bump: u8,
    pub unlock_bump: u8,
    pub withdraw_bump: u8,

    pub balance: u64,
    pub bump: u8,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct VirtualDurableNonce {
    kind: VirtualAccount,

    pub address: Pubkey,
    pub value: Hash,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct VirtualRelayAccount {
    kind: VirtualAccount,

    pub target: Pubkey,
    pub destination: Pubkey,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum ItemState {
    Empty = 0,
    Allocated = 1,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub struct SimpleAllocator<const N: usize, T> {
    pub state: [ItemState; N],
    pub data: [T; N],
}


#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum AccountData {
    Nonce(SimpleAllocator<{NUM_ACCOUNTS}, VirtualDurableNonce>) = 1,
    Timelock(SimpleAllocator<{NUM_ACCOUNTS}, VirtualTimelockAccount>) = 2,
    Relay(SimpleAllocator<{NUM_ACCOUNTS}, VirtualRelayAccount>) = 3,
}
