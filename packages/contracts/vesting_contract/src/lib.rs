use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption, LookUpMap;
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

pub mod investments;
pub mod schemas;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {
	owner : AccountId
	token_contract: AccountId
	schemas: LookUp<String, Schema>
	investments: LookUpMap<String, Investment>
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
    owner : AccountId,
	token_contract: AccountId,
	schemas: LookUp<String, Schema>,
	investments: LookUpMap<String, Investment>

    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this
    }


}




