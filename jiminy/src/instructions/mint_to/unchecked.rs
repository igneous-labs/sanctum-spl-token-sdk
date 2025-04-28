use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::mint_to::{
    MintToIxAccs, MintToIxData, MINT_TO_IX_ACCS_LEN, MINT_TO_IX_IS_SIGNER, MINT_TO_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenInstr};

pub type MintToIxAccounts<'a> = MintToIxAccs<AccountHandle<'a>>;
pub type MintToIxAccountPerms = MintToIxAccs<AccountPerms>;

pub const MINT_TO_IX_ACCOUNT_PERMS: MintToIxAccountPerms = MintToIxAccs(signer_writable_to_perms(
    MINT_TO_IX_IS_SIGNER.0,
    MINT_TO_IX_IS_WRITABLE.0,
));

#[inline]
pub fn mint_to_ix<'account, 'data>(
    spl_token_prog: AccountHandle<'account>,
    accounts: MintToIxAccounts<'account>,
    ix_data: &'data MintToIxData,
) -> SplTokenInstr<'account, 'data, MINT_TO_IX_ACCS_LEN> {
    SplTokenInstr {
        prog: spl_token_prog,
        data: ix_data.as_buf(),
        accounts: accounts.0.into_iter().zip(MINT_TO_IX_ACCOUNT_PERMS.0),
    }
}
