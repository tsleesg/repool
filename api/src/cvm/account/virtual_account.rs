use steel::*;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::Hash;
use crate::prelude::DualTokenAccount;

use super::{
    VirtualDurableNonce,
    VirtualTimelockAccount,
    VirtualRelayAccount,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub enum VirtualAccount {
    Nonce(VirtualDurableNonce),
    Timelock(VirtualTimelockAccount),
    DualToken(DualTokenAccount),
    Relay(VirtualRelayAccount),
}

impl VirtualAccount {
    /// Get the size of this enum
    pub fn get_size(&self) -> usize {
        1 + (match self {
            VirtualAccount::Nonce(_) => VirtualDurableNonce::LEN,
            VirtualAccount::Timelock(_) => VirtualTimelockAccount::LEN,
            VirtualAccount::DualToken(_) => DualTokenAccount::LEN,
            VirtualAccount::Relay(_) => VirtualRelayAccount::LEN,
        })
    }

    pub fn is_timelock(&self) -> bool {
        matches!(self, VirtualAccount::Timelock(_))
    }

    pub fn is_relay(&self) -> bool {
        matches!(self, VirtualAccount::Relay(_))
    }

    pub fn is_nonce(&self) -> bool {
        matches!(self, VirtualAccount::Nonce(_))
    }

    pub fn is_dual_token(&self) -> bool {
        matches!(self, VirtualAccount::DualToken(_))
    }

    pub fn get_hash(&self) -> Hash {
        match self {
            VirtualAccount::Nonce(_) => Hash::new(&self.pack()),
            VirtualAccount::Timelock(_) => Hash::new(&self.pack()),
            VirtualAccount::DualToken(inner) => Hash::new(&inner.instance),
            VirtualAccount::Relay(_) => Hash::new(&self.pack()),
        }
    }

    /// Pack this VirtualAccount into a byte array
    pub fn pack(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.get_size()];
        bytes[0] = match self {
            VirtualAccount::Nonce(_) => 0,
            VirtualAccount::Timelock(_) => 1,
            VirtualAccount::DualToken(_) => 2,
            VirtualAccount::Relay(_) => 3,
        };

        match self {
            VirtualAccount::Nonce(account) => {
                account.pack(&mut bytes[1..]).unwrap();
            },
            VirtualAccount::Timelock(account) => {
                account.pack(&mut bytes[1..]).unwrap();
            },
            VirtualAccount::DualToken(account) => {
                account.pack(&mut bytes[1..]).unwrap();
            },
            VirtualAccount::Relay(account) => {
                account.pack(&mut bytes[1..]).unwrap();
            },
        }
        bytes
    }

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < 1 {
            return Err(ProgramError::InvalidAccountData);
        }

        let variant = input[0];
        let data = &input[1..];
        let size = get_variant_size(variant);

        if data.len() < size {
            return Err(ProgramError::InvalidAccountData);
        }

        match variant {
            0 => Ok(VirtualAccount::Nonce(
                VirtualDurableNonce::unpack(data).unwrap()
            )),
            1 => Ok(VirtualAccount::Timelock(
                VirtualTimelockAccount::unpack(data).unwrap()
            )),
            2 => Ok(VirtualAccount::DualToken(
                DualTokenAccount::unpack(data).unwrap()
            )),
            3 => Ok(VirtualAccount::Relay(
                VirtualRelayAccount::unpack(data).unwrap()
            )),
            _ => Err(ProgramError::InvalidAccountData)
        }
    }
}

impl VirtualAccount {
    pub fn into_inner_nonce(self) -> Option<VirtualDurableNonce> {
        match self {
            VirtualAccount::Nonce(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_inner_timelock(self) -> Option<VirtualTimelockAccount> {
        match self {
            VirtualAccount::Timelock(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_inner_relay(self) -> Option<VirtualRelayAccount> {
        match self {
            VirtualAccount::Relay(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn into_inner_dual_token(self) -> Option<DualTokenAccount> {
        match self {
            VirtualAccount::DualToken(inner) => Some(inner),
            _ => None,
        }
    }
}

fn get_variant_size(variant: u8) -> usize {
    match variant {
        0 => VirtualDurableNonce::LEN,
        1 => VirtualTimelockAccount::LEN,
        2 => DualTokenAccount::LEN,
        3 => VirtualRelayAccount::LEN,
        _ => 0,
    }
}