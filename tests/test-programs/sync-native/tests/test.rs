#![cfg(feature = "test-sbf")]

use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::sync_native::{
    SyncNativeIxAccs, SyncNativeIxAccsDestr, SYNC_NATIVE_IX_IS_SIGNER, SYNC_NATIVE_IX_IS_WRITABLE,
};
use sanctum_spl_token_test_utils::{
    account_from_token_acc, bench_binsize, expect_test::expect, is_tx_balanced,
    key_signer_writable_to_metas, token_acc_for_trf,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
    },
    state::Account as TokenAccount,
};

const PROG_NAME: &str = "sync_native_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("5yNCe2J1zuuf34zuZfNNdjJz4MajzcpgUnLdBKdcs5wg");

const ACC: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const WSOL_MINT: Pubkey = solana_pubkey::pubkey!("So11111111111111111111111111111111111111112");

const ACC_ACC_IDX: usize = 0;

thread_local! {
    static SVM: Mollusk = {
        let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        svm
    };
}

#[test]
fn save_binsize() {
    bench_binsize(PROG_NAME, expect!["1944"]);
}

#[test]
fn sync_native_cus() {
    let amt: u64 = 5_000_000_000;
    let mut accounts = ix_accounts(ACC, token_acc_for_trf(WSOL_MINT, amt, true, AUTH));
    // make lamports out of sync
    accounts[0].1.lamports += 1_000_000;

    let instr = ix(ACC);

    let cus = SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        // Account should have been synced
        let sync_acc = &resulting_accounts[ACC_ACC_IDX].1;
        let sync_ta = TokenAccount::unpack(&sync_acc.data).unwrap();
        assert_eq!(
            sync_acc.lamports,
            sync_ta.amount + sync_ta.is_native.unwrap()
        );

        compute_units_consumed
    });

    expect!["4042"].assert_eq(&cus.to_string());
}

fn ix_accounts(sync: Pubkey, sync_acc: TokenAccount) -> [(Pubkey, Account); 2] {
    [
        (sync, account_from_token_acc(sync_acc)),
        mollusk_svm_programs_token::token::keyed_account(),
    ]
}

fn ix(acc: Pubkey) -> Instruction {
    Instruction {
        program_id: PROG_ID,
        accounts: key_signer_writable_to_metas(
            &SyncNativeIxAccs::from_destr(SyncNativeIxAccsDestr { acc }).0,
            &SYNC_NATIVE_IX_IS_SIGNER.0,
            &SYNC_NATIVE_IX_IS_WRITABLE.0,
        )
        .into_iter()
        .chain(core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        }))
        .collect(),
        data: vec![],
    }
}
