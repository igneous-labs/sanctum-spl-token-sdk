//! The raw account data representation of a token account

use core::mem::{align_of, size_of};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawTokenAccount {
    pub mint: [u8; 32],
    pub authority: [u8; 32],
    pub amount: [u8; 8],
    pub delegate_coption_discm: [u8; 4],
    pub delegate: [u8; 32],

    // this single u8 here fuks alignment up so now
    // we have the worst of both worlds:
    // wasted space + unaligned memory accesses _|_
    //
    // I bet this was because the fields following this were
    // initially unplanned and tacked on afterwards
    pub state: u8,

    pub native_rent_exemption_coption_discm: [u8; 4],
    pub native_rent_exemption: [u8; 8],
    pub delegated_amount: [u8; 8],
    pub close_authority_coption_discm: [u8; 4],
    pub close_authority: [u8; 32],
}

impl RawTokenAccount {
    pub const ACCOUNT_LEN: usize = 165;
}

const _ASSERT_RAW_TOKEN_ACC_SIZE: () =
    assert!(size_of::<RawTokenAccount>() == RawTokenAccount::ACCOUNT_LEN);
const _ASSERT_RAW_TOKEN_ACC_ALIGN: () = assert!(align_of::<RawTokenAccount>() == 1);

// pointer casting "serialization"
impl RawTokenAccount {
    #[inline]
    pub const fn as_acc_data_arr(&self) -> &[u8; Self::ACCOUNT_LEN] {
        // safety: RawTokenAccount has no padding,
        // size checked by _ASSERT_RAW_TOKEN_ACC_SIZE
        unsafe { &*core::ptr::from_ref(self).cast() }
    }
}

// pointer casting "deserialization"
impl RawTokenAccount {
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
        // safety: RawTokenAccount has no padding,
        // size checked by _ASSERT_RAW_TOKEN_ACC_SIZE
        unsafe { &*core::ptr::from_ref(account_data_arr).cast() }
    }
}
