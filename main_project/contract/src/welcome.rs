use near_sdk::{near_bindgen, env};

use crate::*;

#[near_bindgen]
impl Welcome {
    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();

        env::log_str(
          format!(
            "Saving greeting '{}' for account '{}'",
            message, account_id,
          ).as_str()
        );

        self.records.insert(&account_id, &message);
    }

    pub fn get_greeting(&self, account_id: AccountId) -> String {
        match self.records.get(&account_id) {
            Some(greeting) => greeting.to_owned(),
            None => "Hello".to_owned(),
        }
    }
}