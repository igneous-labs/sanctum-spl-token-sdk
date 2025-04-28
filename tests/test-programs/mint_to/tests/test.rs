//! .so file size 3072

#![cfg(feature = "test-sbf")]

use mollusk_svm::{result::InstructionResult, Mollusk};
use proptest::prelude::*;
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::mint_to::{
    MintToIxAccs, MINT_TO_IX_IS_SIGNER, MINT_TO_IX_IS_WRITABLE,
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

const PROG_NAME: &str = "mint_to_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("HTwaWpPCBUwi8kx3nMSY3MKviwwf7mjZncYAgagwXYur");

const MINT: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const TO: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const SUPPLY: u64 = 1234;
const DECIMALS: u8 = 9;
const AMT: u64 = 29_125_461_325;

const MINT_ACC_IDX: usize = 1;
const TO_ACC_IDX: usize = 2;

// CUs: 5710
#[test]
fn mint_to_all_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        MINT,
        init_mint_acc(Some(AUTH), SUPPLY, DECIMALS, None),
        TO,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(MINT, TO, AUTH, None);

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
    assert_eq!(u64::MAX, Mint::unpack(&mint_acc.data).unwrap().supply);
    let to_acc = &resulting_accounts[TO_ACC_IDX].1;
    assert_eq!(
        u64::MAX - SUPPLY,
        TokenAccount::unpack(&to_acc.data).unwrap().amount
    );
}

// CUs: 5683
#[test]
fn mint_to_arg_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        MINT,
        init_mint_acc(Some(AUTH), SUPPLY, DECIMALS, None),
        TO,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(MINT, TO, AUTH, Some(AMT));

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
    assert_eq!(SUPPLY + AMT, Mint::unpack(&mint_acc.data).unwrap().supply);
    let to_acc = &resulting_accounts[TO_ACC_IDX].1;
    assert_eq!(AMT, TokenAccount::unpack(&to_acc.data).unwrap().amount);
}

proptest! {
    #[test]
    fn mint_to_all_cases(
        (mint, to) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint))),
        auth: [u8; 32],
        decimals: u8,
        (supply, init_amt, mint_amt) in
            any::<u64>()
                .prop_flat_map(|supply| (Just(supply), 0..=supply, 0..=u64::MAX - supply))
    ) {
        let [mint, to, auth] = [mint, to, auth]
            .map(Pubkey::new_from_array);
        let svm = mollusk();
        silence_mollusk_prog_logs();

        for arg in [None, Some(mint_amt)] {
            let accounts = ix_accounts(
                mint,
                init_mint_acc(Some(auth), supply, decimals, None),
                to,
                token_acc_for_trf(mint, init_amt, false, Default::default()),
                auth,
            );
            let instr = ix(mint, to, auth, arg);

            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_instruction(&instr, &accounts);

            raw_result.unwrap();

            are_all_accounts_rent_exempt(&resulting_accounts).unwrap();
            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            let expected_mint_amt = match arg {
                None => u64::MAX - supply,
                Some(a) => a,
            };

            let mint_acc = &resulting_accounts[MINT_ACC_IDX].1;
            assert_eq!(supply + expected_mint_amt, Mint::unpack(&mint_acc.data).unwrap().supply);
            let to_acc = &resulting_accounts[TO_ACC_IDX].1;
            assert_eq!(init_amt + expected_mint_amt, TokenAccount::unpack(&to_acc.data).unwrap().amount);
        }
    }
}

fn mollusk() -> Mollusk {
    let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
    mollusk_svm_programs_token::token::add_program(&mut svm);
    svm
}

fn ix_accounts(
    mint: Pubkey,
    mint_acc: Mint,
    to: Pubkey,
    to_acc: TokenAccount,
    auth: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        mollusk_svm_programs_token::token::keyed_account(),
        (mint, account_from_mint(mint_acc)),
        (to, account_from_token_acc(to_acc)),
        (auth, Account::default()),
    ]
}

fn ix(mint: Pubkey, to: Pubkey, auth: Pubkey, amt: Option<u64>) -> Instruction {
    type MintToIxKeys = MintToIxAccs<Pubkey>;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &MintToIxKeys::memset(PROG_ID)
                .with_mint(mint)
                .with_to(to)
                .with_auth(auth)
                .0,
            &MINT_TO_IX_IS_SIGNER.0,
            &MINT_TO_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: amt.map_or_else(Vec::new, |amt| Vec::from(amt.to_le_bytes())),
    }
}
