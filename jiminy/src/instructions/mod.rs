use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

mod internal_utils;

pub mod burn;
pub mod close_account;
pub mod mint_to;
pub mod transfer;

/// `impl IntoIterator<Item = (AccountHandle, AccountPerms)>`
pub type SplTokenAccountHandlePerms<'account, const ACCOUNTS: usize> = Zip<
    array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
    array::IntoIter<AccountPerms, ACCOUNTS>,
>;
