use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{impl_memset, AddrIxData};

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InitAcc3IxAccs<T> {
    /// The token account to be initialized.
    ///
    /// Must be
    /// - rent exempt
    /// - owner set to token program
    /// - allocated to token account space
    pub init: T,

    /// Mint that `init` is of
    pub mint: T,
}

impl_memset!(InitAcc3IxAccs);

pub type InitAcc3IxAccFlags = InitAcc3IxAccs<bool>;

pub const INIT_ACC3_IX_IS_SIGNER: InitAcc3IxAccFlags = InitAcc3IxAccFlags::memset(false);

pub const INIT_ACC3_IX_IS_WRITABLE: InitAcc3IxAccFlags =
    InitAcc3IxAccFlags::memset(false).const_with_init(true);

// Data

pub const INIT_ACC3_IX_DISCM: u8 = 18;

pub const INIT_ACC3_IX_DATA_LEN: usize = InitAcc3IxData::LEN;

pub type InitAcc3IxData = AddrIxData<INIT_ACC3_IX_DISCM>;
