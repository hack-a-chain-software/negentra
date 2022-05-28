use crate::errors::ERR_201;
use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Investment {
    pub account: AccountId,
    pub total_value: u128,
    pub withdrawn_value: u128,
    pub date_in: Option<u64>, // Option - None: uses Schema date
}

impl Investment {
    pub fn new(account: AccountId, total_value: u128, date_in: Option<u64>) -> Self {
        Self {
            account,
            total_value,
            withdrawn_value: 0,
            date_in,
        }
    }

    pub fn increase_withdraw_value(&mut self, value_to_withdraw: u128) {
        assert!(
            (self.withdrawn_value + value_to_withdraw) <= self.total_value,
            "{}",
            ERR_201
        );

        self.withdrawn_value += value_to_withdraw;
    }
}

// new_investment → init investment, must deposit their tokens on
// initialization (INTERNAL, USES FT ON TRANSFER), Create the investment id using the SchemaId+AccountId
//view_investment_data(id) → returns all data, and the available withdraw
