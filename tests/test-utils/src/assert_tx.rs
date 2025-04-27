//! All checks here need to pass for a mollusk instruction sequence execution,
//! else it is an invalid transaction.

use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_rent::Rent;

pub fn is_tx_balanced(pre: &[(Pubkey, Account)], post: &[(Pubkey, Account)]) -> bool {
    let sum_lamports = |acc, (_pk, account): &(Pubkey, Account)| acc + account.lamports;
    let [pre_lamports, post_lamports] = [pre, post].map(|slice| slice.iter().fold(0, sum_lamports));
    pre_lamports == post_lamports
}

pub fn are_all_accounts_rent_exempt(post: &[(Pubkey, Account)]) -> Result<(), &(Pubkey, Account)> {
    let rent = Rent::default();
    post.iter().try_for_each(|tup| {
        let (_, acc) = tup;
        if acc.lamports == 0 || acc.lamports >= rent.minimum_balance(acc.data.len()) {
            Ok(())
        } else {
            Err(tup)
        }
    })
}
