use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{instruction::AccountMeta, program_option::COption, program_pack::Pack},
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

// TokenAccount

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

// Mint

pub fn init_mint_acc(
    mint_authority: Option<Pubkey>,
    supply: u64,
    decimals: u8,
    freeze_authority: Option<Pubkey>,
) -> Mint {
    Mint {
        mint_authority: mint_authority.map_or_else(|| COption::None, COption::Some),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: freeze_authority.map_or_else(|| COption::None, COption::Some),
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

// instructions

pub const fn key_signer_writable_to_metas<const N: usize>(
    keys: &[Pubkey; N],
    is_signer: &[bool; N],
    is_writable: &[bool; N],
) -> [AccountMeta; N] {
    const UNINIT: AccountMeta = AccountMeta {
        pubkey: Pubkey::new_from_array([0; 32]),
        is_signer: false,
        is_writable: false,
    };
    let mut res = [UNINIT; N];
    let mut i = 0;
    while i < N {
        res[i] = AccountMeta {
            pubkey: keys[i],
            is_signer: is_signer[i],
            is_writable: is_writable[i],
        };
        i += 1;
    }
    res
}
