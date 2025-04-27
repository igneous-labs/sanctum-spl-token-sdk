use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::transfer::{
    TransferIxAccs, TransferIxData, TRANSFER_IX_ACCS_LEN, TRANSFER_IX_IS_SIGNER,
    TRANSFER_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenInstr};

pub type TransferIxAccounts<'a> = TransferIxAccs<AccountHandle<'a>>;
pub type TransferIxAccountPerms = TransferIxAccs<AccountPerms>;

pub const TRANSFER_IX_ACCOUNT_PERMS: TransferIxAccountPerms = TransferIxAccs(
    signer_writable_to_perms(TRANSFER_IX_IS_SIGNER.0, TRANSFER_IX_IS_WRITABLE.0),
);

#[inline]
pub fn transfer_ix<'account, 'data>(
    spl_token_prog: AccountHandle<'account>,
    accounts: TransferIxAccounts<'account>,
    ix_data: &'data TransferIxData,
) -> SplTokenInstr<'account, 'data, TRANSFER_IX_ACCS_LEN> {
    SplTokenInstr {
        prog: spl_token_prog,
        data: ix_data.as_buf(),
        accounts: accounts.0.into_iter().zip(TRANSFER_IX_ACCOUNT_PERMS.0),
    }
}
