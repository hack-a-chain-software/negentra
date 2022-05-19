use modified_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use modified_contract_standards::non_fungible_token::{Token, TokenId};
use modified_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, Vector};
use near_sdk::json_types::{ValidAccountId};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use modified_contract_standards::non_fungible_token::utils::{
    refund_deposit_mint
};
use modified_contract_standards::non_fungible_token::royalty::{Royalty, Payout};
use modified_contract_standards::non_fungible_token::events::{NftBurn};
use std::convert::TryInto;

pub mod burn;
pub mod impl_royalties;
pub mod mint;
pub mod owner;

near_sdk::setup_alloc!();

pub fn assert_one_or_more_yocto() {
    assert!(env::attached_deposit() >= 1, "Requires attached deposit of 1 yoctoNEAR or more")
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub tokens: NonFungibleToken,
    pub metadata: LazyOption<NFTContractMetadata>,

    
    
    pub funds_beneficiary: AccountId,
    pub perpetual_royalties: HashMap<AccountId, u128>,
    pub whitelist: LookupMap<AccountId, u128>,
    pub mint_cost: u128,
    pub sales_locked: bool,
    pub only_whitelist: bool,
    pub random_minting: Vector<u128>,

    pub url_media_base: String,
    pub url_reference_base: String
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Royalties,
    Whitelist,
    RandomMinting
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id.clone(),
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            U128(10),
            owner_id.to_string(),
            U128(500),
            "test".to_string(),
            "test".to_string()
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata, mint_cost: U128,
         royalties_account: AccountId, royalties_value: U128, url_media_base: String, url_reference_base: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
                Some(StorageKey::Royalties)
            ),
            funds_beneficiary: royalties_account.clone(),
            perpetual_royalties: HashMap::from([(royalties_account, royalties_value.0)]),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            whitelist: LookupMap::new(StorageKey::Whitelist),
            mint_cost: mint_cost.0,
            sales_locked: true,
            only_whitelist: true,
            random_minting: Vector::new(StorageKey::RandomMinting),
            url_media_base,
            url_reference_base
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

