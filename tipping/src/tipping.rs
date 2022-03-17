use crate::*;


// This function supposedly be in internal.rs, but we're not using
// it anywhere so let's just put it here. 
/// Converts royalty percentage and amount to pay into payout. 
pub(crate) fn royalty_to_payout(
  royalty_percentage: u16,
  amount_to_pay: Balance
) -> u128 {
      royalty_percentage as u128
      * amount_to_pay
      / 10_000u128  // 2.d.p
}


// ================== DEFAULT ROYALTY =====================//


pub(crate) fn default_royalty(
  owner_id: AccountId,
) -> UnorderedMap<AccountId, u16> {
    let mut payout = UnorderedMap::new(
      StorageKey::RoyaltyKey.try_to_vec().unwrap()
    );
    
    // 90% to owner
    payout.insert(&owner_id, &9_000u16);
    
    // 10% to me (is this correct? Predecessor or current?)
    payout.insert(&env::predecessor_account_id(), &1_000u16);

    payout
}


// ======================PAYOUT FEATURE=====================//

pub trait GeneratePayout {
  fn calculate_payout(
    &mut self,
    article_id: ArticleId,
    amount: U128,
  ) -> Payout;

  // actually payout function we'll do later. 
}



#[near_bindgen]
impl GeneratePayout for Contract {

    fn calculate_payout(
      &mut self, 
      article_id: ArticleId, 
      amount: U128
    ) -> Payout {
      let amount_float = amount.into();

      let article = expect_lightweight(
        self.article_by_id.get(&article_id),
        "Article not found"
      );
      let royalty = article.royalty;

      let mut payout_object = Payout {
        payout: UnorderedMap::new(
          StorageKey::PayoutKey.try_to_vec().unwrap()
        ),
      };

      let mut unpayable: u128 = 0u128;

      for (k, v) in royalty.iter() {
        let payout_amount = royalty_to_payout(v, amount_float);

        if payout_amount > MIN_TO_BE_PAYED {
          payout_object.payout.insert(&k, &U128(payout_amount));
        } else {
          // All will go to me as I'm GREEDY! 
          unpayable += payout_amount;
        }
      }

      let me: U128 = expect_lightweight(
        payout_object.payout.get(
          &env::predecessor_account_id()
        ),
        format!(
          "{} does not exist in payout object",
          env::predecessor_account_id()
        ).as_str(),
      );
      
      payout_object.payout.insert(
        &env::predecessor_account_id(), 
        &U128(u128::from(me) + unpayable)
      );

      payout_object
    }
}
