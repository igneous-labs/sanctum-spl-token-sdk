use proptest::prelude::*;
use sanctum_spl_token_core::{
    state::account::{RawTokenAccount, TokenAccount},
    typedefs::AccountState,
};
use sanctum_spl_token_test_utils::{any_init_token_acc, is_opt_eq_copt};
use spl_token::{
    solana_program::program_pack::Pack,
    state::{Account as SolAccount, AccountState as SolAccountState},
};

proptest! {
    #[test]
    fn check_account_against_sol(acc in any_init_token_acc()) {
        let mut d = [0u8; RawTokenAccount::ACCOUNT_LEN];
        SolAccount::pack(acc, &mut d).unwrap();
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
        match SolAccount::unpack(&likely_invalid) {
            Ok(acc) if !matches!(acc.state, SolAccountState::Uninitialized) => {
                prop_assert!(is_us_eq_sol(us.unwrap(), &acc), "{us:#?}, {acc:#?}");
            },
            _ => {
                prop_assert!(us.is_none());
            }
        }
    }
}

fn is_us_eq_sol(us: TokenAccount, sol: &SolAccount) -> bool {
    us.amount() == sol.amount
        && us.auth() == sol.owner.as_array()
        && is_opt_eq_copt(
            us.close_auth(),
            sol.close_authority.as_ref().map(|k| k.as_array()),
        )
        && is_opt_eq_copt(us.delegate(), sol.delegate.as_ref().map(|k| k.as_array()))
        && us.delegated_amount() == sol.delegated_amount
        && us.mint() == sol.mint.as_array()
        && is_opt_eq_copt(us.native_rent_exemption(), sol.is_native)
        && is_account_state_eq(us.state(), sol.state)
}

fn is_account_state_eq(us: AccountState, sol: SolAccountState) -> bool {
    matches!(
        (us, sol),
        (AccountState::Frozen, SolAccountState::Frozen)
            | (AccountState::Initialized, SolAccountState::Initialized)
    )
}
