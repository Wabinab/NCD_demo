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
use crate::tipping::*;


pub type ArticleId = String;


/// Helper structure for keys of persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    RoyaltyKey,
    PayoutKey
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
        owner_id: AccountId::new_unchecked(String::from("")),
        article_id: "".to_owned(),
        royalty: UnorderedMap::new(
          StorageKey::RoyaltyKey.try_to_vec().unwrap()
        ),
      }
    }
}

impl Article {
    fn new(owner_id: AccountId, article_number: u64) -> Self {
      Self {
        // as we need owner_id.clone(), we move to first place. 
        royalty: default_royalty(owner_id.clone()),
        article_id: format!("{}{}", owner_id.clone(), article_number), 
        owner_id,
      }
    }
}



#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
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
