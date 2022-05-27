use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

pub mod investment;
pub mod schema;

use crate::errors::{ERR_001};

use investment::Investment;
use schema::Schema; 

pub const FRACTION_BASE: u128 = 10_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
    owner: AccountId,
    token_contract: AccountId,
    schemas: LookupMap<String, Schema>,
    investments: LookupMap<String, Investment>,
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
        };
    }
}

impl Contract{
    pub fn only_owner(&self){
        assert_eq!(env::predecessor_account_id(), self.owner, "{}", ERR_001);
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
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&get_test_meta())),
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
        let mut context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string()), 0;
        testing_env!(context);
        let _contract = Contract::default();
    }
}
