#![cfg(feature = "test-sbf")]

use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use sanctum_spl_token_jiminy::sanctum_spl_token_core::state::account::RawTokenAccount;
use sanctum_spl_token_test_utils::{
    account_from_mint, bench_binsize, expect_test::expect, init_mint_acc, is_tx_balanced,
    key_signer_writable_to_metas, TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
};
use solana_account::Account;
use solana_pubkey::Pubkey;
use spl_token::{
    solana_program::{instruction::Instruction, program_pack::Pack},
    state::{Account as TokenAccount, Mint},
};

const PROG_NAME: &str = "init_account_test";
const PROG_ID: Pubkey = solana_pubkey::pubkey!("1n1t2J1zuuf34zuZfNNdjJz4MajzcpgUnLdBKdcs5wg");

const INIT: Pubkey = solana_pubkey::pubkey!("FmqrDYpnekE92iPotx8PGQed8fQ9DbeMuE7ASeA9Q72x");
const MINT: Pubkey = solana_pubkey::pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm");
const OWNER: Pubkey = solana_pubkey::pubkey!("2mQbNpB6tbF6cguY7M6NjGozGLTUwJVeUBceWqEH3gkt");

thread_local! {
    static SVM: Mollusk = {
        let mut svm = Mollusk::new(&PROG_ID, PROG_NAME);
        mollusk_svm_programs_token::token::add_program(&mut svm);
        svm
    };
}

#[test]
fn save_binsize() {
    bench_binsize(PROG_NAME, expect!["2696"]);
}

#[test]
fn init_account3_cus() {
    const INIT_ACC_IDX: usize = 0;
    const SUPPLY: u64 = 1_000_000_000;
    const DECIMALS: u8 = 9;

    let accounts = ix_accounts3(
        INIT,
        MINT,
        init_mint_acc(None, SUPPLY, DECIMALS, None),
        OWNER,
    );
    let instr = ix3(INIT, MINT, OWNER);

    let cus = SVM.with(|svm| {
        let InstructionResult {
            compute_units_consumed,
            raw_result,
            resulting_accounts,
            ..
        } = svm.process_and_validate_instruction(&instr, &accounts, &[Check::all_rent_exempt()]);

        raw_result.unwrap();

        assert!(is_tx_balanced(&accounts, &resulting_accounts));

        // Account should be created
        let close_acc = &resulting_accounts[INIT_ACC_IDX].1;
        let ta = TokenAccount::unpack(&close_acc.data).unwrap();
        assert_eq!(ta.mint, MINT);

        compute_units_consumed
    });

    expect!["5305"].assert_eq(&cus.to_string());
}

fn ix_accounts3(
    init: Pubkey,
    mint_pk: Pubkey,
    mint: Mint,
    owner: Pubkey,
) -> [(Pubkey, Account); 4] {
    [
        (
            init,
            Account {
                lamports: TOKEN_ACC_RENT_EXEMPT_LAMPORTS,
                data: vec![0; RawTokenAccount::ACCOUNT_LEN],
                owner: spl_token::ID,
                executable: false,
                rent_epoch: u64::MAX,
            },
        ),
        (mint_pk, account_from_mint(mint)),
        (owner, Account::default()),
        mollusk_svm_programs_token::token::keyed_account(),
    ]
}

fn ix3(init: Pubkey, mint: Pubkey, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: PROG_ID,
        accounts: key_signer_writable_to_metas(
            &[init, mint, owner, spl_token::ID],
            &[false, false, false, false],
            &[true, false, false, false],
        )
        .into(),
        data: vec![3],
    }
}
