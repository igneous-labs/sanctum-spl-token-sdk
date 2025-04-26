use proptest::prelude::*;
use sanctum_spl_token_core::state::account::{AccountState, RawTokenAccount, TokenAccount};
use sanctum_spl_token_test_utils::any_token_acc;
use spl_token::{
    solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey},
    state::{Account, AccountState as SolAccountState},
};

proptest! {
    #[test]
    fn check_account_against_sol(acc in any_token_acc()) {
        let mut d = [0u8; RawTokenAccount::ACCOUNT_LEN];
        Account::pack(acc, &mut d).unwrap();
        let raw =  RawTokenAccount::of_acc_data(&d).unwrap();
        let us = TokenAccount::try_from_raw(raw).unwrap();

        prop_assert!(is_us_eq_sol(us, &acc), "{us:#?}, {acc:#?}");
    }
}

proptest! {
    #[test]
    fn check_negative_account_against_sol(
        likely_invalid: [u8; RawTokenAccount::ACCOUNT_LEN]
    ) {
        let raw = RawTokenAccount::of_acc_data_arr(&likely_invalid);
        let us = TokenAccount::try_from_raw(raw);
        // use sol as the ref because Uninitialized accounts
        // are valid for sol but not for us
        match Account::unpack(&likely_invalid) {
            Ok(acc) => {
                prop_assert!(is_us_eq_sol(us.unwrap(), &acc), "{us:#?}, {acc:#?}");
            },
            Err(_) => {
                prop_assert!(us.is_none());
            }
        }
    }
}

fn is_us_eq_sol(us: TokenAccount, sol: &Account) -> bool {
    us.amount() == sol.amount
        && us.authority() == sol.owner.as_array()
        && is_opt_eq(
            us.close_authority().map(|p| *p),
            sol.close_authority.map(Pubkey::to_bytes),
        )
        && is_opt_eq(
            us.delegate().map(|p| *p),
            sol.delegate.map(Pubkey::to_bytes),
        )
        && us.delegated_amount() == sol.delegated_amount
        && us.mint() == sol.mint.as_array()
        && is_opt_eq(us.native_rent_exemption(), sol.is_native)
        && is_account_state_eq(us.state(), sol.state)
}

fn is_opt_eq<T: PartialEq>(us: Option<T>, sol: COption<T>) -> bool {
    match (us, sol) {
        (None, COption::None) => true,
        (Some(us), COption::Some(sol)) => us == sol,
        _ => false,
    }
}

fn is_account_state_eq(us: AccountState, sol: SolAccountState) -> bool {
    match (us, sol) {
        (AccountState::Frozen, SolAccountState::Frozen)
        | (AccountState::Initialized, SolAccountState::Initialized) => true,
        _ => false,
    }
}
