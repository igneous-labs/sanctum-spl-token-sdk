//! .so file size 6128

#![cfg(feature = "test-sbf")]

use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use proptest::prelude::*;
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::transfer::{
    NewTransferCheckedIxAccsBuilder, NewTransferIxAccsBuilder, TRANSFER_CHECKED_IX_IS_SIGNER,
    TRANSFER_CHECKED_IX_IS_WRITABLE, TRANSFER_IX_IS_SIGNER, TRANSFER_IX_IS_WRITABLE,
};
use sanctum_spl_token_test_utils::{
    account_from_mint, account_from_token_acc, init_mint_acc, is_tx_balanced,
    key_signer_writable_to_metas, silence_mollusk_prog_logs, token_acc_for_trf,
    TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
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

const PROG_NAME: &str = "transfer_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("8AxeW1wz52Qe8cWV5NZf8eSF2QaEy5xhvv3kTZxdcDhX");

const SRC: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const DST: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");
const AUTH: Pubkey = solana_pubkey::pubkey!("2AHbbAHQQrQsEP7yrE9PGWpkn7Uz27PKJBByRwkurnWG");
const MINT: Pubkey = solana_pubkey::pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm");
const AMT: u64 = 29_125_461_325;

const SRC_ACC_IDX: usize = 1;
const DST_ACC_IDX: usize = 2;

thread_local! {
    static SVM: Mollusk = {
        let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        svm
    };
}

// CUs: 5906
#[test]
fn transfer_all_non_native_cus() {
    let accounts = ix_accounts(
        SRC,
        token_acc_for_trf(MINT, AMT, false, AUTH),
        DST,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(SRC, DST, AUTH, None);

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        eprintln!("{compute_units_consumed} CUs");

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        [(SRC_ACC_IDX, 0), (DST_ACC_IDX, AMT)]
            .iter()
            .for_each(|(idx, expected_amt)| {
                let acc = &resulting_accounts[*idx].1;
                assert_eq!(
                    *expected_amt,
                    TokenAccount::unpack(&acc.data).unwrap().amount
                );
            });
    });
}

// CUs: 5870
#[test]
fn transfer_arg_non_native_cus() {
    let accounts = ix_accounts(
        SRC,
        token_acc_for_trf(MINT, AMT, false, AUTH),
        DST,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(SRC, DST, AUTH, Some(AMT));

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        eprintln!("{compute_units_consumed} CUs");

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        [(SRC_ACC_IDX, 0), (DST_ACC_IDX, AMT)]
            .iter()
            .for_each(|(idx, expected_amt)| {
                let acc = &resulting_accounts[*idx].1;
                assert_eq!(
                    *expected_amt,
                    TokenAccount::unpack(&acc.data).unwrap().amount
                );
            });
    })
}

// CUs: 7566
#[test]
fn transfer_checked_arg_non_native_cus() {
    let accounts = ix_accounts_checked(
        SRC,
        token_acc_for_trf(MINT, AMT, false, AUTH),
        DST,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
        MINT,
        init_mint_acc(None, 2 * AMT, 1, None),
    );
    let instr = ix_checked(SRC, DST, AUTH, MINT, Some(AMT));

    SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        eprintln!("{compute_units_consumed} CUs");

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        [(SRC_ACC_IDX, 0), (DST_ACC_IDX, AMT)]
            .iter()
            .for_each(|(idx, expected_amt)| {
                let acc = &resulting_accounts[*idx].1;
                assert_eq!(
                    *expected_amt,
                    TokenAccount::unpack(&acc.data).unwrap().amount
                );
            });
    })
}

proptest! {
    #[test]
    fn transfer_all_valid_cases(
        (mint, src, dst) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint)))
                .prop_flat_map(|(mint, src)| (Just(mint), Just(src),  any::<[u8; 32]>().prop_filter("", move |k| *k != mint && *k != src))),
        auth: [u8; 32],
        is_native: bool,
        (dst_amt, src_amt, trf_amt) in
            (0..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS)
                .prop_flat_map(|x| (Just(x), 0..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS - x))
                .prop_flat_map(|(x, y)| (Just(x), Just(y), 0..=y)),
    ) {
        let [mint, src, dst, auth] = [mint, src, dst, auth]
            .map(Pubkey::new_from_array);
        silence_mollusk_prog_logs();

        let accounts = ix_accounts(
            src,
            token_acc_for_trf(mint, src_amt, is_native, auth),
            dst,
            token_acc_for_trf(mint, dst_amt, is_native, Default::default()),
            auth,
        );

        for arg in [None, Some(trf_amt)] {
            let instr = ix(src, dst, auth, arg);

            SVM.with(|svm| {
                let InstructionResult {
                    raw_result,
                    resulting_accounts,
                    ..
                } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

                raw_result.unwrap();

                prop_assert!(is_tx_balanced(&accounts, &resulting_accounts));

                let expected_trf_amt = match arg {
                    None => src_amt,
                    Some(a) => a,
                };
                for (idx, expected_amt) in [
                    (SRC_ACC_IDX, src_amt - expected_trf_amt),
                    (DST_ACC_IDX, dst_amt + expected_trf_amt),
                ] {
                    let acc = &resulting_accounts[idx].1;
                    prop_assert_eq!(
                        expected_amt,
                        TokenAccount::unpack(&acc.data).unwrap().amount
                    );
                }

                Ok(())
            }).unwrap();
        }
    }
}

proptest! {
    #[test]
    fn transfer_checked_all_valid_cases_should_match_transfer(
        (mint, src, dst) in
            any::<[u8; 32]>().prop_flat_map(|mint| (Just(mint), any::<[u8; 32]>().prop_filter("", move |k| *k != mint)))
                .prop_flat_map(|(mint, src)| (Just(mint), Just(src),  any::<[u8; 32]>().prop_filter("", move |k| *k != mint && *k != src))),
        auth: [u8; 32],
        decimals: u8,
        is_native: bool,
        (dst_amt, src_amt, trf_amt) in
            (0..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS)
                .prop_flat_map(|x| (Just(x), 0..=u64::MAX - TOKEN_ACC_RENT_EXEMPT_LAMPORTS - x))
                .prop_flat_map(|(x, y)| (Just(x), Just(y), 0..=y)),
    ) {
        let [mint, src, dst, auth] = [mint, src, dst, auth]
            .map(Pubkey::new_from_array);
        silence_mollusk_prog_logs();

        let accounts = ix_accounts(
            src,
            token_acc_for_trf(mint, src_amt, is_native, auth),
            dst,
            token_acc_for_trf(mint, dst_amt, is_native, Default::default()),
            auth,
        );
        let accounts_checked = ix_accounts_checked(
            src,
            token_acc_for_trf(mint, src_amt, is_native, auth),
            dst,
            token_acc_for_trf(mint, dst_amt, is_native, Default::default()),
            auth,
            mint,
            init_mint_acc(None, u64::MAX, decimals, None),
        );

        for arg in [None, Some(trf_amt)] {
            let instr = ix(src, dst, auth, arg);
            let instr_checked = ix_checked(src, dst, auth, mint, arg);

            SVM.with(|svm| {
                let r = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);
                let r_checked = svm.process_and_validate_instruction(&instr_checked, &accounts_checked, &[Check::all_rent_exempt()]);

                prop_assert_eq!(r.raw_result, r_checked.raw_result);
                prop_assert_eq!(r.program_result, r_checked.program_result);
                prop_assert_eq!(r.return_data, r_checked.return_data);
                // zip so that it terminates early and omits check for mint
                // since mint account only exists in r_checked and not r
                prop_assert!(
                    r.resulting_accounts.iter().zip(r_checked.resulting_accounts.iter())
                        .all(|(a1, a2)| a1 == a2)
                );

                Ok(())
            }).unwrap();
        }
    }
}

fn ix_accounts(
    src: Pubkey,
    src_acc: TokenAccount,

    dst: Pubkey,
    dst_acc: TokenAccount,

    auth: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        mollusk_svm_programs_token::token::keyed_account(),
        (src, account_from_token_acc(src_acc)),
        (dst, account_from_token_acc(dst_acc)),
        (auth, Account::default()),
    ]
}

fn ix_accounts_checked(
    src: Pubkey,
    src_acc: TokenAccount,

    dst: Pubkey,
    dst_acc: TokenAccount,

    auth: Pubkey,

    mint: Pubkey,
    mint_acc: Mint,
) -> [(Pubkey, Account); 5] {
    let [a0, a1, a2, a3] = ix_accounts(src, src_acc, dst, dst_acc, auth);
    [a0, a1, a2, a3, (mint, account_from_mint(mint_acc))]
}

fn ix(src: Pubkey, dst: Pubkey, auth: Pubkey, amt: Option<u64>) -> Instruction {
    const DISCM: u8 = 0;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &NewTransferIxAccsBuilder::start()
                .with_src(src)
                .with_dst(dst)
                .with_auth(auth)
                .build()
                .0,
            &TRANSFER_IX_IS_SIGNER.0,
            &TRANSFER_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: amt.map_or_else(
            || vec![DISCM],
            |amt| core::iter::once(DISCM).chain(amt.to_le_bytes()).collect(),
        ),
    }
}

fn ix_checked(
    src: Pubkey,
    dst: Pubkey,
    auth: Pubkey,
    mint: Pubkey,
    amt: Option<u64>,
) -> Instruction {
    const DISCM: u8 = 1;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &NewTransferCheckedIxAccsBuilder::start()
                .with_src(src)
                .with_dst(dst)
                .with_auth(auth)
                .with_mint(mint)
                .build()
                .0,
            &TRANSFER_CHECKED_IX_IS_SIGNER.0,
            &TRANSFER_CHECKED_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: amt.map_or_else(
            || vec![DISCM],
            |amt| core::iter::once(DISCM).chain(amt.to_le_bytes()).collect(),
        ),
    }
}
