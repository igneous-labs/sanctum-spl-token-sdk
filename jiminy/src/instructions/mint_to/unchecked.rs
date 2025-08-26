use jiminy_cpi::{account::AccountHandle, AccountPerms};
use sanctum_spl_token_core::instructions::mint_to::{
    MintToIxAccs, MINT_TO_IX_ACCS_LEN, MINT_TO_IX_IS_SIGNER, MINT_TO_IX_IS_WRITABLE,
};

use crate::instructions::{internal_utils::signer_writable_to_perms, SplTokenAccountHandlePerms};

pub type MintToIxAccounts<'a> = MintToIxAccs<AccountHandle<'a>>;
pub type MintToIxAccountPerms = MintToIxAccs<AccountPerms>;

pub const MINT_TO_IX_ACCOUNT_PERMS: MintToIxAccountPerms = MintToIxAccs(signer_writable_to_perms(
    MINT_TO_IX_IS_SIGNER.0,
    MINT_TO_IX_IS_WRITABLE.0,
));

#[inline]
pub fn mint_to_ix_account_handle_perms(
    a: MintToIxAccounts<'_>,
) -> SplTokenAccountHandlePerms<'_, MINT_TO_IX_ACCS_LEN> {
    a.0.into_iter().zip(MINT_TO_IX_ACCOUNT_PERMS.0)
}
