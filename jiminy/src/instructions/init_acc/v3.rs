use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::init_acc::{
    InitAcc3IxAccs, INIT_ACC3_IX_ACCS_LEN, INIT_ACC3_IX_IS_SIGNER, INIT_ACC3_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type InitAcc3IxAccounts<'a> = InitAcc3IxAccs<AccountHandle<'a>>;
pub type InitAcc3IxAccountPerms = InitAcc3IxAccs<AccountPerms>;

pub const INIT_ACC3_IX_ACCOUNT_PERMS: InitAcc3IxAccountPerms = InitAcc3IxAccs(
    signer_writable_to_perms(INIT_ACC3_IX_IS_SIGNER.0, INIT_ACC3_IX_IS_WRITABLE.0),
);

#[inline]
pub fn init_acc3_ix_account_handle_perms(
    a: InitAcc3IxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, INIT_ACC3_IX_ACCS_LEN> {
    a.0.into_iter().zip(INIT_ACC3_IX_ACCOUNT_PERMS.0)
}
