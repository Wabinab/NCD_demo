use crate::*;


// This function supposedly be in internal.rs, but we're not using
// it anywhere so let's just put it here. 
/// Converts royalty percentage and amount to pay into payout. 
pub(crate) fn royalty_to_payout(
  royalty_percentage: u32,
  amount_to_pay: Balance
) -> U128 {
  U128(
    royalty_percentage as u128
    * amount_to_pay
    / 10_000u128  // 2.d.p
  )
}