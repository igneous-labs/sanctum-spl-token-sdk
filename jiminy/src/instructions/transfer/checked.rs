use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::transfer::{
    TransferCheckedIxAccs, TRANSFER_CHECKED_IX_ACCS_LEN, TRANSFER_CHECKED_IX_IS_SIGNER,
    TRANSFER_CHECKED_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type TransferCheckedIxAccounts<'a> = TransferCheckedIxAccs<AccountHandle<'a>>;
pub type TransferCheckedIxAccountPerms = TransferCheckedIxAccs<AccountPerms>;

pub const TRANSFER_CHECKED_IX_ACCOUNT_PERMS: TransferCheckedIxAccountPerms =
    TransferCheckedIxAccs(signer_writable_to_perms(
        TRANSFER_CHECKED_IX_IS_SIGNER.0,
        TRANSFER_CHECKED_IX_IS_WRITABLE.0,
    ));

#[inline]
pub fn transfer_checked_ix_account_handle_perms(
    a: TransferCheckedIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, TRANSFER_CHECKED_IX_ACCS_LEN> {
    a.0.into_iter().zip(TRANSFER_CHECKED_IX_ACCOUNT_PERMS.0)
}
