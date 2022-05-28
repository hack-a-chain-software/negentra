use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

pub mod errors;
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

//----------------------------------- TEST -------------------------------------------------

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance, VMContext};

    use super::*;
    use std::convert::TryFrom;

    pub const TOTAL_SUPPLY: Balance = 1_000;
    pub const CONTRACT_ACCOUNT: &str = "contract.testnet";
    pub const TOKEN_ACCOUNT: &str = "token.testnet";
    pub const SIGNER_ACCOUNT: &str = "signer.testnet";
    pub const OWNER_ACCOUNT: &str = "owner.testnet";

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    pub fn get_context(
        input: Vec<u8>,
        is_view: bool,
        attached_deposit: u128,
        account_balance: u128,
        signer_id: AccountId,
        block_timestamp: u64,
    ) -> VMContext {
        VMContext {
            current_account_id: CONTRACT_ACCOUNT.to_string(),
            signer_account_id: signer_id.clone(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: signer_id.clone(),
            input,
            block_index: 0,
            block_timestamp,
            account_balance,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    pub fn init_contract() -> Contract {
        Contract {
            owner: OWNER_ACCOUNT.to_string(),
            token_contract: TOKEN_ACCOUNT.to_string(),
            schemas: LookupMap::new(StorageKey::Schemas), // inicializa o lupmap
            investments: LookupMap::new(StorageKey::Investments),
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0); // vec!() -> da pra inicializar assim, tem otimizacao ( macro vec)
        testing_env!(context);

        let contract = Contract::new(OWNER_ACCOUNT.to_string(), TOKEN_ACCOUNT.to_string());
        assert_eq!(contract.owner, OWNER_ACCOUNT.to_string());
        assert_eq!(contract.token_contract, TOKEN_ACCOUNT.to_string());
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_default() {
        let mut context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0);
        testing_env!(context);
        let _contract = Contract::default();
    }
}
