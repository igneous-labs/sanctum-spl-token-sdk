//! This CPIs spl-token transfer
//!
//! Args:
//! - `discm`: u8. If 0, uses Transfer. If 1, uses TransferChecked with decimals read from mint acc
//! - `amount: Option<u64>` amount to transfer. If None (empty ix data), entire balance of src is transferred.

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{BuiltInProgramError, ProgramError};
use sanctum_spl_token_jiminy::{
    instructions::transfer::{
        transfer_checked_ix_account_handle_perms, transfer_ix_account_handle_perms,
    },
    sanctum_spl_token_core::{
        instructions::transfer::{
            TransferCheckedIxAccs, TransferCheckedIxData, TransferIxAccs, TransferIxData,
        },
        state::{
            account::{RawTokenAccount, TokenAccount},
            mint::{Mint, RawMint},
        },
    },
};

const MAX_ACCOUNTS: usize = 5;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;
type Cpi = jiminy_cpi::Cpi<MAX_ACCOUNTS>;

jiminy_entrypoint::entrypoint!(process_ix, MAX_ACCOUNTS);

fn process_ix(
    accounts: &mut Accounts,
    data: &[u8],
    _prog_id: &[u8; 32],
) -> Result<(), ProgramError> {
    let Some(a) = accounts.as_slice().first_chunk() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [spl_token, src] = *a;

    let Some((discm, amt_data)) = data.split_first() else {
        return Err(ProgramError::custom(1));
    };

    let amt = match *amt_data {
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

    let spl_token_key = *accounts.get(spl_token).key();

    match discm {
        // Transfer
        0 => {
            let Some((_spl_token, a)) = accounts.as_slice().split_last_chunk() else {
                return Err(ProgramError::from_builtin(
                    BuiltInProgramError::NotEnoughAccountKeys,
                ));
            };
            Cpi::new().invoke_signed(
                accounts,
                &spl_token_key,
                TransferIxData::new(amt).as_buf(),
                transfer_ix_account_handle_perms(TransferIxAccs(*a)),
                &[],
            )
        }
        // TransferChecked
        1 => {
            let Some((_spl_token, a)) = accounts.as_slice().split_last_chunk() else {
                return Err(ProgramError::from_builtin(
                    BuiltInProgramError::NotEnoughAccountKeys,
                ));
            };
            let accs = TransferCheckedIxAccs(*a);
            let mint_acc = accounts.get(*accs.mint());
            let decimals = RawMint::of_acc_data(mint_acc.data())
                .and_then(Mint::try_from_raw)
                .ok_or(ProgramError::from_builtin(
                    BuiltInProgramError::InvalidAccountData,
                ))?
                .decimals();
            Cpi::new().invoke_signed(
                accounts,
                &spl_token_key,
                TransferCheckedIxData::new(amt, decimals).as_buf(),
                transfer_checked_ix_account_handle_perms(accs),
                &[],
            )
        }
        _ => Err(ProgramError::from_builtin(
            BuiltInProgramError::InvalidInstructionData,
        )),
    }
}
