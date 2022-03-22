use near_sdk::json_types::{U128};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
  near_bindgen, ext_contract, AccountId, Balance, 
  PublicKey, Gas, PanicOnDefault, env, require, 
  Promise
};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::serde::{Serialize};
use near_helper::{yoctonear_to_near, assert_predecessor_is_current, 
  expect_lightweight
};

use std::collections::HashMap;


pub mod tipping;
pub mod article;
pub mod internal;

use crate::tipping::*;
use crate::article::*;
use crate::internal::*;


pub type ArticleId = String;


/// Helper structure for keys of persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    RoyaltyKey,
    PayoutKey,
    ArticleKey
}


/// Vanilla sending near directly from wallet cost 0.00008N
/// transaction fee, so we must send more than that, let's 
/// set the limit to 0.0001N, otherwise not eligible to
/// receive your share. Indeed, someone else will receive
/// your share instead. (not very fair but whatever I don't care)
const MIN_TO_BE_PAYED: u128 = 50_000_000_000_000_000_000;

/// Minimum tip amount is 1e-3 NEAR. 
const MIN_TIPPING_AMOUNT: u128 = 1_000_000_000_000_000_000_000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// contract owner
    pub owner_id: AccountId,

    /// Map an ArticleId to the Article object/struct. 
    pub article_by_id: LookupMap<ArticleId, Article>,

    // We don't save metadata or others to save contract 
    // space. 

    /// Refund to owner, a temporary variable that deletes entry
    /// after refunding. 
    pub refund: HashMap<AccountId, Balance>,
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
      Self {
        owner_id,  // maybe use env::current_account_id()? 
        article_by_id: LookupMap::new(
          StorageKey::ArticleKey.try_to_vec().unwrap()
        ),
        refund: HashMap::new(),
      }
    }


    pub fn get_refund(&self, sender_id: AccountId) -> U128 {
      match self.refund.get(&sender_id) {
        Some(deposit) => U128::from(*deposit),
        None => 0.into()
      }
    }


    pub fn get_article_by_id(&self, article_id: ArticleId) -> String {
      let article = expect_lightweight(
        self.article_by_id.get(&article_id),
        "Cannot find article"
      );
      format!("{:#?}",article)
    }
}


pub trait ContractCore {
    fn fetch_owner(&self) -> AccountId;
}


#[near_bindgen]
impl ContractCore for Contract {
    fn fetch_owner(&self) -> AccountId {
      self.owner_id.clone()
    }
}


// ========================== ARTICLE ============================== //


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Article {
    /// article owner. DO NOT CONFUSE with Contract's owner id. 
    pub owner_id: AccountId,

    /// Owner_name + number. 
    /// E.g. Wabinab1, Wabinab29, Wabinab3973. 
    /// No Wabinab0001 as it restrict to 9999 stupidly. 
    pub article_id: ArticleId,

    /// Royalty be 2 decimal places. 
    /// But we use integer, so max is 10000 (= 100.00%). 
    /// E.g. 2985 = 29.85%. 
    pub royalty: HashMap<AccountId, u16>,
}

impl Default for Article {
    fn default() -> Self {
      Self {
        owner_id: env::current_account_id(),
        article_id: "".to_owned(),
        royalty: HashMap::new(),
      }
    }
}


// ======================================= PAYOUT =============================== //

// #[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}


impl Default for Payout {
    fn default() -> Self {
      Self {
        payout: HashMap::new(),
      }
    }
}


// ==================================== CALLBACKS =============================== //

#[ext_contract(ext_self)]
pub trait ExtMultisender {
    fn on_transfer_attached_tokens(
      &mut self,
      sender_id: AccountId,
      amount_sent: U128,
      recipient: AccountId,
    );

    fn on_refund(&mut self);
}


// ==================================== TESTS ============================= //


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, PublicKey};
    use std::convert::TryInto;


    fn alice() -> AccountId {
      "alice.near".parse().unwrap()
    }


    fn bob() -> AccountId {
      "bob.near".parse().unwrap()
    }


    fn tipping() -> AccountId {
      "tipping.near".parse().unwrap()
    }


    fn default_context(deposit: u128) -> VMContext {
      VMContextBuilder::new()
          .current_account_id(tipping())
          .signer_account_id(bob())
          .attached_deposit(deposit)
          .build()
    }

    
    fn contract_add_new_article_default() -> Contract {
      let mut contract = Contract::new(tipping());
      contract.add_new_article_default(1);

      contract
    }


    #[test]
    fn test_create_new_articles_default() {
      testing_env!(default_context(0));

      let contract = contract_add_new_article_default();

      assert_eq!(
        contract.owner_id, 
        tipping()
      );

      let article1 = contract.article_by_id
          .get(&"bob.near1".to_owned())
          .expect("article_id changed implementation?");

      // Royalty assertions. 
      assert_eq!(article1.royalty.len(), 0, "Royalty payout incorrect length");
    }


    #[test]
    fn test_create_multiple_articles_single_account() {
      testing_env!(default_context(0));

      let mut contract = Contract::new(tipping());
      contract.add_new_article_default(1);
      contract.add_new_article_default(3);

      let g = contract.article_by_id;
      assert!(g.get(&"bob.near1".to_owned()).is_some());
      assert!(g.get(&"bob.near2".to_owned()).is_none());
      assert!(g.get(&"bob.near3".to_owned()).is_some());
      // assert!(contract.article_by_id.contains_key(&"bob.near3".to_owned()));
      
    }


    #[test]
    fn test_default_article_payout_correct_amount() {
      let deposit: u128 = 1_000_000_000_000_000_000_000_000;  // 1 NEAR
      testing_env!(default_context(deposit));

      let mut contract = contract_add_new_article_default();

      let payout_object = contract.calculate_payout("bob.near1".to_owned());

      assert_eq!(
        payout_object.payout, 
        HashMap::from([
          (bob(), U128(900_000_000_000_000_000_000_000)),
          (tipping(), U128(100_000_000_000_000_000_000_000)),
        ]),
      );
    }


    #[test]
    #[should_panic]
    fn test_default_article_panic_if_not_enough_deposit() {
      let deposit: u128 = 100_000;  // far less than 1 NEAR
      assert!(
        deposit < MIN_TIPPING_AMOUNT, 
        "Our mock deposit is larger than MIN_TIPPING_AMOUNT"
      );
      testing_env!(default_context(deposit));

      let mut contract = contract_add_new_article_default();

      contract.calculate_payout("bob.near1".to_owned());
    }


    #[test]
    #[should_panic]
    fn test_tipping_yourself_failed() {
      let deposit: u128 = MIN_TIPPING_AMOUNT;
      testing_env!(default_context(deposit));

      let mut contract = contract_add_new_article_default();

      contract.send_payout("bob.near1".to_owned());
    }

}