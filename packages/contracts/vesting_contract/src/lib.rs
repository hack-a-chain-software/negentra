use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId,  BorshStorageKey, PanicOnDefault};

pub mod errors;
pub mod events;
pub mod investment;
pub mod schema;
pub mod utils;

use crate::errors::{ERR_001, ERR_002, ERR_003, ERR_004, ERR_005, ERR_006};
use crate::utils::{create_investment_id, split_investment_id};

use investment::Investment;
use schema::{CurveType, Schema};

pub const FRACTION_BASE: u128 = 10_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner: AccountId,
    pub token_contract: AccountId,
    pub schemas: LookupMap<String, Schema>,
    pub investments: LookupMap<String, Investment>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Schemas,
    Investments,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId, token_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        assert!(
            env::is_valid_account_id(owner.as_bytes()),
            "Invalid owner account"
        );

        // declarando os atributos
        Self {
            owner,
            token_contract,
            schemas: LookupMap::new(StorageKey::Schemas), // inicializa o lupmap
            investments: LookupMap::new(StorageKey::Investments),
        }
    }
}

//utils
impl Contract {
    pub fn only_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "{}", ERR_001);
    }
}

// high level functions
impl Contract {
    pub fn new_schema(
        &mut self,
        category: String,
        total_quantity: U128,
        initial_release: U128, //releases should be a fraction
        cliff_release: U128,
        final_release: U128,
        initial_timestamp: U64,
        cliff_delta: U64,
        final_delta: U64,
        curve_type: CurveType,
    ) {
        assert!(!self.schemas.contains_key(&category), "{}", ERR_002);

        let schema = Schema::new(
            category.clone(),
            total_quantity.0,
            initial_release.0,
            cliff_release.0,
            final_release.0,
            initial_timestamp.0,
            cliff_delta.0,
            final_delta.0,
            curve_type,
        );

        self.schemas.insert(&category, &schema);
    }

    pub fn new_investment(
        &mut self,
        category: String,
        account: AccountId,
        total_value: U128,
        date_in: Option<U64>,
    ) {
        let investment_id = create_investment_id(category.clone(), account.clone());
        assert!(
            !self.investments.contains_key(&investment_id),
            "{}",
            ERR_003
        );

        let schema = self.schemas.get(&category).expect(ERR_002);
        let allocated_quantity = schema.aloccated_quantity + total_value.0;
        assert!(allocated_quantity <= schema.total_quantity, "{}", ERR_004);
        
        let investment = Investment::new(account, total_value.0, date_in.map(|v| v.0));
        
        self.investments.insert(&investment_id, &investment);
    }

    pub fn calculate_avalibe_withdraw(
        &self,
        curent_time_stamp: u64,
        investment_id: String,
    ) -> u128 {
        let schema_id = split_investment_id(investment_id.clone()).remove(0);

        let schema = self.schemas.get(&schema_id).expect(ERR_005);
        let investment = self.investments.get(&investment_id).expect(ERR_006);

        let initial_timestamp: u64;

        if let Some(time_stamp) = investment.date_in {
            initial_timestamp = time_stamp;
        } else {
            initial_timestamp = schema.initial_timestamp;
        }

        let release: u128;

        if curent_time_stamp >= (initial_timestamp + schema.cliff_delta + schema.final_delta) {
            release = FRACTION_BASE;
        } else if curent_time_stamp >= (initial_timestamp + schema.cliff_delta) {
            let elapsed_curve_time = curent_time_stamp - initial_timestamp - schema.cliff_delta;

            release = schema.initial_release
                + schema.curve_type.calculate_curve_return(
                    schema.final_delta,
                    schema.cliff_release,
                    elapsed_curve_time,
                );
        } else if curent_time_stamp >= initial_timestamp {
            release = schema.initial_release;
        } else {
            release = 0;
        }

        let total_vested = (release * investment.total_value) / FRACTION_BASE;
        total_vested - investment.withdrawn_value
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};

    use super::*;

    use unit_testing::*;

    pub mod unit_testing;

    #[test]
    fn test_new() {
        let context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0); // vec!() -> da pra inicializar assim, tem otimizacao ( macro vec)
        testing_env!(context);

        let contract = Contract::new(OWNER_ACCOUNT.to_string(), TOKEN_ACCOUNT.to_string());
        assert_eq!(contract.owner, OWNER_ACCOUNT.to_string());
        assert_eq!(contract.token_contract, TOKEN_ACCOUNT.to_string());
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0);
        testing_env!(context);
        let _contract = Contract::default();
    }

}
