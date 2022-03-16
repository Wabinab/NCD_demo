use near_sdk::{env, PromiseResult, require};


pub(crate) fn is_promise_success() -> bool {
  require!(
    env::promise_results_count() == 1,
    "Contract expected a result on the callback."
  );

  match env::promise_result(0) {
    PromiseResult::Successful(_) => true,
    _ => false,
  }
}


pub(crate) fn expect_lightweight<T>(
  option: Option<T>,
  message: &str,
) -> T {
  option.unwrap_or_else(|| env::panic_str(message))
}


pub(crate) fn assert_predecessor_is_current(message: &str) {
  require!(
    env::predecessor_account_id() == env::current_account_id(),
    message
  )
}