use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{program_option::COption, program_pack::Pack},
    state::{Account as TokenAccount, AccountState, Mint},
};

use crate::{MINT_RENT_EXEMPT_LAMPORTS, TOKEN_ACC_RENT_EXEMPT_LAMPORTS};

pub fn is_opt_eq_copt<T: PartialEq>(us: Option<T>, sol: COption<T>) -> bool {
    match (us, sol) {
        (None, COption::None) => true,
        (Some(us), COption::Some(sol)) => us == sol,
        _ => false,
    }
}

pub fn token_acc_for_trf(mint: Pubkey, amount: u64, is_native: bool, auth: Pubkey) -> TokenAccount {
    TokenAccount {
        mint,
        owner: auth,
        amount,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: if is_native {
            COption::Some(TOKEN_ACC_RENT_EXEMPT_LAMPORTS)
        } else {
            COption::None
        },
        delegated_amount: 0,
        close_authority: COption::None,
    }
}

pub fn account_from_token_acc(token_acc: TokenAccount) -> Account {
    let mut data = vec![0u8; 165];
    TokenAccount::pack(token_acc, data.as_mut_slice()).unwrap();
    Account {
        data,
        lamports: TOKEN_ACC_RENT_EXEMPT_LAMPORTS
            + if token_acc.is_native() {
                token_acc.amount
            } else {
                0
            },
        owner: spl_token::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub fn account_from_mint(mint: Mint) -> Account {
    let mut data = vec![0u8; 82];
    Mint::pack(mint, data.as_mut_slice()).unwrap();
    Account {
        data,
        lamports: MINT_RENT_EXEMPT_LAMPORTS,
        owner: spl_token::ID,
        executable: false,
        rent_epoch: u64::MAX,
    }
}
