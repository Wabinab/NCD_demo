use near_sdk::{AccountId, require, env};

pub(crate) fn assert_signer_not_owner(article_owner: AccountId) {
    require!(
      env::signer_account_id() != article_owner,
      "You cannot tip your own article!"
    );
}