use near_sdk::{PublicKey, Promise, Balance, env};
use crate::*;

const SEND_DEPOSIT: Balance = 1_100_000_000_000_000_000_000_000;  // 1.1N

#[near_bindgen]
impl Signup {
    pub fn send(&mut self, public_key: PublicKey) {
      ext_signup::on_send(
        public_key,
        env::current_account_id(),
        NO_DEPOSIT,
        GAS
      );
    }

    pub fn on_send(&mut self, public_key: PublicKey) {
      ext_linkdrop::send(
        public_key,
        "testnet".parse().unwrap(),
        SEND_DEPOSIT,
        GAS
      );
    }

    pub fn get_key_balance(&mut self, public_key: PublicKey) -> Promise {
      ext_linkdrop::get_key_balance(
        public_key,
        "testnet".parse().unwrap(),
        NO_DEPOSIT,
        GAS
      )
    }


}