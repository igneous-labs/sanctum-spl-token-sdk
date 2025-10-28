use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::close_account::{
    CloseAccountIxAccs, CLOSE_ACCOUNT_IX_ACCS_LEN, CLOSE_ACCOUNT_IX_IS_SIGNER,
    CLOSE_ACCOUNT_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type CloseAccountIxAccounts<'a> = CloseAccountIxAccs<AccountHandle<'a>>;
pub type CloseAccountIxAccountPerms = CloseAccountIxAccs<AccountPerms>;

pub const CLOSE_ACCOUNT_IX_ACCOUNT_PERMS: CloseAccountIxAccountPerms = CloseAccountIxAccs(
    signer_writable_to_perms(CLOSE_ACCOUNT_IX_IS_SIGNER.0, CLOSE_ACCOUNT_IX_IS_WRITABLE.0),
);

#[inline]
pub fn close_account_ix_account_handle_perms(
    a: CloseAccountIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, CLOSE_ACCOUNT_IX_ACCS_LEN> {
    a.0.into_iter().zip(CLOSE_ACCOUNT_IX_ACCOUNT_PERMS.0)
}
