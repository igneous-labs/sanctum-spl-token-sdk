//! This CPIs spl-token close account
//!
//! Args:
//! - No instruction data needed

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{BuiltInProgramError, ProgramError};
use sanctum_spl_token_jiminy::{
    instructions::close_account::close_account_ix_account_handle_perms,
    sanctum_spl_token_core::instructions::close_account::{
        CloseAccountIxData, NewCloseAccountIxAccsBuilder,
    },
};

const MAX_ACCOUNTS: usize = 4;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<MAX_ACCOUNTS>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    _data: &[u8],
    _prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    let [spl_token, account_to_close, refund_rent_to, auth] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [spl_token, account_to_close, refund_rent_to, auth] =
        [spl_token, account_to_close, refund_rent_to, auth].map(|h| *h);

    let spl_token_key = *accounts.get(spl_token).key();
    Cpi::new().invoke_signed(
        accounts,
        &spl_token_key,
        CloseAccountIxData::as_buf(),
        close_account_ix_account_handle_perms(
            NewCloseAccountIxAccsBuilder::start()
                .with_account_to_close(account_to_close)
                .with_refund_rent_to(refund_rent_to)
                .with_auth(auth)
                .build(),
        ),
        &[],
    )
}
