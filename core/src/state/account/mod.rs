mod raw;

pub use raw::*;

use crate::typedefs::{
    AccountState, {unpack_valid_coption, COptionDiscm},
};

/// A token account that has been verified to be valid,
/// which means:
/// - all enums are of valid bitpatterns
/// - not in uninitialized state
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenAccount<'a>(&'a RawTokenAccount);

/// Constructors
impl<'a> TokenAccount<'a> {
    /// The only way to safely obtain this struct.
    /// Returns None if `raw` is not a valid token account.
    #[inline]
    pub const fn try_from_raw(raw: &'a RawTokenAccount) -> Option<Self> {
        if COptionDiscm::try_from_arr(&raw.delegate_coption_discm).is_none()
            || AccountState::try_from_u8(raw.state).is_none()
            || COptionDiscm::try_from_arr(&raw.native_rent_exemption_coption_discm).is_none()
            || COptionDiscm::try_from_arr(&raw.close_auth_coption_discm).is_none()
        {
            None
        } else {
            Some(Self(raw))
        }
    }

    /// # Safety
    /// `raw` must be a valid token account, which means:
    /// - all enums are of valid bitpatterns (COption, AccountState)
    /// - not in uninitialized state
    #[inline]
    pub const unsafe fn from_raw_unchecked(raw: &'a RawTokenAccount) -> Self {
        Self(raw)
    }
}

/// Accessors
impl TokenAccount<'_> {
    #[inline]
    pub const fn as_raw(&self) -> &RawTokenAccount {
        self.0
    }

    #[inline]
    pub const fn mint(&self) -> &[u8; 32] {
        &self.0.mint
    }

    /// AKA token account owner, renamed to avoid clash with account.owner
    #[inline]
    pub const fn auth(&self) -> &[u8; 32] {
        &self.0.auth
    }

    #[inline]
    pub const fn amount(&self) -> u64 {
        u64::from_le_bytes(self.0.amount)
    }

    #[inline]
    pub const fn delegate(&self) -> Option<&[u8; 32]> {
        // valid coption checked at construction
        unpack_valid_coption(&self.0.delegate_coption_discm, &self.0.delegate)
    }

    #[inline]
    pub const fn state(&self) -> AccountState {
        match AccountState::try_from_u8(self.0.state) {
            Some(a) => a,
            // valid AccountState checked at construction
            None => unreachable!(),
        }
    }

    /// AKA `is_native`. If this is a wsol account, returns
    /// Some(rent_exempt_lamports). If this is not a wsol account,
    /// return None.
    #[inline]
    pub const fn native_rent_exemption(&self) -> Option<u64> {
        // valid coption checked at construction
        match unpack_valid_coption(
            &self.0.native_rent_exemption_coption_discm,
            &self.0.native_rent_exemption,
        ) {
            None => None,
            Some(bytes) => Some(u64::from_le_bytes(*bytes)),
        }
    }

    #[inline]
    pub const fn delegated_amount(&self) -> u64 {
        u64::from_le_bytes(self.0.delegated_amount)
    }

    #[inline]
    pub const fn close_auth(&self) -> Option<&[u8; 32]> {
        // valid coption checked at construction
        unpack_valid_coption(&self.0.close_auth_coption_discm, &self.0.close_auth)
    }
}
