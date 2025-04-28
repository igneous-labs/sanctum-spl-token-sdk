//! .so file size 3216

#![cfg(feature = "test-sbf")]

use mollusk_svm::{result::InstructionResult, Mollusk};
use proptest::prelude::*;
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::burn::{
    BurnIxAccs, BURN_IX_IS_SIGNER, BURN_IX_IS_WRITABLE,
};
use sanctum_spl_token_test_utils::{
    account_from_mint, account_from_token_acc, are_all_accounts_rent_exempt, init_mint_acc,
    is_tx_balanced, key_signer_writable_to_metas, silence_mollusk_prog_logs, token_acc_for_trf,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
    },
    state::{Account as TokenAccount, Mint},
};

const PROG_NAME: &str = "burn_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("DWT1tG7AMF5NNGnKV1aeixYSGdmRqgUJiB7jqFtLXCEh");

const MINT: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const FROM: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const SUPPLY: u64 = 29_125_461_325;
const DECIMALS: u8 = 9;
const AMT: u64 = 1_234;
const INIT_AMT: u64 = AMT * 2;

const FROM_ACC_IDX: usize = 1;
const MINT_ACC_IDX: usize = 2;

// CUs: 5890
#[test]
fn burn_all_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        FROM,
        token_acc_for_trf(MINT, INIT_AMT, false, AUTH),
        MINT,
        init_mint_acc(None, SUPPLY, DECIMALS, None),
        AUTH,
    );
    let instr = ix(FROM, MINT, AUTH, None);

    let InstructionResult {
        compute_units_consumed,
        raw_result,
        resulting_accounts,
        ..
    } = svm.process_instruction(&instr, &accounts);

    raw_result.unwrap();

    eprintln!("{compute_units_consumed} CUs");

    are_all_accounts_rent_exempt(&resulting_accounts).unwrap();
    assert!(is_tx_balanced(&accounts, &resulting_accounts));

    let mint_acc = &resulting_accounts[MINT_ACC_IDX].1;
    assert_eq!(
        SUPPLY - INIT_AMT,
        Mint::unpack(&mint_acc.data).unwrap().supply
    );
    let from_acc = &resulting_accounts[FROM_ACC_IDX].1;
    assert_eq!(0, TokenAccount::unpack(&from_acc.data).unwrap().amount);
}

// CUs: 5852
#[test]
fn burn_arg_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        FROM,
        token_acc_for_trf(MINT, INIT_AMT, false, AUTH),
        MINT,
        init_mint_acc(None, SUPPLY, DECIMALS, None),
        AUTH,
    );
    let instr = ix(FROM, MINT, AUTH, Some(AMT));

    let InstructionResult {
        compute_units_consumed,
        raw_result,
        resulting_accounts,
        ..
    } = svm.process_instruction(&instr, &accounts);

    raw_result.unwrap();

    eprintln!("{compute_units_consumed} CUs");

    are_all_accounts_rent_exempt(&resulting_accounts).unwrap();
    assert!(is_tx_balanced(&accounts, &resulting_accounts));

    let mint_acc = &resulting_accounts[MINT_ACC_IDX].1;
    assert_eq!(SUPPLY - AMT, Mint::unpack(&mint_acc.data).unwrap().supply);
    let from_acc = &resulting_accounts[FROM_ACC_IDX].1;
    assert_eq!(
        INIT_AMT - AMT,
        TokenAccount::unpack(&from_acc.data).unwrap().amount
    );
}

proptest! {
    #[test]
    fn burn_all_cases(
        (mint, from) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint))),
        auth: [u8; 32],
        decimals: u8,
        (supply, init_amt, burn_amt) in
            any::<u64>()
                .prop_flat_map(|supply| (Just(supply), 0..=supply))
                .prop_flat_map(|(supply, init_amt)| (Just(supply), Just(init_amt), 0..=init_amt))
    ) {
        let [mint, from, auth] = [mint, from, auth]
            .map(Pubkey::new_from_array);
        let svm = mollusk();
        silence_mollusk_prog_logs();

        for arg in [None, Some(burn_amt)] {
            let accounts = ix_accounts(
                from,
                token_acc_for_trf(mint, init_amt, false, auth),
                mint,
                init_mint_acc(None, supply, decimals, None),
                auth,
            );
            let instr = ix(from, mint, auth, arg);

            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_instruction(&instr, &accounts);

            raw_result.unwrap();

            are_all_accounts_rent_exempt(&resulting_accounts).unwrap();
            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            let expected_burn_amt = match arg {
                None => init_amt,
                Some(a) => a,
            };

            let mint_acc = &resulting_accounts[MINT_ACC_IDX].1;
            assert_eq!(supply - expected_burn_amt, Mint::unpack(&mint_acc.data).unwrap().supply);
            let to_acc = &resulting_accounts[FROM_ACC_IDX].1;
            assert_eq!(init_amt - expected_burn_amt, TokenAccount::unpack(&to_acc.data).unwrap().amount);
        }
    }
}

fn mollusk() -> Mollusk {
    let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
    mollusk_svm_programs_token::token::add_program(&mut svm);
    svm
}

fn ix_accounts(
    from: Pubkey,
    from_acc: TokenAccount,
    mint: Pubkey,
    mint_acc: Mint,
    auth: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        mollusk_svm_programs_token::token::keyed_account(),
        (from, account_from_token_acc(from_acc)),
        (mint, account_from_mint(mint_acc)),
        (auth, Account::default()),
    ]
}

fn ix(from: Pubkey, mint: Pubkey, auth: Pubkey, amt: Option<u64>) -> Instruction {
    type BurnIxKeys = BurnIxAccs<Pubkey>;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &BurnIxKeys::memset(PROG_ID)
                .with_from(from)
                .with_mint(mint)
                .with_auth(auth)
                .0,
            &BURN_IX_IS_SIGNER.0,
            &BURN_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: amt.map_or_else(Vec::new, |amt| Vec::from(amt.to_le_bytes())),
    }
}
