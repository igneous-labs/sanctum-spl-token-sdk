//! This CPIs spl-token burn
//!
//! Args:
//! - `amount: Option<u64>` amount to burn. If None (empty ix data), burns all the tokens in token account.

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{BuiltInProgramError, ProgramError};
use sanctum_spl_token_jiminy::{
    instructions::burn::{burn_ix, BurnIxAccounts},
    sanctum_spl_token_core::{
        instructions::burn::BurnIxData,
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
    let [spl_token, from, mint, auth] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [spl_token, from, mint, auth] = [spl_token, from, mint, auth].map(|h| *h);

    let amt = match *data {
        [] => {
            let token_acc = accounts.get(from);
            let token_acc = RawTokenAccount::of_acc_data(token_acc.data())
                .and_then(TokenAccount::try_from_raw)
                .ok_or(ProgramError::from_builtin(
                    BuiltInProgramError::InvalidAccountData,
                ))?;
            token_acc.amount()
        }

        // Everything here applies to similar code in the other test programs
        //
        // this looks stupid, but less error prone than
        // `if len() == 8`, because length is explicit.
        //
        // Perf characteristics:
        // - Causes this branch to take 3 fewer CUs, but the empty branch above to take 3 more
        //  - But if you put this branch before `[]`, this change gets reversed
        // - Binary sizes are the same
        [i0, i1, i2, i3, i4, i5, i6, i7] => u64::from_le_bytes([i0, i1, i2, i3, i4, i5, i6, i7]),

        _ => {
            return Err(ProgramError::from_builtin(
                BuiltInProgramError::InvalidInstructionData,
            ))
        }
    };

    Cpi::new().invoke_signed(
        accounts,
        burn_ix(
            spl_token,
            BurnIxAccounts::memset(spl_token)
                .with_from(from)
                .with_mint(mint)
                .with_auth(auth),
            &BurnIxData::new(amt),
        ),
        &[],
    )
}
