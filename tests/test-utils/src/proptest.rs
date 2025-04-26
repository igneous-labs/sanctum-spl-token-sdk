use proptest::{prelude::*, strategy::Union};
use spl_token::{
    solana_program::{program_option::COption, pubkey::Pubkey},
    state::{Account, AccountState},
};

pub fn any_token_acc() -> impl Strategy<Value = Account> {
    (
        any::<[u8; 32]>(),
        any::<[u8; 32]>(),
        any::<u64>(),
        any_coption::<[u8; 32]>(),
        Union::new([Just(AccountState::Frozen), Just(AccountState::Initialized)]),
        any_coption::<u64>(),
        any::<u64>(),
        any_coption::<[u8; 32]>(),
    )
        .prop_map(
            |(
                mint,
                owner,
                amount,
                delegate,
                state,
                is_native,
                delegated_amount,
                close_authority,
            )| Account {
                mint: Pubkey::new_from_array(mint),
                owner: Pubkey::new_from_array(owner),
                amount,
                delegate: delegate.map(Pubkey::new_from_array),
                state,
                is_native,
                delegated_amount,
                close_authority: close_authority.map(Pubkey::new_from_array),
            },
        )
}

fn any_coption<T: Arbitrary + Clone + 'static>() -> impl Strategy<Value = COption<T>> {
    Union::new([
        Just(COption::None).boxed(),
        any::<T>().prop_map(|val| COption::Some(val)).boxed(),
    ])
}
