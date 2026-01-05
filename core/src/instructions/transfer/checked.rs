use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, AmtCheckedIxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferCheckedIxAccs<T> {
    pub src: T,
    pub mint: T,
    pub dst: T,
    pub auth: T,
}

impl_memset!(TransferCheckedIxAccs);

pub type TransferCheckedIxAccsFlag = TransferCheckedIxAccs<bool>;

pub const TRANSFER_CHECKED_IX_IS_SIGNER: TransferCheckedIxAccsFlag =
    TransferCheckedIxAccsFlag::memset(false).const_with_auth(true);

pub const TRANSFER_CHECKED_IX_IS_WRITABLE: TransferCheckedIxAccsFlag =
    TransferCheckedIxAccsFlag::memset(false)
        .const_with_src(true)
        .const_with_dst(true);

// Data

pub const TRANSFER_CHECKED_IX_DISCM: u8 = 12;

pub const TRANSFER_CHECKED_IX_DATA_LEN: usize = TransferCheckedIxData::LEN;

pub type TransferCheckedIxData = AmtCheckedIxData<TRANSFER_CHECKED_IX_DISCM>;
