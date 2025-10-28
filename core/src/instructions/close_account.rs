use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::impl_memset;

// Accounts

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CloseAccountIxAccs<T> {
    pub account_to_close: T,
    pub refund_rent_to: T,
    pub auth: T,
}

impl<T: Copy> CloseAccountIxAccs<T> {
    impl_memset!(CLOSE_ACCOUNT_IX_ACCS_LEN);
}

pub type CloseAccountIxAccsFlag = CloseAccountIxAccs<bool>;

pub const CLOSE_ACCOUNT_IX_IS_SIGNER: CloseAccountIxAccsFlag =
    CloseAccountIxAccsFlag::memset(false).const_with_auth(true);

pub const CLOSE_ACCOUNT_IX_IS_WRITABLE: CloseAccountIxAccsFlag =
    CloseAccountIxAccsFlag::memset(false)
        .const_with_account_to_close(true)
        .const_with_refund_rent_to(true);

// Data

pub const CLOSE_ACCOUNT_IX_DISCM: u8 = 9;

pub const CLOSE_ACCOUNT_IX_DATA_LEN: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CloseAccountIxData;

impl CloseAccountIxData {
    pub const DATA: u8 = CLOSE_ACCOUNT_IX_DISCM;

    #[inline]
    pub const fn as_buf() -> &'static [u8; CLOSE_ACCOUNT_IX_DATA_LEN] {
        &[Self::DATA]
    }
}
