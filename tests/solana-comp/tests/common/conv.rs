use spl_token::solana_program::instruction::{AccountMeta, Instruction};

pub fn to_sol_ix<const A: usize>(
    program: &[u8; 32],
    accounts: &[[u8; 32]; A],
    is_signer: &[bool; A],
    is_writable: &[bool; A],
    data: &[u8],
) -> Instruction {
    Instruction {
        program_id: Into::into(*program),
        accounts: accounts
            .iter()
            .copied()
            .zip(is_signer.iter().copied())
            .zip(is_writable.iter().copied())
            .map(|((pubkey, is_signer), is_writable)| AccountMeta {
                pubkey: pubkey.into(),
                is_signer,
                is_writable,
            })
            .collect(),
        data: Vec::from(data),
    }
}
