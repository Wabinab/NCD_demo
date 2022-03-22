use crate::*;
use near_sdk::Gas;
use near_sdk::utils::{is_promise_success};


/// Callback gas
pub const CALLBACK: Gas = Gas(25_000_000_000_000);


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
  fn calculate_payout(
    &mut self,
    article_id: ArticleId,
  ) -> Payout;

  // actually payout function we'll do later. 
  fn send_payout(
    &mut self,
    article_id: ArticleId
  );

  /// callbacks for send payout
  fn on_transfer_attached_tokens(
    &mut self,
    sender_id: AccountId,
    amount_sent: U128,
    recipient: AccountId,
  );


  /// callbacks for refund
  fn on_refund(&mut self);
}



#[near_bindgen]
impl GeneratePayout for Contract {

    #[result_serializer(borsh)]  // This one. 
    fn calculate_payout(
      &mut self, 
      article_id: ArticleId, 
    ) -> Payout {
      let amount: u128 = env::attached_deposit();

      // cache error message to &str type from String type. 
      let error_message = format!(
        "The tip you send is less than we can handle. Min: {} NEAR.",
        yoctonear_to_near(MIN_TIPPING_AMOUNT)
      );
      let error_message: &str = error_message.as_str();
      // Assertions to attached enough money to continue. 
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
      

      let mut payout_object = Payout::default();

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
    }


    #[result_serializer(borsh)]
    #[payable]
    fn send_payout(
      &mut self,
      article_id: ArticleId
    ) {
      // repetitive, think how to remove this repetition later. 
      // by passing article to calculate_payout instead of article_id. 
      let article = expect_lightweight(
        self.article_by_id.get(&article_id),
        "Article not found, either it's not yet created or error. "
      );
      assert_signer_not_owner(article.owner_id);


      let payout_object = self.calculate_payout(article_id);

      for (account, amount) in payout_object.payout.iter() {
        let amount_u128: u128 = amount.clone().into();

        env::log_str(
          format!(
            "Sending {} yNEAR (~{} NEAR) to account @{}",
            amount_u128,
            yoctonear_to_near(amount_u128),
            account.clone()
          ).as_str()
        );

        Promise::new(account.clone())
            .transfer(amount_u128)
            .then(
              ext_self::on_transfer_attached_tokens(
                env::signer_account_id(),
                amount.clone(),
                account.clone(),
                env::current_account_id(),
                0,
                CALLBACK
              )
            );

        // If there are refund, refund. If no, continue on. 
        let refund: Balance = self.get_refund(env::signer_account_id()).into();
        if refund > 0u128 {
          Promise::new(env::signer_account_id())
              .transfer(refund)
              .then(
                ext_self::on_refund(
                  env::current_account_id(),
                  0,
                  CALLBACK
                )
              );
        }
      }
    }


    fn on_transfer_attached_tokens(
      &mut self,
      sender_id: AccountId,
      amount_sent: U128,
      recipient: AccountId,
    ) {
      assert_predecessor_is_current("Can only be called by contract.");

      let transfer_succeeded = is_promise_success();
      if !transfer_succeeded {
        env::log_str(
          format!(
            "Transaction to @{} failed. {} yNEAR (~{} NEAR) will be refunded.",
            recipient,
            amount_sent.0,
            yoctonear_to_near(amount_sent.0)
          ).as_str(),
        );

        let previous_balance: Balance = self.get_refund(sender_id.clone()).into();
        self.refund.insert(sender_id, previous_balance + amount_sent.0);
      }
    }


    fn on_refund(
      &mut self 
    ) {
      assert_predecessor_is_current("Can only be called by contract.");

      let transfer_succeeded = is_promise_success();

      if !transfer_succeeded {
        env::log_str("Please contact support!");
      } else {
        env::log_str("Refund success. Cleaning up!");
        self.refund.remove(&env::signer_account_id());
      }
    }
}
