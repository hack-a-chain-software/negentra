use crate::*;
use near_sdk::{env, near_bindgen, PromiseResult};

#[near_bindgen]
impl Contract {
    #[private]
    pub fn undo_transfer(&mut self, value_to_return: U128, investment_id: String) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_val) => {}
            PromiseResult::Failed => {
                self.undo_withdraw_investment(investment_id, value_to_return.0)
            }
        }
    }
}
