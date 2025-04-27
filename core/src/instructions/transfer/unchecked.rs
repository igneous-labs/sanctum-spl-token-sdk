use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{caba, impl_memset};

// Accounts

#[generic_array_struct(pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferIxAccs<T> {
    pub src: T,
    pub dst: T,
    pub src_auth: T,
}

impl<T: Copy> TransferIxAccs<T> {
    impl_memset!(TRANSFER_IX_ACCS_LEN);
}

pub type TransferIxAccsFlag = TransferIxAccs<bool>;

pub const TRANSFER_IX_IS_SIGNER: TransferIxAccsFlag =
    TransferIxAccsFlag::memset(false).const_with_src_auth(true);

pub const TRANSFER_IX_IS_WRITABLE: TransferIxAccsFlag = TransferIxAccsFlag::memset(false)
    .const_with_src(true)
    .const_with_dst(true);

// Data

pub const TRANSFER_IX_DISCM: u8 = 3;

pub const TRANSFER_IX_DATA_LEN: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferIxData([u8; TRANSFER_IX_DATA_LEN]);

impl TransferIxData {
    #[inline]
    pub const fn new(amount: u64) -> Self {
        const A: usize = TRANSFER_IX_DATA_LEN;

        let mut d = [0u8; A];

        d = caba::<A, 0, 1>(d, &[TRANSFER_IX_DISCM]);
        d = caba::<A, 1, 8>(d, &amount.to_le_bytes());

        Self(d)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; TRANSFER_IX_DATA_LEN] {
        &self.0
    }
}
