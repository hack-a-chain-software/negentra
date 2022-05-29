use crate::errors::{ERR_301, ERR_302, ERR_303};
use crate::*;
pub use near_sdk::serde_json::{self, json, Value};
use near_sdk::{env, near_bindgen};

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
}
