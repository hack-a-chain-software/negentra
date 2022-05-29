use crate::errors::{ERR_401, ERR_402};
use crate::utils::{split_investment_id};
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

//blockchain exposed

#[near_bindgen]
impl Contract {
    pub fn view_schema(&self, schema_category: String) -> crate::schema::Schema {
        self.schemas.get(&schema_category).expect(ERR_002)
    }

    pub fn view_all_schemas(&self) -> Vec<String> {
        self.schemas.keys().collect()
    }

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
}

// pub account: AccountId,
//     pub total_value: u128, // all the tokens avalible for this investor -  wont change
//     pub withdrawn_value: u128, // the amount of tokens that the user already withdrew - will change
//     pub date_in:
