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

const CLOSE: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const DST: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const MINT: Pubkey = solana_pubkey::pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm");

const CLOSE_ACC_IDX: usize = 1;
const DST_ACC_IDX: usize = 2;

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
    let dst_initial_lamports: u64 = 5_000_000_000;
    let accounts = ix_accounts(
        CLOSE,
        token_acc_for_trf(MINT, 0, false, AUTH),
        DST,
        dst_initial_lamports,
        AUTH,
    );
    let instr = ix(CLOSE, DST, AUTH);

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
        let close_acc = &resulting_accounts[CLOSE_ACC_IDX].1;
        assert_eq!(0, close_acc.lamports);
        assert!(close_acc.data.is_empty());

        // Destination should receive all lamports
        let dst_acc = &resulting_accounts[DST_ACC_IDX].1;
        assert_eq!(
            dst_initial_lamports + TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
            dst_acc.lamports
        );

        save_cus_to_file("close", compute_units_consumed);
    });
}

proptest! {
    #[test]
    fn close_account_all_valid_cases(
        (mint, close, dst) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint)))
                .prop_flat_map(|(mint, close)| (Just(mint), Just(close),  any::<[u8; 32]>().prop_filter("", move |k| *k != mint && *k != close))),
        auth: [u8; 32],
        dst_initial_lamports in 0u64..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
    ) {
        let [mint, close, dst, auth] = [mint, close, dst, auth]
            .map(Pubkey::new_from_array);
        silence_mollusk_prog_logs();

        let accounts = ix_accounts(
            close,
            token_acc_for_trf(mint, 0, false, auth),
            dst,
            dst_initial_lamports,
            auth,
        );
        let instr = ix(close, dst, auth);

        SVM.with(|svm| {
            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

            raw_result.unwrap();

            prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

            // Account should be cleared
            let close_acc = &resulting_accounts[CLOSE_ACC_IDX].1;
            prop_assert_eq!(0, close_acc.lamports);
            prop_assert!(close_acc.data.is_empty());

            // Refund rent to should receive all lamports
            let dst_acc = &resulting_accounts[DST_ACC_IDX].1;
            prop_assert_eq!(
                dst_initial_lamports + TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
                dst_acc.lamports
            );

            Ok(())
        }).unwrap();
    }
}

fn ix_accounts(
    close: Pubkey,
    close_acc: TokenAccount,
    dst: Pubkey,
    dst_lamports: u64,
    auth: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        mollusk_svm_programs_token::token::keyed_account(),
        (close, account_from_token_acc(close_acc)),
        (
            dst,
            Account {
                lamports: dst_lamports,
                ..Account::default()
            },
        ),
        (auth, Account::default()),
    ]
}

fn ix(close: Pubkey, dst: Pubkey, auth: Pubkey) -> Instruction {
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
                .with_close(close)
                .with_dst(dst)
                .with_auth(auth)
                .0,
            &CLOSE_ACCOUNT_IX_IS_SIGNER.0,
            &CLOSE_ACCOUNT_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: vec![],
    }
}
