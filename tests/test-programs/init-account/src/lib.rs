//! This CPIs spl-token init account
//!
//! Args:
//! - single byte discm determines which variant of InitAccount to call

#![allow(unexpected_cfgs)]

use jiminy_cpi::{
    account::AccountHandle,
    program_error::{ProgramError, INVALID_INSTRUCTION_DATA, NOT_ENOUGH_ACCOUNT_KEYS},
};
use sanctum_spl_token_jiminy::{
    instructions::init_acc::init_acc3_ix_account_handle_perms,
    sanctum_spl_token_core::instructions::init_acc::{InitAcc3IxData, NewInitAcc3IxAccsBuilder},
};

const MAX_ACCOUNTS: usize = 4;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<MAX_ACCOUNTS>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    data: &[u8],
    _prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    match data.first().ok_or(INVALID_INSTRUCTION_DATA)? {
        3 => {
            let accs = *accounts
                .as_slice()
                .first_chunk()
                .ok_or(NOT_ENOUGH_ACCOUNT_KEYS)?;
            process_init_acc3(accounts, &accs)
        }
        _ => Err(INVALID_INSTRUCTION_DATA.into()),
    }
}

fn process_init_acc3<'acc>(
    accounts: &mut Accounts<'acc>,
    &[init, mint, owner]: &[AccountHandle<'acc>; 3],
) -> Result<(), ProgramError> {
    // Using invoke_signed instead of invoke_fwd to test that AccountPerms are correct
    Cpi::new().invoke_signed(
        accounts,
        &sanctum_spl_token_jiminy::sanctum_spl_token_core::ID,
        InitAcc3IxData::new(accounts.get(owner).key()).as_buf(),
        init_acc3_ix_account_handle_perms(
            NewInitAcc3IxAccsBuilder::start()
                .with_init(init)
                .with_mint(mint)
                .build(),
        ),
        &[],
    )
}
