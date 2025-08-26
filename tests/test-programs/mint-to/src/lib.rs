//! This CPIs spl-token mint to
//!
//! Args:
//! - `amount: Option<u64>` amount to mint. If None (empty ix data), (u64::MAX - current_supply) is minted.

#![allow(unexpected_cfgs)]

use jiminy_cpi::program_error::{BuiltInProgramError, ProgramError};
use sanctum_spl_token_jiminy::{
    instructions::mint_to::mint_to_ix_account_handle_perms,
    sanctum_spl_token_core::{
        instructions::mint_to::{MintToIxData, NewMintToIxAccsBuilder},
        state::mint::{Mint, RawMint},
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
    let [spl_token, mint, to, auth] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [spl_token, mint, to, auth] = [spl_token, mint, to, auth].map(|h| *h);

    let amt = match *data {
        [] => {
            let mint_acc = accounts.get(mint);
            let mint_acc = RawMint::of_acc_data(mint_acc.data())
                .and_then(Mint::try_from_raw)
                .ok_or(ProgramError::from_builtin(
                    BuiltInProgramError::InvalidAccountData,
                ))?;
            u64::MAX - mint_acc.supply()
        }
        [i0, i1, i2, i3, i4, i5, i6, i7] => u64::from_le_bytes([i0, i1, i2, i3, i4, i5, i6, i7]),
        _ => {
            return Err(ProgramError::from_builtin(
                BuiltInProgramError::InvalidInstructionData,
            ))
        }
    };

    let spl_token_key = *accounts.get(spl_token).key();
    Cpi::new().invoke_signed(
        accounts,
        &spl_token_key,
        MintToIxData::new(amt).as_buf(),
        mint_to_ix_account_handle_perms(
            NewMintToIxAccsBuilder::start()
                .with_mint(mint)
                .with_to(to)
                .with_auth(auth)
                .build(),
        ),
        &[],
    )
}
