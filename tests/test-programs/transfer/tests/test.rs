//! .so file size 3216

#![cfg(feature = "test-sbf")]

use mollusk_svm::{result::InstructionResult, Mollusk};
use proptest::prelude::*;
use sanctum_spl_token_jiminy::sanctum_spl_token_core::instructions::transfer::{
    TransferIxAccs, TRANSFER_IX_IS_SIGNER, TRANSFER_IX_IS_WRITABLE,
};
use sanctum_spl_token_test_utils::{
    account_from_token_acc, are_all_accounts_rent_exempt, is_tx_balanced,
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
    state::Account as TokenAccount,
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

// CUs: 5875
#[test]
fn transfer_all_non_native_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        SRC,
        token_acc_for_trf(MINT, AMT, false, AUTH),
        DST,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(SRC, DST, AUTH, None);

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

    [(SRC_ACC_IDX, 0), (DST_ACC_IDX, AMT)]
        .iter()
        .for_each(|(idx, expected_amt)| {
            let acc = &resulting_accounts[*idx].1;
            assert_eq!(
                *expected_amt,
                TokenAccount::unpack(&acc.data).unwrap().amount
            );
        });
}

// CUs: 5837
#[test]
fn transfer_arg_non_native_cus() {
    let svm = mollusk();
    let accounts = ix_accounts(
        SRC,
        token_acc_for_trf(MINT, AMT, false, AUTH),
        DST,
        token_acc_for_trf(MINT, 0, false, Default::default()),
        AUTH,
    );
    let instr = ix(SRC, DST, AUTH, Some(AMT));

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

    [(SRC_ACC_IDX, 0), (DST_ACC_IDX, AMT)]
        .iter()
        .for_each(|(idx, expected_amt)| {
            let acc = &resulting_accounts[*idx].1;
            assert_eq!(
                *expected_amt,
                TokenAccount::unpack(&acc.data).unwrap().amount
            );
        });
}

proptest! {
    #[test]
    fn transfer_all_cases(
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
        let svm = mollusk();
        silence_mollusk_prog_logs();

        for arg in [None, Some(trf_amt)] {
            let accounts = ix_accounts(
                src,
                token_acc_for_trf(mint, src_amt, is_native, auth),
                dst,
                token_acc_for_trf(mint, dst_amt, is_native, Default::default()),
                auth,
            );
            let instr = ix(src, dst, auth, arg);

            let InstructionResult {
                raw_result,
                resulting_accounts,
                ..
            } = svm.process_instruction(&instr, &accounts);

            raw_result.unwrap();

            are_all_accounts_rent_exempt(&resulting_accounts).unwrap();
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
        }
    }
}

fn mollusk() -> Mollusk {
    let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
    mollusk_svm_programs_token::token::add_program(&mut svm);
    svm
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

fn ix(src: Pubkey, dst: Pubkey, auth: Pubkey, amt: Option<u64>) -> Instruction {
    type TransferIxKeys = TransferIxAccs<Pubkey>;

    Instruction {
        program_id: PROG_ID,
        accounts: core::iter::once(AccountMeta {
            pubkey: spl_token::ID,
            is_signer: false,
            is_writable: false,
        })
        .chain(key_signer_writable_to_metas(
            &TransferIxKeys::memset(PROG_ID)
                .with_src(src)
                .with_dst(dst)
                .with_auth(auth)
                .0,
            &TRANSFER_IX_IS_SIGNER.0,
            &TRANSFER_IX_IS_WRITABLE.0,
        ))
        .collect(),
        data: amt.map_or_else(Vec::new, |amt| Vec::from(amt.to_le_bytes())),
    }
}
