use crate::errors::{ERR_301, ERR_302, ERR_303, ERR_304};
use crate::ext_interface::*;
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen, utils::assert_one_yocto, Promise};

//blockchain exposed

#[near_bindgen]
impl Contract {
    // ft on transfer is an onwner action, but it is called by the token contract
    pub fn ft_on_transfer(&mut self, sender_id: String, amount: U128, msg: String) -> U128 {
        // validate the predecessor is the token contract
        assert_eq!(
            env::predecessor_account_id(),
            self.token_contract,
            "{}",
            ERR_302
        );

        //validate that the sender is the contract owner
        assert_eq!(sender_id, self.owner, "{}", ERR_301);

        let parsed_message: Value = serde_json::from_str(&msg).expect(ERR_303);

        let category: String = parsed_message["category"]
            .as_str()
            .expect(ERR_303)
            .to_string();

        let initial_release: u128 = parsed_message["initial_release"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let cliff_release: u128 = parsed_message["cliff_release"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let final_release: u128 = parsed_message["final_release"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let initial_timestamp: u64 = parsed_message["initial_timestamp"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let cliff_delta: u64 = parsed_message["cliff_delta"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let final_delta: u64 = parsed_message["final_delta"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let curve_type: String = parsed_message["curve_type"]
            .as_str()
            .expect(ERR_303)
            .to_string();

        let discrete_period: u64 = parsed_message["discrete_period"]
            .as_str()
            .expect(ERR_303)
            .parse()
            .expect(ERR_303);

        let curve;
        if curve_type == "Linear" {
            curve = CurveType::Linear { discrete_period };
        } else {
            panic!("Curve tyoe not supported. Currently, only curve type available is: 'Linear'");
        }

        self.new_schema(
            category,
            amount.0,
            initial_release,
            cliff_release,
            final_release,
            initial_timestamp,
            cliff_delta,
            final_delta,
            curve,
        );

        U128(0)
    }

    pub fn create_investment(
        &mut self,
        category: String,
        account: AccountId,
        total_value: U128,
        date_in: Option<U64>,
    ) {
        //validate that the sender is the contract owner
        self.only_owner();
        self.new_investment(category, account, total_value, date_in);
    }

    #[payable]
    pub fn owner_withdraw_investments(
        &mut self,
        value_to_withdraw: U128,
        category: String,
        investor_account: AccountId,
    ) -> Promise {
        assert_one_yocto();
        self.only_owner();
        assert!(env::prepaid_gas() >= BASE_GAS * 3, "{}", ERR_304);
        let now = env::block_timestamp();

        let investment_id = create_investment_id(category.clone(), investor_account.clone());
        self.withdraw_investment(now, investment_id.clone(), value_to_withdraw.0);

        token_contract::ft_transfer(
            investor_account.clone(),
            value_to_withdraw,
            "Vesting withdraw".to_string(),
            &self.token_contract,
            1,
            BASE_GAS,
        )
        .then(ext_self::undo_transfer(
            value_to_withdraw,
            investment_id,
            self.owner.clone(),
            &env::current_account_id(),
            0,
            BASE_GAS,
        ))
    }
}
