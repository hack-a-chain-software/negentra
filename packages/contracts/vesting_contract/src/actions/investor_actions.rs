use crate::errors::{ERR_401, ERR_402};
use crate::utils::{create_investment_id, split_investment_id};
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

//blockchain exposed

#[near_bindgen]
impl Contract {
    
}

// pub account: AccountId,
//     pub total_value: u128, // all the tokens avalible for this investor -  wont change
//     pub withdrawn_value: u128, // the amount of tokens that the user already withdrew - will change
//     pub date_in:
