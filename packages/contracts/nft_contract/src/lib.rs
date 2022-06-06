use modified_contract_standards::non_fungible_token::events::NftBurn;
use modified_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
};
use modified_contract_standards::non_fungible_token::royalty::{Payout, Royalty};
use modified_contract_standards::non_fungible_token::NonFungibleToken;
use modified_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, Vector};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::utils::assert_one_yocto;
use near_sdk::{env, near_bindgen, AccountId, BorshIntoStorageKey, PanicOnDefault, Promise};
use std::collections::HashMap;
use sum_tree::SumTree;

pub use std::convert::TryFrom;
use token_format::ItemType;

pub mod burn;
pub mod impl_royalties;
pub mod mint;
pub mod owner;
pub mod token_format;

near_sdk::setup_alloc!();

pub fn assert_one_or_more_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of 1 yoctoNEAR or more"
    )
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,

    pub item_types: LookupMap<u64, ItemType>,
    pub item_count: u64,
    pub item_amount_tree:
        SumTree<LookupMap<u64, u64>, LookupMap<u64, u64>, LookupMap<u64, u64>, Vector<u64>>,

    pub perpetual_royalties: HashMap<AccountId, u128>,

    pub mint_token: AccountId,
    pub mint_cost: u128,
}

#[derive(BorshSerialize)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    ItemTypes,
    ItemAmountTree,
}

impl BorshIntoStorageKey for StorageKey {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: ValidAccountId,
        metadata: NFTContractMetadata,
        mint_token: AccountId,
        mint_cost: U128,
        royalties_account: AccountId,
        royalties_value: U128,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        let perpetual_royalties = HashMap::from([(royalties_account, royalties_value.0)]);

        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
                perpetual_royalties.clone(),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            item_types: LookupMap::new(StorageKey::ItemTypes),
            item_count: 0,
            item_amount_tree: SumTree::<
                LookupMap<u64, u64>,
                LookupMap<u64, u64>,
                LookupMap<u64, u64>,
                Vector<u64>,
            >::new(StorageKey::ItemAmountTree),
            perpetual_royalties,
            mint_token,
            mint_cost: mint_cost.0,
        }
    }
}

modified_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
modified_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
modified_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

impl Contract {
    fn only_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Only owner can call this function"
        );
    }
}

#[cfg(test)]
mod tests {
    pub use near_sdk::{testing_env, Balance, MockedBlockchain, VMContext};

    pub use super::*;

    pub const CONTRACT_ACCOUNT: &str = "contract.testnet";
    pub const TOKEN_ACCOUNT: &str = "token.testnet";
    pub const OWNER_ACCOUNT: &str = "owner.testnet";
    pub const USER_ACCOUNT: &str = "user.testnet";

    pub fn get_context(
        input: Vec<u8>,
        is_view: bool,
        attached_deposit: u128,
        account_balance: u128,
        signer_id: AccountId,
        block_timestamp: u64,
        prepaid_gas: u64,
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
            prepaid_gas,
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    pub fn init_contract() -> Contract {
        let mut royalties: HashMap<AccountId, u128> = HashMap::new();
        royalties.insert(OWNER_ACCOUNT.to_string(), 500);
        Contract {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                ValidAccountId::try_from(OWNER_ACCOUNT).unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
                royalties.clone(),
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&NFTContractMetadata {
                    spec: "1.0.0".to_string(), // required, essentially a version like "nft-1.0.0"
                    name: "test".to_string(),  // required, ex. "Mosaics"
                    symbol: "BLA".to_string(), // required, ex. "MOSIAC"
                    icon: None,                // Data URL
                    base_uri: None, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
                    reference: None, // URL to a JSON file with more info
                    reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
                }),
            ),
            item_types: LookupMap::new(StorageKey::ItemTypes),
            item_count: 0,
            item_amount_tree: SumTree::<
                LookupMap<u64, u64>,
                LookupMap<u64, u64>,
                LookupMap<u64, u64>,
                Vector<u64>,
            >::new(StorageKey::ItemAmountTree),
            perpetual_royalties: royalties,
            mint_token: TOKEN_ACCOUNT.to_string(),
            mint_cost: 1000,
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(
            vec![],
            false,
            0,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        ); // vec!() -> da pra inicializar assim, tem otimizacao (macro vec)
        testing_env!(context);

        let mint_cost = U128(10);
        let royalties_value = U128(15);
        let metadata = NFTContractMetadata {
            spec: "nft-1.0.0".to_string(), // required, essentially a version like "nft-1.0.0"
            name: "test".to_string(),      // required, ex. "Mosaics"
            symbol: "BLA".to_string(),     // required, ex. "MOSIAC"
            icon: None,                    // Data URL
            base_uri: None, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
            reference: None, // URL to a JSON file with more info
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        };

        let contract = Contract::new(
            ValidAccountId::try_from(OWNER_ACCOUNT).unwrap(),
            metadata.clone(),
            TOKEN_ACCOUNT.to_string(),
            mint_cost,
            OWNER_ACCOUNT.to_string(),
            royalties_value,
        );

        assert_eq!(contract.tokens.owner_id, OWNER_ACCOUNT.to_string());
        assert_eq!(contract.metadata.get().unwrap(), metadata);
        assert_eq!(contract.mint_token, TOKEN_ACCOUNT.to_string());
        assert_eq!(
            contract
                .perpetual_royalties
                .get(&OWNER_ACCOUNT.to_string())
                .unwrap(),
            &15u128
        );
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(
            vec![],
            false,
            0,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);
        let _contract = Contract::default();
    }
    // #[test]
    // #[should_panic(expected = "Already initialized")]
    // fn test_existing_state() {
    //     let context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0, 10u64.pow(18));
    //     testing_env!(context);

    //     let _contract = init_contract();

    // }
    // #[test]
    // fn implementation_test_calculate_avalibe_withdraw() {
    //     // This is an implementation test. It's used to assert that implementation of code is correct
    //     // in case of refactoring you can safely disregard this test and erase it
    //     // Asserts that calculate_available_withdraw method of Contract is calculating the correct
    //     // amount.
    //     let context = get_context(vec![], false, 0, 0, OWNER_ACCOUNT.to_string(), 0, 10u64.pow(18));
    //     testing_env!(context);
    //     let mut contract = init_contract();
    //     let total_quantity: u128 = 1_000_000_000;
    //     let schema_name = "schema".to_string();
    //     let investor_name = "investor".to_string();
    //     let investment_id = create_investment_id(schema_name.clone(), investor_name.clone());

    //     let initial_release = 30;
    //     let cliff_release = 50;
    //     let final_release = 20;

    //     let schema = schema::Schema {
    //         category: schema_name.clone(),
    //         allocated_quantity: 0,
    //         total_quantity,
    //         initial_release,
    //         cliff_release,
    //         final_release,
    //         initial_timestamp: 0,
    //         cliff_delta: 100_000,
    //         final_delta: 100_000,
    //         curve_type: schema::CurveType::Linear { discrete_period: 10 },
    //         investments: Vec::new()
    //     };

    //     contract.schemas.insert(&schema_name, &schema);

    //     let mut investment = investment::Investment {
    //         account: investor_name,
    //         total_value: total_quantity,
    //         withdrawn_value: 0,
    //         date_in: None,
    //     };
    //     contract.investments.insert(&investment_id, &investment);

    //     //assert initial stage
    //     assert_eq!(
    //         contract.calculate_available_withdraw(0, investment_id.clone()),
    //         (initial_release * total_quantity) / FRACTION_BASE
    //     );
    //     assert_eq!(
    //         contract.calculate_available_withdraw(100_000, investment_id.clone()),
    //         (initial_release * total_quantity) / FRACTION_BASE
    //     );

    //     //assert curve stage
    //     let curve_percentage = schema.curve_type.calculate_curve_return(
    //         schema.final_delta,
    //         schema.cliff_release,
    //         30_000,
    //     );
    //     assert_eq!(
    //         contract.calculate_available_withdraw(130_000, investment_id.clone()),
    //         ((initial_release * total_quantity) + (curve_percentage * total_quantity))
    //             / FRACTION_BASE
    //     );

    //     //assert final stage
    //     assert_eq!(
    //         contract.calculate_available_withdraw(200_000, investment_id.clone()),
    //         total_quantity
    //     );

    //     let withdrawn_amount = 100;
    //     investment.withdrawn_value = withdrawn_amount;
    //     contract.investments.insert(&investment_id, &investment);

    //     //assert initial stage
    //     assert_eq!(
    //         contract.calculate_available_withdraw(0, investment_id.clone()),
    //         ((initial_release * total_quantity) / FRACTION_BASE) - withdrawn_amount
    //     );
    //     assert_eq!(
    //         contract.calculate_available_withdraw(100_000, investment_id.clone()),
    //         ((initial_release * total_quantity) / FRACTION_BASE) - withdrawn_amount
    //     );

    //     //assert curve stage
    //     let curve_percentage1 = schema.curve_type.calculate_curve_return(
    //         schema.final_delta,
    //         schema.cliff_release,
    //         30_000,
    //     );
    //     assert_eq!(
    //         contract.calculate_available_withdraw(130_000, investment_id.clone()),
    //         (((initial_release * total_quantity) + (curve_percentage1 * total_quantity))
    //             / FRACTION_BASE)
    //             - withdrawn_amount
    //     );

    //     //assert final stage
    //     assert_eq!(
    //         contract.calculate_available_withdraw(200_000, investment_id.clone()),
    //         total_quantity - withdrawn_amount
    //     );
    // }
}
