use near_sdk::{Promise, env, require};

use crate::*;

#[near_bindgen]
impl LinkDrop {
    /// Allows given public key to claim sent balance. 
    #[payable]
    pub fn send(
      &mut self,
      public_key: PublicKey
    ) -> Promise {
      require!(
        env::attached_deposit() > ACCESS_KEY_ALLOWANCE,
        format!(
          "Attached deposit {} must be greater than {}",
          env::attached_deposit(),
          ACCESS_KEY_ALLOWANCE,
        ).to_string()
      );

      let value = self.accounts.get(&public_key).unwrap_or(0);

      self.accounts.insert(
        &public_key, 
        &(value + env::attached_deposit() - ACCESS_KEY_ALLOWANCE),
      );

      // eprintln!("Key: {:?}\nValue: {:?}", 
      //   public_key.clone(),
      //   self.accounts.get(&public_key)
      // );

      Promise::new(env::current_account_id()).add_access_key(
        public_key,
        ACCESS_KEY_ALLOWANCE,
        env::current_account_id(),
        "claim,create_account_and_claim".to_owned(),
        // "create_account_and_claim".to_owned(),
      )
    }

    /// Claim tokens for specific account that are attached to the 
    /// public key this transaction is signed with.
    pub fn claim(
      &mut self,
      account_id: AccountId
    ) -> Promise {
      assert_predecessor_is_current("You can only claim with this account.");

      require!(
        env::is_valid_account_id(account_id.as_bytes()),
        "Invalid account id"
      );

      let amount = expect_lightweight(
        self.accounts.remove(&env::signer_account_pk()),
        "Unexpected public key"
      );

      Promise::new(env::current_account_id())
          .delete_key(env::signer_account_pk());
      
      Promise::new(account_id)
          .transfer(amount)
    }

    /// Create new account and claim tokens to it
    pub fn create_account_and_claim(
      &mut self,
      new_account_id: AccountId,
      new_public_key: PublicKey
    ) -> Promise {
      // assert_predecessor_is_current(
      //   "You must create and claim account from this account only."
      // );
      env::log_str(
        format!(
          "Predecessor: {}\nCurrent: {}",
          env::predecessor_account_id(),
          env::current_account_id()
        ).as_str(),
      );

      eprintln!("Pass");
      require!(
        env::is_valid_account_id(new_account_id.as_bytes()),
        "Invalid account id"
      );

      eprintln!("Pass");

      let amount = self.accounts.remove(&env::signer_account_pk()).expect("What?");

      eprintln!("Amount: {}", amount);
      if amount == 0 { env::panic_str("Amount is zero")};
      Promise::new(new_account_id)
          .create_account()
          .add_full_access_key(new_public_key)
          .transfer(amount)
          .then(
            ext_self::on_account_created_and_claimed(
              U128(amount),
              env::current_account_id(),
              NO_DEPOSIT,
              ON_CREATE_ACCOUNT_CALLBACK_GAS,
            )
          )
    }


    // =====================CALLBACKS=======================


    pub fn on_account_created_and_claimed(
      &mut self,
      amount: U128
    ) -> bool {
      assert_predecessor_is_current(
        "Callback can only be called from the contract"
      );

      let creation_succeeded = is_promise_success();

      if creation_succeeded {
        Promise::new(env::current_account_id())
            .delete_key(env::signer_account_pk());
      } else {
        // Put amount back
        self.accounts
            .insert(&env::signer_account_pk(), &amount.into());
      }

      creation_succeeded
    }


    /// Returns the balance associated with given key.
    pub fn get_key_balance(
      &self,
      key: PublicKey
    ) -> U128 {
      expect_lightweight(
        self.accounts.get(&key.into()), 
        "Key is missing"
      ).into()
    }
}


