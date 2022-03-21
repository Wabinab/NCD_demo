use crate::*;
use near_sdk::utils::{assert_one_yocto};



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


// ======================PAYOUT FEATURE=====================//

pub trait GeneratePayout {
  fn do_payout(
    &mut self,
    article_id: ArticleId,
  ) -> Payout;

  // actually payout function we'll do later. 
}



#[near_bindgen]
impl GeneratePayout for Contract {

    #[payable]
    #[result_serializer(borsh)]  // This one. 
    fn do_payout(
      &mut self, 
      article_id: ArticleId, 
    ) -> Payout {
      let amount: u128 = env::attached_deposit();

      let error_message = format!(
        "The tip you send is less than we can handle. Min: {} NEAR.",
        yoctonear_to_near(MIN_TIPPING_AMOUNT)
      );
      let error_message: &str = error_message.as_str();

      require!(
        amount >= MIN_TIPPING_AMOUNT, 
        error_message,
      );

      let article = expect_lightweight(
        self.article_by_id.get(&article_id),
        "Article not found, either it's not yet created or error. "
      );


      let royalty = article.royalty;
      let owner_id = article.owner_id;

      for account in royalty.keys() {
        require!(
          env::is_valid_account_id(account.as_bytes())
        )
      }
      require!(
        env::is_valid_account_id(owner_id.as_bytes())
      );
      

      let mut payout_object = Payout {
        payout: HashMap::new()
      };

      // First, pay the owner, regardless of events. 
      payout_object.payout.insert(owner_id, U128(royalty_to_payout(9_000u16, amount)));

      let mut current_left_royalty: u16 = 1_000u16;

      for (k, v) in royalty.iter() {
        current_left_royalty -= v;
        let payout_amount = royalty_to_payout(v.clone(), amount);

        if payout_amount > MIN_TO_BE_PAYED {
          payout_object.payout.insert(k.clone(), U128(payout_amount));
        } else {
          // All will go to me as I'm GREEDY! 
          current_left_royalty += v;
        }
      }

      // We don't check this because MIN_TIPPING_AMOUNT restrict it to be payable by being 10x
      // of MIN_TO_BE_PAYED. 
      payout_object.payout.insert(
        env::current_account_id(), 
        U128(royalty_to_payout(current_left_royalty, amount))
      );

      payout_object

      // use Promise + callbacks to "multisend" the payments. 
    }
}
