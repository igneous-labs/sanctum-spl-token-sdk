use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{caba, impl_memset};

// Accounts

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BurnIxAccs<T> {
    pub from: T,
    pub mint: T,
    pub auth: T,
}

impl<T: Copy> BurnIxAccs<T> {
    impl_memset!(BURN_IX_ACCS_LEN);
}

pub type BurnIxAccsFlag = BurnIxAccs<bool>;

pub const BURN_IX_IS_SIGNER: BurnIxAccsFlag = BurnIxAccsFlag::memset(false).const_with_auth(true);

pub const BURN_IX_IS_WRITABLE: BurnIxAccsFlag = BurnIxAccsFlag::memset(false)
    .const_with_from(true)
    .const_with_mint(true);

// Data

pub const BURN_IX_DISCM: u8 = 8;

pub const BURN_IX_DATA_LEN: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BurnIxData([u8; BURN_IX_DATA_LEN]);

impl BurnIxData {
    #[inline]
    pub const fn new(amount: u64) -> Self {
        const A: usize = BURN_IX_DATA_LEN;

        let mut d = [0u8; A];

        d = caba::<A, 0, 1>(d, &[BURN_IX_DISCM]);
        d = caba::<A, 1, 8>(d, &amount.to_le_bytes());

        Self(d)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; BURN_IX_DATA_LEN] {
        &self.0
    }
}
