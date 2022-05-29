use crate::errors::{ERR_403};
use crate::ext_interface::*;
use crate::utils::{create_investment_id};
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen, utils::assert_one_yocto, Promise};


//blockchain exposed

#[near_bindgen]
impl Contract {
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
