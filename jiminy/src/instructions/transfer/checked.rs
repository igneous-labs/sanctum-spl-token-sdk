use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::transfer::{
    TransferCheckedIxAccs, TransferCheckedIxData, TRANSFER_CHECKED_IX_ACCS_LEN,
    TRANSFER_CHECKED_IX_IS_SIGNER, TRANSFER_CHECKED_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenInstr};

pub type TransferCheckedIxAccounts<'a> = TransferCheckedIxAccs<AccountHandle<'a>>;
pub type TransferCheckedIxAccountPerms = TransferCheckedIxAccs<AccountPerms>;

pub const TRANSFER_CHECKED_IX_ACCOUNT_PERMS: TransferCheckedIxAccountPerms =
    TransferCheckedIxAccs(signer_writable_to_perms(
        TRANSFER_CHECKED_IX_IS_SIGNER.0,
        TRANSFER_CHECKED_IX_IS_WRITABLE.0,
    ));

#[inline]
pub fn transfer_checked_ix<'account, 'data>(
    spl_token_prog: AccountHandle<'account>,
    accounts: TransferCheckedIxAccounts<'account>,
    ix_data: &'data TransferCheckedIxData,
) -> SplTokenInstr<'account, 'data, TRANSFER_CHECKED_IX_ACCS_LEN> {
    SplTokenInstr {
        prog: spl_token_prog,
        data: ix_data.as_buf(),
        accounts: accounts
            .0
            .into_iter()
            .zip(TRANSFER_CHECKED_IX_ACCOUNT_PERMS.0),
    }
}
