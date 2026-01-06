use generic_array_struct::generic_array_struct;

use crate::instructions::internal_utils::{caba, discm_checked, impl_memset};

// Accounts

#[generic_array_struct(builder destr trymap pub)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SetAuthIxAccs<T> {
    /// Token or mint account to set authority of
    pub set: T,

    /// Current authority
    pub auth: T,
}

impl_memset!(SetAuthIxAccs);

pub type SetAuthIxAccsFlag = SetAuthIxAccs<bool>;

pub const SET_AUTH_IX_IS_SIGNER: SetAuthIxAccsFlag =
    SetAuthIxAccsFlag::memset(false).const_with_auth(true);

pub const SET_AUTH_IX_IS_WRITABLE: SetAuthIxAccsFlag =
    SetAuthIxAccsFlag::memset(false).const_with_set(true);

// Data

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
}

impl AuthType {
    #[inline]
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::MintTokens => 0,
            Self::FreezeAccount => 1,
            Self::AccountOwner => 2,
            Self::CloseAccount => 3,
        }
    }

    #[inline]
    pub const fn try_from_u8(v: u8) -> Option<Self> {
        Some(match v {
            0 => Self::MintTokens,
            1 => Self::FreezeAccount,
            2 => Self::AccountOwner,
            3 => Self::CloseAccount,
            _ => return None,
        })
    }
}

pub const SET_AUTH_IX_DISCM: u8 = 6;

pub const SET_AUTH_IX_DISCM_LEN: usize = 35;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetAuthIxData([u8; SET_AUTH_IX_DISCM_LEN]);

impl SetAuthIxData {
    pub const LEN: usize = SET_AUTH_IX_DISCM_LEN;

    #[inline]
    pub const fn new(ty: AuthType, new_auth: Option<&[u8; 32]>) -> Self {
        const A: usize = SET_AUTH_IX_DISCM_LEN;

        let mut res = [0; A];

        res = caba::<A, 0, 1>(res, &[SET_AUTH_IX_DISCM]);
        res = caba::<A, 1, 1>(res, &[ty.into_u8()]);
        res = match new_auth {
            Some(a) => {
                res = caba::<A, 2, 1>(res, &[1]);
                caba::<A, 3, 32>(res, a)
            }
            None => caba::<A, 2, 1>(res, &[0]),
        };

        Self(res)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8] {
        match self.0[2] {
            // safety: in bounds
            0 => unsafe { self.0.first_chunk::<3>().unwrap_unchecked() },
            1 => &self.0,
            // safety: discm guaranteed to be correct at construction
            _ => unreachable!(),
        }
    }

    /// Fallible
    #[inline]
    pub const fn parse_no_discm(data: &[u8; 34]) -> Option<(AuthType, Option<&[u8; 32]>)> {
        let auth_type = match AuthType::try_from_u8(data[0]) {
            Some(x) => x,
            None => return None,
        };
        let new_auth = match data[1] {
            0 => None,
            // safety: in bounds
            1 => Some(unsafe { data.last_chunk().unwrap_unchecked() }),
            _ => return None,
        };
        Some((auth_type, new_auth))
    }

    /// Returns `None` if discm does not match,
    /// in addition to possible failures in [`Self::parse_no_discm`]
    #[inline]
    pub const fn parse(
        data: &[u8; SET_AUTH_IX_DISCM_LEN],
    ) -> Option<(AuthType, Option<&[u8; 32]>)> {
        match discm_checked(SET_AUTH_IX_DISCM, data) {
            None => None,
            Some(d) => Self::parse_no_discm(d),
        }
    }
}
