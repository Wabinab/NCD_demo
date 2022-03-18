use near_sdk::json_types::{U128};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
  near_bindgen, ext_contract, AccountId, Balance, 
  PublicKey, Gas, PanicOnDefault, env
};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::serde::{Serialize};
use near_helper::*;


pub mod tipping;
pub mod article;

use crate::tipping::*;
use crate::article::*;


pub type ArticleId = String;


/// Helper structure for keys of persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    RoyaltyKey,
    PayoutKey,
    ArticleKey
}


// Vanilla sending near directly from wallet cost 0.00008N
// transaction fee, so we must send more than that, let's 
// set the limit to 0.0001N, otherwise not eligible to
// receive your share. Indeed, someone else will receive
// your share instead. (not very fair but whatever I don't care)
const MIN_TO_BE_PAYED: u128 = 100_000_000_000_000_000_000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// contract owner
    pub owner_id: AccountId,

    /// Map an ArticleId to the Article object/struct. 
    pub article_by_id: LookupMap<ArticleId, Article>,

    // We don't save metadata or others to save contract 
    // space. 
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
      Self {
        owner_id, 
        article_by_id: LookupMap::new(
          StorageKey::ArticleKey.try_to_vec().unwrap()
        ),
      }
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
#[derive(BorshDeserialize, BorshSerialize)]
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
    pub royalty: UnorderedMap<AccountId, u16>,
}

impl Default for Article {
    fn default() -> Self {
      Self {
        owner_id: env::current_account_id(),
        article_id: "".to_owned(),
        royalty: UnorderedMap::new(
          StorageKey::RoyaltyKey.try_to_vec().unwrap()
        ),
      }
    }
}


// ======================================= PAYOUT =============================== //

// MAYBE INCLUDE #[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Payout {
    pub payout: UnorderedMap<AccountId, U128>,
}

impl Default for Payout {
    fn default() -> Self {
      Self {
        payout: UnorderedMap::new(
          StorageKey::PayoutKey.try_to_vec().unwrap()
        ),
      }
    }
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


    fn default_context() -> VMContext {
      VMContextBuilder::new()
          .current_account_id(tipping())
          .signer_account_id(bob())
          .build()
    }


    #[test]
    fn test_create_new_articles_default() {
      testing_env!(default_context());

      let mut contract = Contract::new(tipping());
      contract.add_new_article_default(bob(), 1);

      assert_eq!(
        contract.owner_id, 
        tipping()
      );

      let g = contract.article_by_id
          .get(&"bob.near1".to_owned())
          .expect("article_id changed implementation?");

      // Royalty assertions. 
      let mut count = 0;

      for (k, v) in g.royalty.iter() {
        count += 1;
        eprintln!("{}", k.clone());
        if k == bob() {
          assert_eq!(v, 9000);
        } else {
          assert_eq!(v, 1000);
        }
      }

      assert_eq!(count, 2, "Royalty payout incorrect length");
    }


    #[test]
    fn test_create_multiple_articles_single_account() {
      testing_env!(default_context());

      let mut contract = Contract::new(tipping());
      contract.add_new_article_default(bob(), 1);
      contract.add_new_article_default(bob(), 3);

      let g = contract.article_by_id;
      assert!(g.get(&"bob.near1".to_owned()).is_some());
      assert!(g.get(&"bob.near2".to_owned()).is_none());
      assert!(g.get(&"bob.near3".to_owned()).is_some());
      // assert!(contract.article_by_id.contains_key(&"bob.near3".to_owned()));
      
    }

}