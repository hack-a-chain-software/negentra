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
