use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::set_auth::{
    SetAuthIxAccs, SET_AUTH_IX_ACCS_LEN, SET_AUTH_IX_IS_SIGNER, SET_AUTH_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type SetAuthIxAccounts<'a> = SetAuthIxAccs<AccountHandle<'a>>;
pub type SetAuthIxAccountPerms = SetAuthIxAccs<AccountPerms>;

pub const SET_AUTH_IX_ACCOUNT_PERMS: SetAuthIxAccountPerms = SetAuthIxAccs(
    signer_writable_to_perms(SET_AUTH_IX_IS_SIGNER.0, SET_AUTH_IX_IS_WRITABLE.0),
);

#[inline]
pub fn set_auth_ix_account_handle_perms(
    a: SetAuthIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, SET_AUTH_IX_ACCS_LEN> {
    a.0.into_iter().zip(SET_AUTH_IX_ACCOUNT_PERMS.0)
}
