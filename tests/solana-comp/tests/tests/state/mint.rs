use proptest::prelude::*;
use sanctum_spl_token_core::state::mint::{Mint, RawMint};
use sanctum_spl_token_test_utils::{any_init_mint, is_opt_eq_copt};
use spl_token::{solana_program::program_pack::Pack, state::Mint as SolMint};

proptest! {
    #[test]
    fn check_account_against_sol(acc in any_init_mint()) {
        let mut d = [0u8; RawMint::ACCOUNT_LEN];
        SolMint::pack(acc, &mut d).unwrap();
        let raw =  RawMint::of_acc_data(&d).unwrap();
        let us = Mint::try_from_raw(raw).unwrap();

        prop_assert!(is_us_eq_sol(us, &acc), "{us:#?}, {acc:#?}");
    }
}

proptest! {
    #[test]
    fn check_negative_account_against_sol(
        likely_invalid: [u8; RawMint::ACCOUNT_LEN]
    ) {
        let raw = RawMint::of_acc_data_arr(&likely_invalid);
        let us = Mint::try_from_raw(raw);
        match SolMint::unpack(&likely_invalid) {
            Ok(acc) if acc.is_initialized => {
                prop_assert!(is_us_eq_sol(us.unwrap(), &acc), "{us:#?}, {acc:#?}");
            },
            _ => {
                prop_assert!(us.is_none());
            }
        }
    }
}

fn is_us_eq_sol(us: Mint, sol: &SolMint) -> bool {
    us.decimals() == sol.decimals
        && is_opt_eq_copt(
            us.freeze_auth(),
            sol.freeze_authority.as_ref().map(|k| k.as_array()),
        )
        && is_opt_eq_copt(
            us.mint_auth(),
            sol.mint_authority.as_ref().map(|k| k.as_array()),
        )
        && us.supply() == sol.supply
        && sol.is_initialized
}
