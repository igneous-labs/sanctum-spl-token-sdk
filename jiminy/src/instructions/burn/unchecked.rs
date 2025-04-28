use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::burn::{
    BurnIxAccs, BurnIxData, BURN_IX_ACCS_LEN, BURN_IX_IS_SIGNER, BURN_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenInstr};

pub type BurnIxAccounts<'a> = BurnIxAccs<AccountHandle<'a>>;
pub type BurnIxAccountPerms = BurnIxAccs<AccountPerms>;

pub const BURN_IX_ACCOUNT_PERMS: BurnIxAccountPerms = BurnIxAccs(signer_writable_to_perms(
    BURN_IX_IS_SIGNER.0,
    BURN_IX_IS_WRITABLE.0,
));

#[inline]
pub fn burn_ix<'account, 'data>(
    spl_token_prog: AccountHandle<'account>,
    accounts: BurnIxAccounts<'account>,
    ix_data: &'data BurnIxData,
) -> SplTokenInstr<'account, 'data, BURN_IX_ACCS_LEN> {
    SplTokenInstr {
        prog: spl_token_prog,
        data: ix_data.as_buf(),
        accounts: accounts.0.into_iter().zip(BURN_IX_ACCOUNT_PERMS.0),
    }
}
