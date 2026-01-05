use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, U64IxData};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferIxAccs<T> {
    pub src: T,
    pub dst: T,
    pub auth: T,
}

impl_memset!(TransferIxAccs);

pub type TransferIxAccsFlag = TransferIxAccs<bool>;

pub const TRANSFER_IX_IS_SIGNER: TransferIxAccsFlag =
    TransferIxAccsFlag::memset(false).const_with_auth(true);

pub const TRANSFER_IX_IS_WRITABLE: TransferIxAccsFlag = TransferIxAccsFlag::memset(false)
    .const_with_src(true)
    .const_with_dst(true);

// Data

pub const TRANSFER_IX_DISCM: u8 = 3;

pub const TRANSFER_IX_DATA_LEN: usize = TransferIxData::LEN;

pub type TransferIxData = U64IxData<TRANSFER_IX_DISCM>;
