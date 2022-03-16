use near_sdk::json_types::{U128};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
  near_bindgen, ext_contract, AccountId, Balance, 
  PublicKey, Gas
};
use near_sdk::collections::{LookupMap};


pub use crate::welcome::*;
// pub use crate::linkdrop::*;
pub use crate::internal::*;
pub use crate::signup::*;

pub mod welcome;
// pub mod linkdrop;
pub mod internal;
pub mod signup;


const NO_DEPOSIT: u128 = 0;

pub const GAS: Gas = Gas(20_000_000_000_000);


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<AccountId, String>,
}

impl Default for Welcome {
  fn default() -> Self {
    Self {
      records: LookupMap::new(b"a".to_vec()),
    }
  }
}





// =====================================
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Signup {

}

impl Default for Signup {
    fn default() -> Self {
      Self {}
    }
}


#[ext_contract(ext_linkdrop)]
pub trait ExtLinkDrop {
    fn send(public_key: PublicKey) -> Promise;

    fn get_key_balance(public_key: PublicKey) -> U128;
}


#[ext_contract(ext_signup)]
pub trait ExtSignUp {
    fn on_send(public_key: PublicKey) -> Promise;
}


// =====================================








#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, PublicKey, BlockHeight};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
      VMContext {
          current_account_id: "alice_near".parse().unwrap(),
          signer_account_id: "bob_near".parse().unwrap(),
          signer_account_pk: vec![0, 1, 2],
          predecessor_account_id: "carol_near".parse().unwrap(),
          input,
          block_index: 0,
          block_timestamp: 0,
          account_balance: 0,
          account_locked_balance: 0,
          storage_usage: 0,
          attached_deposit: 0,
          prepaid_gas: 10u64.pow(18),
          random_seed: vec![0, 1, 2],
          is_view,
          output_data_receivers: vec![],
          epoch_height: 19,
      }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_owned());
        assert_eq!(
            "howdy",
            contract.get_greeting("bob_near".parse().unwrap())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello",
            contract.get_greeting("francis.near".parse().unwrap())
        );
    }

}
