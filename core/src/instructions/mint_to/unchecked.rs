use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{caba, impl_memset};

// Accounts

#[generic_array_struct(builder pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MintToIxAccs<T> {
    pub mint: T,
    pub to: T,
    pub auth: T,
}

impl<T: Copy> MintToIxAccs<T> {
    impl_memset!(MINT_TO_IX_ACCS_LEN);
}

pub type MintToIxAccsFlag = MintToIxAccs<bool>;

pub const MINT_TO_IX_IS_SIGNER: MintToIxAccsFlag =
    MintToIxAccsFlag::memset(false).const_with_auth(true);

pub const MINT_TO_IX_IS_WRITABLE: MintToIxAccsFlag = MintToIxAccsFlag::memset(false)
    .const_with_mint(true)
    .const_with_to(true);

// Data

pub const MINT_TO_IX_DISCM: u8 = 7;

pub const MINT_TO_IX_DATA_LEN: usize = 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MintToIxData([u8; MINT_TO_IX_DATA_LEN]);

impl MintToIxData {
    #[inline]
    pub const fn new(amount: u64) -> Self {
        const A: usize = MINT_TO_IX_DATA_LEN;

        let mut d = [0u8; A];

        d = caba::<A, 0, 1>(d, &[MINT_TO_IX_DISCM]);
        d = caba::<A, 1, 8>(d, &amount.to_le_bytes());

        Self(d)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; MINT_TO_IX_DATA_LEN] {
        &self.0
    }
}
