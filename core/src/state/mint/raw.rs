//! The raw account data representation of a token mint

use core::mem::{align_of, size_of};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawMint {
    pub mint_auth_discm: [u8; 4],
    pub mint_auth: [u8; 32],

    // they couldve had a 8-byte aligned struct with no internal padding
    // but they chose to put the 4-byte COption discriminant at the start ._.
    pub supply: [u8; 8],

    pub decimals: u8,
    pub is_init: u8,
    pub freeze_auth_discm: [u8; 4],
    pub freeze_auth: [u8; 32],
}

impl RawMint {
    pub const ACCOUNT_LEN: usize = 82;
}

const _ASSERT_RAW_SIZE: () = assert!(size_of::<RawMint>() == RawMint::ACCOUNT_LEN);
const _ASSERT_RAW_ALIGN: () = assert!(align_of::<RawMint>() == 1);

// pointer casting "serialization"
impl RawMint {
    #[inline]
    pub const fn as_acc_data_arr(&self) -> &[u8; Self::ACCOUNT_LEN] {
        // safety:
        // - has no padding
        // - size checked by _ASSERT_RAW_SIZE
        unsafe { &*core::ptr::from_ref(self).cast() }
    }
}

// pointer casting "deserialization"
impl RawMint {
    /// Returns `None` if `account_data` is not of the right size
    #[inline]
    pub const fn of_acc_data(account_data: &[u8]) -> Option<&Self> {
        match account_data.len() {
            Self::ACCOUNT_LEN => unsafe { Some(Self::of_acc_data_unchecked(account_data)) },
            _ => None,
        }
    }

    /// # Safety
    /// - `account_data` must be of `Self::ACCOUNT_LEN`
    #[inline]
    pub const unsafe fn of_acc_data_unchecked(account_data: &[u8]) -> &Self {
        Self::of_acc_data_arr(&*account_data.as_ptr().cast())
    }

    #[inline]
    pub const fn of_acc_data_arr(account_data_arr: &[u8; Self::ACCOUNT_LEN]) -> &Self {
        // safety:
        // - has no padding
        // - size checked by _ASSERT_RAW_SIZE
        // - align=1 checked by _ASSERT_RAW_ALIGN
        unsafe { &*core::ptr::from_ref(account_data_arr).cast() }
    }
}
