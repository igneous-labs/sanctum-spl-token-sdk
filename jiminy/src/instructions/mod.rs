use core::{array, iter::Zip};

use jiminy_cpi::{account::AccountHandle, AccountPerms};

mod internal_utils;

pub mod burn;
pub mod mint_to;
pub mod transfer;

pub type SplTokenInstr<'account, 'data, const ACCOUNTS: usize> = jiminy_cpi::Instr<
    'account,
    'data,
    Zip<
        array::IntoIter<AccountHandle<'account>, ACCOUNTS>,
        array::IntoIter<AccountPerms, ACCOUNTS>,
    >,
>;
