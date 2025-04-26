mod raw;

pub use raw::*;

use crate::typedefs::{
    coption::{unpack_valid_coption, COptionDiscm},
    u8_bool::try_bool_from_u8,
};

/// A mint account that has been verified to be valid,
/// which means:
/// - all types are of valid bitpatterns
/// - initialized
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mint<'a>(&'a RawMint);

/// Constructors
impl<'a> Mint<'a> {
    /// The only way to safely obtain this struct.
    /// Returns None if `raw` is not a valid token account.
    #[inline]
    pub const fn try_from_raw(raw: &'a RawMint) -> Option<Self> {
        let is_init = match try_bool_from_u8(raw.is_init) {
            Some(is_init) => is_init,
            None => false,
        };
        if COptionDiscm::try_from_arr(&raw.mint_auth_discm).is_none()
            || COptionDiscm::try_from_arr(&raw.freeze_auth_discm).is_none()
            || !is_init
        {
            None
        } else {
            Some(Self(raw))
        }
    }

    /// # Safety
    /// `raw` must be a valid mint account, which means:
    /// - all types are of valid bitpatterns (COption, bool)
    /// - not in uninitialized state
    #[inline]
    pub const unsafe fn from_raw_unchecked(raw: &'a RawMint) -> Self {
        Self(raw)
    }
}

/// Accessors
impl Mint<'_> {
    #[inline]
    pub const fn as_raw(&self) -> &RawMint {
        self.0
    }

    #[inline]
    pub const fn mint_auth(&self) -> Option<&[u8; 32]> {
        // valid coption checked at construction
        unpack_valid_coption(&self.0.mint_auth_discm, &self.0.mint_auth)
    }

    #[inline]
    pub const fn supply(&self) -> u64 {
        u64::from_le_bytes(self.0.supply)
    }

    #[inline]
    pub const fn decimals(&self) -> u8 {
        self.0.decimals
    }

    #[inline]
    pub const fn freeze_auth(&self) -> Option<&[u8; 32]> {
        // valid coption checked at construction
        unpack_valid_coption(&self.0.freeze_auth_discm, &self.0.freeze_auth)
    }
}
