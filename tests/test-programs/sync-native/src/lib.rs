//! This CPIs spl-token sync native
//!
//! Args:
//! - No instruction data needed

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{ProgramError, NOT_ENOUGH_ACCOUNT_KEYS};
use sanctum_spl_token_jiminy::{
    instructions::sync_native::sync_native_ix_account_handle_perms,
    sanctum_spl_token_core::instructions::sync_native::{
        SyncNativeIxAccs, SyncNativeIxAccsDestr, SyncNativeIxData,
    },
};

const MAX_ACCOUNTS: usize = 1;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<MAX_ACCOUNTS>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    _data: &[u8],
    _prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    let acc = *accounts.as_slice().first().ok_or(NOT_ENOUGH_ACCOUNT_KEYS)?;

    // Using invoke_signed instead of invoke_fwd to test that AccountPerms are correct
    Cpi::new().invoke_signed(
        accounts,
        &sanctum_spl_token_jiminy::sanctum_spl_token_core::ID,
        SyncNativeIxData::as_buf(),
        sync_native_ix_account_handle_perms(SyncNativeIxAccs::from_destr(SyncNativeIxAccsDestr {
            acc,
        })),
        &[],
    )
}
