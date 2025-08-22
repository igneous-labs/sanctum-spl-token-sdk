use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{caba, impl_memset};

// Accounts

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferCheckedIxAccs<T> {
    pub src: T,
    pub mint: T,
    pub dst: T,
    pub auth: T,
}

impl<T: Copy> TransferCheckedIxAccs<T> {
    impl_memset!(TRANSFER_CHECKED_IX_ACCS_LEN);
}

pub type TransferCheckedIxAccsFlag = TransferCheckedIxAccs<bool>;

pub const TRANSFER_CHECKED_IX_IS_SIGNER: TransferCheckedIxAccsFlag =
    TransferCheckedIxAccsFlag::memset(false).const_with_auth(true);

pub const TRANSFER_CHECKED_IX_IS_WRITABLE: TransferCheckedIxAccsFlag =
    TransferCheckedIxAccsFlag::memset(false)
        .const_with_src(true)
        .const_with_dst(true);

// Data

pub const TRANSFER_CHECKED_IX_DISCM: u8 = 12;

pub const TRANSFER_CHECKED_IX_DATA_LEN: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferCheckedIxData([u8; TRANSFER_CHECKED_IX_DATA_LEN]);

impl TransferCheckedIxData {
    #[inline]
    pub const fn new(amount: u64, decimals: u8) -> Self {
        const A: usize = TRANSFER_CHECKED_IX_DATA_LEN;

        let mut d = [0u8; A];

        d = caba::<A, 0, 1>(d, &[TRANSFER_CHECKED_IX_DISCM]);
        d = caba::<A, 1, 8>(d, &amount.to_le_bytes());
        d = caba::<A, 9, 1>(d, &[decimals]);

        Self(d)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; TRANSFER_CHECKED_IX_DATA_LEN] {
        &self.0
    }
}
