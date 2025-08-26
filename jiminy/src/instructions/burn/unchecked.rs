use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::burn::{
    BurnIxAccs, BURN_IX_ACCS_LEN, BURN_IX_IS_SIGNER, BURN_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type BurnIxAccounts<'a> = BurnIxAccs<AccountHandle<'a>>;
pub type BurnIxAccountPerms = BurnIxAccs<AccountPerms>;

pub const BURN_IX_ACCOUNT_PERMS: BurnIxAccountPerms = BurnIxAccs(signer_writable_to_perms(
    BURN_IX_IS_SIGNER.0,
    BURN_IX_IS_WRITABLE.0,
));

#[inline]
pub fn burn_ix_account_handle_perms(
    a: BurnIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, BURN_IX_ACCS_LEN> {
    a.0.into_iter().zip(BURN_IX_ACCOUNT_PERMS.0)
}
