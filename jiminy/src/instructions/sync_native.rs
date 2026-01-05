use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::sync_native::{
    SyncNativeIxAccs, SYNC_NATIVE_IX_ACCS_LEN, SYNC_NATIVE_IX_IS_SIGNER, SYNC_NATIVE_IX_IS_WRITABLE,
};

use super::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type SyncNativeIxAccounts<'a> = SyncNativeIxAccs<AccountHandle<'a>>;
pub type SyncNativeIxAccountPerms = SyncNativeIxAccs<AccountPerms>;

pub const SYNC_NATIVE_IX_ACCOUNT_PERMS: SyncNativeIxAccountPerms = SyncNativeIxAccs(
    signer_writable_to_perms(SYNC_NATIVE_IX_IS_SIGNER.0, SYNC_NATIVE_IX_IS_WRITABLE.0),
);

#[inline]
pub fn sync_native_ix_account_handle_perms(
    a: SyncNativeIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, SYNC_NATIVE_IX_ACCS_LEN> {
    a.0.into_iter().zip(SYNC_NATIVE_IX_ACCOUNT_PERMS.0)
}
