use crate::errors::{ERR_401, ERR_402, ERR_403};
use crate::ext_interface::*;
use crate::utils::{create_investment_id, split_investment_id};
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen, utils::assert_one_yocto, Promise};
use std::collections::HashMap;

//blockchain exposed

#[near_bindgen]
impl Contract {
    pub fn view_investment(&self, investment_id: AccountId) -> HashMap<String, String> {
        let investment = self.investments.get(&investment_id).expect(ERR_401);
        let available_withdraw =
            self.calculate_available_withdraw(env::block_timestamp(), investment_id.clone());

        let schema_id = split_investment_id(investment_id.clone()).remove(0);
        let schema = self.schemas.get(&schema_id).expect(ERR_402);

        let initial_timestamp: u64;

        if let Some(time_stamp) = investment.date_in {
            initial_timestamp = time_stamp;
        } else {
            initial_timestamp = schema.initial_timestamp;
        }

        let mut invest_data = HashMap::new();

        invest_data.insert("investment_id".to_string(), investment_id);
        invest_data.insert(
            "withdrawn_value".to_string(),
            investment.withdrawn_value.to_string(),
        );
        invest_data.insert("init. date".to_string(), initial_timestamp.to_string());
        invest_data.insert(
            "available_withdraw".to_string(),
            available_withdraw.to_string(),
        );
        invest_data
    }

    #[payable]
    pub fn withdraw_your_investments(
        &mut self,
        value_to_withdraw: U128,
        category: String,
    ) -> Promise {
        assert_one_yocto();
        assert!(env::prepaid_gas() >= BASE_GAS * 3, "{}", ERR_403);
        let predecessor = env::predecessor_account_id();
        let now = env::block_timestamp();

        let investment_id = create_investment_id(category.clone(), predecessor.clone());
        self.withdraw_investment(now, investment_id.clone(), value_to_withdraw.0);

        token_contract::ft_transfer(
            predecessor.clone(),
            value_to_withdraw,
            "Vesting withdraw".to_string(),
            &self.token_contract,
            1,
            BASE_GAS,
        )
        .then(ext_self::undo_transfer(
            value_to_withdraw,
            investment_id,
            &env::current_account_id(),
            0,
            BASE_GAS,
        ))
    }
}
