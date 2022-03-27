use crate::*;
use near_sdk::Gas;
use near_sdk::utils::{is_promise_success};


/// Callback gas
pub const CALLBACK: Gas = Gas(25_000_000_000_000);


// This function supposedly be in internal.rs, but we're not using
// it anywhere so let's just put it here. 
// /// Converts royalty percentage and amount to pay into payout. 
// pub(crate) fn royalty_to_payout(
//   royalty_percentage: u16,
//   amount_to_pay: Balance
// ) -> u128 {
//       royalty_percentage as u128
//       * amount_to_pay
//       / 10_000u128  // 2.d.p
// }


/// Cheap version of converting royalty percentage and amount to payout. 
pub(crate) fn royalty_to_payout_cheap(
  royalty_percentage: u16, 
  amount_to_pay: u32,
  power: u8,
) -> u128 {
    let mut value = (royalty_percentage as u32
    * amount_to_pay
    / 10_000u32).to_string();

    for _ in 0..power {
      value.push_str("0");
    }

    value.parse().unwrap()
}


// pub(crate) fn calculate_min(power: u8) -> u32 {
//     let differences = power - MIN_TO_BE_PAYED_POWER;
//     if differences > 0 {
//       0u32
//     } else if differences == 0 {
//       MIN_TO_BE_PAYED_NUM.parse().unwrap()
//     } else {
//       let mut num_str: String = MIN_TO_BE_PAYED_NUM.to_owned();
//       for i in 0..differences {
//         num_str.push_str("0");
//       }
//       num_str.parse().unwrap()
//     }
// }


// ======================PAYOUT FEATURE=====================//

pub trait GeneratePayout {
  fn calculate_payout(
    &mut self,
    article_id: ArticleId,
  ) -> PayoutType;

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
    ) -> PayoutType {
      let amount: u128 = env::attached_deposit();
     
      // Assertions to attached enough money to continue. 
      require!(
        amount >= MIN_TIPPING_AMOUNT, 
        ERR_MSG,
      );

      let article = expect_lightweight(
        self.article_by_id.get(&article_id),
        "Article not found, either it's not yet created or error. "
      );

      let royalty = article.royalty;
      let owner_id = article.owner_id;
      assert_signer_not_owner(owner_id.clone());

      for account in royalty.keys() {
        require!(
          env::is_valid_account_id(account.as_bytes())
        )
      }
      require!(
        env::is_valid_account_id(owner_id.as_bytes())
      );
      

      let mut payout_hashmap: PayoutType = HashMap::new();
      
      let (amount_digits, power) = as_scientific_notation(amount, 4);
      // let min_to_be_payed = calculate_min(power);
      payout_hashmap.insert(
        owner_id, 
        U128(royalty_to_payout_cheap(9000u16, amount_digits, power))
      );

      let mut current_left_royalty: u16 = 1_000u16;

      for (k, v) in royalty.iter() {
        current_left_royalty -= v;
        let payout_amount = royalty_to_payout_cheap(v.clone(), amount_digits, power);

        if payout_amount > MIN_TO_BE_PAYED {
          payout_hashmap.insert(k.clone(), U128(payout_amount));
        } else {
          // All will go to me as I'm GREEDY! 
          current_left_royalty += v;
        }
      }

      // We don't check this because MIN_TIPPING_AMOUNT restrict it to be payable by being 10x
      // of MIN_TO_BE_PAYED. 
      payout_hashmap.insert(
        env::current_account_id(), 
        U128(royalty_to_payout_cheap(current_left_royalty, amount_digits, power))
      );

      payout_hashmap
    }


    #[result_serializer(borsh)]
    #[payable]
    fn send_payout(
      &mut self,
      article_id: ArticleId
    ) {
      let payout_object = self.calculate_payout(article_id);
      let mut logs: String = "".to_owned();

      for (account, amount) in payout_object.iter() {
        let account = account.clone();
        let amount_u128: u128 = amount.clone().into();

        let log = format!(
          "Sending {} yNEAR (~{} NEAR) to account @{}",
          amount_u128,
          yoctonear_to_near(amount_u128),
          account
        );
        logs.push_str(&log);

        Promise::new(account.clone())
            .transfer(amount_u128)
            .then(
              ext_self::on_transfer_attached_tokens(
                env::signer_account_id(),
                amount.clone(),
                account,
                env::current_account_id(),
                0,
                CALLBACK
              )
            );

        env::log_str(format!("{}\nDone!", logs).as_str());

        // If there are refund, refund. If no, continue on. 
        let owner_id = env::signer_account_id();
        let refund: Balance = self.get_refund(owner_id.clone()).into();
        if refund > 0u128 {
          env::log_str(
            format!(
              "Returning {} to @{}",
              refund.clone(),
              owner_id
            ).as_str()
          );
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


    fn on_refund(&mut self ) {
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