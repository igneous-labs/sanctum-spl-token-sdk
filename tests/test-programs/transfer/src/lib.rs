//! This CPIs spl-token transfer
//!
//! Args:
//! - `amount: Option<u64>` amount to transfer. If None (empty ix data), entire balance of src is transferred.

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{BuiltInProgramError, ProgramError};
use sanctum_spl_token_jiminy::{
    instructions::transfer::{transfer_ix, TransferIxAccounts},
    sanctum_spl_token_core::{
        instructions::transfer::TransferIxData,
        state::account::{RawTokenAccount, TokenAccount},
    },
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
    let [spl_token, src, dst, auth] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [spl_token, src, dst, auth] = [spl_token, src, dst, auth].map(|h| *h);

    let amt = match *data {
        [] => {
            let src_acc = accounts.get(src);
            let src_acc = RawTokenAccount::of_acc_data(src_acc.data())
                .and_then(TokenAccount::try_from_raw)
                .ok_or(ProgramError::from_builtin(
                    BuiltInProgramError::InvalidAccountData,
                ))?;
            src_acc.amount()
        }
        [i0, i1, i2, i3, i4, i5, i6, i7] => u64::from_le_bytes([i0, i1, i2, i3, i4, i5, i6, i7]),
        _ => {
            return Err(ProgramError::from_builtin(
                BuiltInProgramError::InvalidInstructionData,
            ))
        }
    };

    Cpi::new().invoke_signed(
        accounts,
        transfer_ix(
            spl_token,
            TransferIxAccounts::memset(spl_token)
                .with_src(src)
                .with_dst(dst)
                .with_auth(auth),
            &TransferIxData::new(amt),
        ),
        &[],
    )
}
