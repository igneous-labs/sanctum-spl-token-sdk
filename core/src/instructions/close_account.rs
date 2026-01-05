use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, DismOnlyIxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CloseAccountIxAccs<T> {
    pub close: T,
    pub dst: T,
    pub auth: T,
}

impl_memset!(CloseAccountIxAccs);

pub type CloseAccountIxAccsFlag = CloseAccountIxAccs<bool>;

pub const CLOSE_ACCOUNT_IX_IS_SIGNER: CloseAccountIxAccsFlag =
    CloseAccountIxAccsFlag::memset(false).const_with_auth(true);

pub const CLOSE_ACCOUNT_IX_IS_WRITABLE: CloseAccountIxAccsFlag =
    CloseAccountIxAccsFlag::memset(false)
        .const_with_close(true)
        .const_with_dst(true);

// Data

pub const CLOSE_ACCOUNT_IX_DISCM: u8 = 9;

pub const CLOSE_ACCOUNT_IX_DATA_LEN: usize = CloseAccountIxData::LEN;

pub type CloseAccountIxData = DismOnlyIxData<CLOSE_ACCOUNT_IX_DISCM>;
