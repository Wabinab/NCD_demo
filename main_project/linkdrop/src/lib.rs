use near_sdk::json_types::{U128};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
  near_bindgen, ext_contract, AccountId, Balance, 
  PublicKey, Gas
};
use near_sdk::collections::{LookupMap};

pub mod linkdrop;
pub mod internal;

pub use crate::linkdrop::*;
use crate::internal::*;


// 0.03N 
// 0.019N required for verify 12-word remembered. 
const ACCESS_KEY_ALLOWANCE: u128 = 30_000_000_000_000_000_000_000;  // 0.01N
const NO_DEPOSIT: u128 = 0;

pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: Gas = Gas(30_000_000_000_000);


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LinkDrop {
    pub accounts: LookupMap<PublicKey, Balance>,
}

impl Default for LinkDrop {
  fn default() -> Self {
    Self {
      accounts: LookupMap::new(b"d".to_vec()),
    }
  }
}


#[ext_contract(ext_self)]
pub trait ExtLinkDrop {
    fn on_account_created(
      &mut self,
      predecessor_account_id: AccountId,
      amount: U128
    ) -> bool;

    fn on_account_created_and_claimed(
      &mut self, 
      amount: U128
    ) -> bool;
}



#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, PublicKey, BlockHeight};

    pub struct VMContextBuilder {
        context: VMContext,
    }

    impl VMContextBuilder {
      pub fn new() -> Self {
          Self {
              context: VMContext {
                  current_account_id: "".to_owned(),
                  signer_account_id: "".to_owned(),
                  signer_account_pk: vec![],
                  predecessor_account_id: "".to_owned(),
                  input: vec![],
                  block_index: 0,
                  epoch_height: 0,
                  block_timestamp: 0,
                  account_balance: 0,
                  account_locked_balance: 0,
                  storage_usage: 10u64.pow(6),
                  attached_deposit: 0,
                  prepaid_gas: 10u64.pow(18),
                  random_seed: vec![0, 1, 2],
                  is_view: false,
                  output_data_receivers: vec![],
              },
          }
      }
      
      // These String requires changing to AccountId in the future. 
      pub fn current_account_id(mut self, account_id: AccountId) -> Self {
          self.context.current_account_id = account_id.to_string();
          self
      }

      #[allow(dead_code)]
      pub fn signer_account_id(mut self, account_id: AccountId) -> Self {
          self.context.signer_account_id = account_id.to_string();
          self
      }

      pub fn predecessor_account_id(mut self, account_id: AccountId) -> Self {
          self.context.predecessor_account_id = account_id.to_string();
          self
      }

      #[allow(dead_code)]
      pub fn block_index(mut self, block_index: BlockHeight) -> Self {
          self.context.block_index = block_index;
          self
      }

      pub fn attached_deposit(mut self, amount: Balance) -> Self {
          self.context.attached_deposit = amount;
          self
      }

      pub fn account_balance(mut self, amount: Balance) -> Self {
          self.context.account_balance = amount;
          self
      }

      #[allow(dead_code)]
      pub fn account_locked_balance(mut self, amount: Balance) -> Self {
          self.context.account_locked_balance = amount;
          self
      }

      pub fn signer_account_pk(mut self, pk: PublicKey) -> Self {
          self.context.signer_account_pk = pk.into_bytes();
          self
      }

      pub fn finish(self) -> VMContext {
          self.context
      }
    }

    // These requires return AccountId in the future. For now no, because we're 
    // changing string to ACcountId and change back to string, which is inefficient
    // during testing phase. 
    fn linkdrop() -> AccountId {
        "linkdrop".parse().unwrap()
    }

    fn bob() -> AccountId {
        "bob.near".parse().unwrap()
    }

    
    #[test]
    fn test_create_account() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
            let deposit: u128 = 1_000_000;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .predecessor_account_id(linkdrop())
            .signer_account_pk(pk.clone())
            .attached_deposit(deposit)
            .finish());
        // contract.create_account(bob(), pk);
        contract.create_account_and_claim(bob(), pk);
        // TODO: verify that promise was created with funds for given username.
    }

    #[test]
    #[should_panic]
    fn test_create_invalid_account() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
        let deposit: u128 = 1_000_000;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.create_account_and_claim("XYZ".parse().unwrap(), pk);
    }

    #[test]
    #[should_panic]
    fn test_get_missing_balance_panics() {
        let contract = LinkDrop::default();
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .finish());
        contract.get_key_balance("ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
                                  .parse().unwrap());
    }

    #[test]
    fn test_get_missing_balance_success() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
        let deposit = ACCESS_KEY_ALLOWANCE * 100;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.send(pk.clone());
        // try getting the balance of the key
        let balance:u128 = contract.get_key_balance(pk.try_into().unwrap()).try_into().unwrap();
        assert_eq!(
            balance,
            deposit - ACCESS_KEY_ALLOWANCE
        );
    }

    #[test]
    #[should_panic]
    fn test_claim_invalid_account() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
        // Deposit money to linkdrop contract.
        let deposit = ACCESS_KEY_ALLOWANCE * 100;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.send(pk.clone());
        // Now, send new transaction to link drop contract.
        let context = VMContextBuilder::new()
            .current_account_id(linkdrop())
            .predecessor_account_id(linkdrop())
            .signer_account_pk(pk)
            .account_balance(deposit)
            .finish();
        testing_env!(context);
        let pk2 = "ed25519:2S87aQ1PM9o6eBcEXnTR5yBAVRTiNmvj8J8ngZ6FzSca"
            .parse()
            .unwrap();
        contract.create_account_and_claim("XYZ".parse().unwrap(), pk2);
    }

    #[test]
    #[should_panic(expect = "Attached deposit 0 must be greater than 30000000000000000000000")]
    fn test_drop_claim_failed_panic() {
      let mut contract = LinkDrop::default();
      let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
          .parse()
          .unwrap();
      // Deposit money to linkdrop contract.
      let deposit = 0;
      testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.send(pk.clone());
    }

    #[test]
    fn test_drop_claim() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
        // Deposit money to linkdrop contract.
        let deposit = ACCESS_KEY_ALLOWANCE * 100;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.send(pk.clone());
        // Now, send new transaction to link drop contract.
        let context = VMContextBuilder::new()
            .current_account_id(linkdrop())
            .predecessor_account_id(linkdrop())
            .signer_account_pk(pk.into())
            .account_balance(deposit)
            .finish();
        testing_env!(context);
        let pk2 = "ed25519:2S87aQ1PM9o6eBcEXnTR5yBAVRTiNmvj8J8ngZ6FzSca"
            .parse()
            .unwrap();
        contract.create_account_and_claim(bob(), pk2);
        // TODO: verify that proper promises were created.
    }

    #[test]
    fn test_send_two_times() {
        let mut contract = LinkDrop::default();
        let pk: PublicKey = "ed25519:qSq3LoufLvTCTNGC3LJePMDGrok8dHMQ5A1YD9psbiz"
            .parse()
            .unwrap();
        // Deposit money to linkdrop contract.
        let deposit = ACCESS_KEY_ALLOWANCE * 100;
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .attached_deposit(deposit)
            .finish());
        contract.send(pk.clone());
        assert_eq!(contract.get_key_balance(pk.clone()), (deposit - ACCESS_KEY_ALLOWANCE).into());
        testing_env!(VMContextBuilder::new()
            .current_account_id(linkdrop())
            .account_balance(deposit)
            .attached_deposit(deposit + 1)
            .finish());
        contract.send(pk.clone());
        assert_eq!(
            contract.accounts.get(&pk.into()).unwrap(),
            deposit + deposit + 1 - 2 * ACCESS_KEY_ALLOWANCE
        );
    }
}
