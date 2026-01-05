use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, U64IxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MintToIxAccs<T> {
    pub mint: T,
    pub to: T,
    pub auth: T,
}

impl_memset!(MintToIxAccs);

pub type MintToIxAccsFlag = MintToIxAccs<bool>;

pub const MINT_TO_IX_IS_SIGNER: MintToIxAccsFlag =
    MintToIxAccsFlag::memset(false).const_with_auth(true);

pub const MINT_TO_IX_IS_WRITABLE: MintToIxAccsFlag = MintToIxAccsFlag::memset(false)
    .const_with_mint(true)
    .const_with_to(true);

// Data

pub const MINT_TO_IX_DISCM: u8 = 7;

pub const MINT_TO_IX_DATA_LEN: usize = MintToIxData::LEN;

pub type MintToIxData = U64IxData<MINT_TO_IX_DISCM>;
