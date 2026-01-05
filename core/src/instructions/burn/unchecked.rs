use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, U64IxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BurnIxAccs<T> {
    pub from: T,
    pub mint: T,
    pub auth: T,
}

impl_memset!(BurnIxAccs);

pub type BurnIxAccsFlag = BurnIxAccs<bool>;

pub const BURN_IX_IS_SIGNER: BurnIxAccsFlag = BurnIxAccsFlag::memset(false).const_with_auth(true);

pub const BURN_IX_IS_WRITABLE: BurnIxAccsFlag = BurnIxAccsFlag::memset(false)
    .const_with_from(true)
    .const_with_mint(true);

// Data

pub const BURN_IX_DISCM: u8 = 8;

pub const BURN_IX_DATA_LEN: usize = BurnIxData::LEN;

pub type BurnIxData = U64IxData<BURN_IX_DISCM>;
