use proptest::{option, prelude::*, strategy::Union};
use sanctum_spl_token_core::{
    instructions::set_auth::{
        AuthType, SetAuthIxAccs, SetAuthIxAccsDestr, SetAuthIxData, SET_AUTH_IX_IS_SIGNER,
        SET_AUTH_IX_IS_WRITABLE,
    },
    ID,
};
use spl_token::instruction::{set_authority, AuthorityType};

use crate::common::to_sol_ix;

/// Copied from upstream.
/// AuthorityType::from method is probably private because its name clashes with
/// `From::from`
const fn sol_authority_type_from(index: u8) -> Option<AuthorityType> {
    Some(match index {
        0 => AuthorityType::MintTokens,
        1 => AuthorityType::FreezeAccount,
        2 => AuthorityType::AccountOwner,
        3 => AuthorityType::CloseAccount,
        _ => return None,
    })
}

proptest! {
    #[test]
    fn check_set_auth_ix_against_sol(
        ty in Union::new([
            AuthType::MintTokens,
            AuthType::FreezeAccount,
            AuthType::AccountOwner,
            AuthType::CloseAccount,
        ]
        .map(Just)),
        new_auth in option::of(any::<[u8; 32]>()),
        set: [u8; 32],
        auth: [u8; 32],
    ) {
        let sol = set_authority(
            &ID.into(),
            &set.into(),
            new_auth.map(Into::into).as_ref(),
            sol_authority_type_from(ty.into_u8()).unwrap(),
            &auth.into(),
            &[]
        ).unwrap();
        let us = to_sol_ix(
            &ID,
            &SetAuthIxAccs::from_destr(SetAuthIxAccsDestr { set, auth }).0,
            &SET_AUTH_IX_IS_SIGNER.0,
            &SET_AUTH_IX_IS_WRITABLE.0,
            SetAuthIxData::new(ty, new_auth.as_ref()).as_buf(),
        );
        prop_assert_eq!(sol, us);
    }
}
