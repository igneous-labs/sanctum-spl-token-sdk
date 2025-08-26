use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::transfer::{
    TransferIxAccs, TRANSFER_IX_ACCS_LEN, TRANSFER_IX_IS_SIGNER, TRANSFER_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type TransferIxAccounts<'a> = TransferIxAccs<AccountHandle<'a>>;
pub type TransferIxAccountPerms = TransferIxAccs<AccountPerms>;

pub const TRANSFER_IX_ACCOUNT_PERMS: TransferIxAccountPerms = TransferIxAccs(
    signer_writable_to_perms(TRANSFER_IX_IS_SIGNER.0, TRANSFER_IX_IS_WRITABLE.0),
);

#[inline]
pub fn transfer_ix_account_handle_perms(
    a: TransferIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, TRANSFER_IX_ACCS_LEN> {
    a.0.into_iter().zip(TRANSFER_IX_ACCOUNT_PERMS.0)
}
