#![cfg(feature = "test-sbf")]

use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use proptest::prelude::*;
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::close_account::{
    CloseAccountIxAccs, CLOSE_ACCOUNT_IX_IS_SIGNER, CLOSE_ACCOUNT_IX_IS_WRITABLE,
};
use sanctum_spl_token_test_utils::{
    account_from_token_acc, is_tx_balanced, key_signer_writable_to_metas, save_binsize_to_file,
    save_cus_to_file, silence_mollusk_prog_logs, token_acc_for_trf, TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::instruction::{AccountMeta, Instruction},
    state::Account as TokenAccount,
};

const PROG_NAME: &str = "close_account_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("C1ose2J1zuuf34zuZfNNdjJz4MajzcpgUnLdBKdcs5wg");

const ACCOUNT_TO_CLOSE: Pubkey =
    solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const REFUND_RENT_TO: Pubkey =
    solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const MINT: Pubkey = solana_pubkey::pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm");

const ACCOUNT_TO_CLOSE_ACC_IDX: usize = 1;
const REFUND_RENT_TO_ACC_IDX: usize = 2;

thread_local! {
    static SVM: Mollusk = {
        let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        svm
    };
}

#[test]
fn save_binsize() {
    save_binsize_to_file(PROG_NAME);
}

#[test]
fn close_account_cus() {
    let refund_rent_to_initial_lamports: u64 = 5_000_000_000;
    let accounts = ix_accounts(
        ACCOUNT_TO_CLOSE,
        token_acc_for_trf(MINT, 0, false, AUTH),
        REFUND_RENT_TO,
        refund_rent_to_initial_lamports,
        AUTH,
    );
    let instr = ix(ACCOUNT_TO_CLOSE, REFUND_RENT_TO, AUTH);

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        // Account should be cleared
        let account_to_close_acc = &resulting_accounts[ACCOUNT_TO_CLOSE_ACC_IDX].1;
        assert_eq!(0, account_to_close_acc.lamports);
        assert!(account_to_close_acc.data.is_empty());

        // Refund rent to should receive all lamports
        let refund_rent_to_acc = &resulting_accounts[REFUND_RENT_TO_ACC_IDX].1;
        assert_eq!(
            refund_rent_to_initial_lamports + TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
            refund_rent_to_acc.lamports
        );

        save_cus_to_file("close", compute_units_consumed);
    });
}

proptest! {
    #[test]
    fn close_account_all_valid_cases(
        (mint, account_to_close, refund_rent_to) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint)))
                .prop_flat_map(|(mint, account)| (Just(mint), Just(account),  any::<[u8; 32]>().prop_filter("", move |k| *k != mint && *k != account))),
        auth: [u8; 32],
        refund_rent_to_initial_lamports in 0u64..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
    ) {
        let [mint, account_to_close, refund_rent_to, auth] = [mint, account_to_close, refund_rent_to, auth]
            .map(Pubkey::new_from_array);
        silence_mollusk_prog_logs();

        let accounts = ix_accounts(
            account_to_close,
            token_acc_for_trf(mint, 0, false, auth),
            refund_rent_to,
            refund_rent_to_initial_lamports,
            auth,
        );
        let instr = ix(account_to_close, refund_rent_to, auth);

        SVM.with(|svm| {
            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

            raw_result.unwrap();

            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            // Account should be cleared
            let account_to_close_acc = &resulting_accounts[ACCOUNT_TO_CLOSE_ACC_IDX].1;
            prop_assert_eq!(0, account_to_close_acc.lamports);
            prop_assert!(account_to_close_acc.data.is_empty());

            // Refund rent to should receive all lamports
            let refund_rent_to_acc = &resulting_accounts[REFUND_RENT_TO_ACC_IDX].1;
            prop_assert_eq!(
                refund_rent_to_initial_lamports + TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
                refund_rent_to_acc.lamports
            );

            Ok(())
        }).unwrap();
    }
}

fn ix_accounts(
    account_to_close: Pubkey,
    account_to_close_acc: TokenAccount,
    refund_rent_to: Pubkey,
    refund_rent_to_lamports: u64,
    auth: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        mollusk_svm_programs_token::token::keyed_account(),
        (
            account_to_close,
            account_from_token_acc(account_to_close_acc),
        ),
        (
            refund_rent_to,
            Account {
                lamports: refund_rent_to_lamports,
                ..Account::default()
            },
        ),
        (auth, Account::default()),
    ]
}

fn ix(account_to_close: Pubkey, refund_rent_to: Pubkey, auth: Pubkey) -> Instruction {
    type CloseAccountIxKeys = CloseAccountIxAccs<Pubkey>;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &CloseAccountIxKeys::memset(PROG_ID)
                .with_account_to_close(account_to_close)
                .with_refund_rent_to(refund_rent_to)
                .with_auth(auth)
                .0,
            &CLOSE_ACCOUNT_IX_IS_SIGNER.0,
            &CLOSE_ACCOUNT_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: vec![],
    }
}
