/*!
Non-Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::non_fungible_token::core::StorageKey as InternalStorageKey;
use near_contract_standards::non_fungible_token::events::NftMint;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token_count: u128,
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    nft_owners: LookupMap<AccountId, MintOption>,
    nft_packs: LookupMap<MintOption, Vec<TokenMetadata>>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", tag = "type")]
pub enum MintOption {
    Male,
    Female,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    NftOwners,
    NftPacks,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            token_count: 0,
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            nft_owners: LookupMap::new(StorageKey::NftOwners),
            nft_packs: LookupMap::new(StorageKey::NftPacks),
        }
    }

    #[payable]
    pub fn add_to_collection(&mut self, collection: MintOption, metadata: TokenMetadata) {
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id);
        let mut data_vec = self.nft_packs.get(&collection).unwrap_or(Vec::new());
        data_vec.push(metadata);
        self.nft_packs.insert(&collection, &data_vec);
    }

    #[payable]
    pub fn remove_from_collection(&mut self, collection: MintOption, index: U64) {
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id);
        let mut data_vec = self.nft_packs.get(&collection).unwrap();
        data_vec.remove(index.0 as usize);
        self.nft_packs.insert(&collection, &data_vec);
    }

    pub fn view_collection(&self, collection: MintOption) -> Vec<TokenMetadata> {
        self.nft_packs.get(&collection).unwrap_or(Vec::new())
    }

    #[payable]
    pub fn nft_mint(&mut self, user_type: MintOption) {
        let initial_storage = env::storage_usage();

        let minter = env::predecessor_account_id();
        assert!(
            !self.nft_owners.contains_key(&minter),
            "user already minted"
        );
        let tokens_to_mint = self.nft_packs.get(&user_type).unwrap();
        let mut token_counter = self.token_count;
        for item in tokens_to_mint.into_iter() {
            self.internal_mint_without_refund(token_counter.to_string(), minter.clone(), Some(item));
            token_counter += 1;
        }
        let minted_vec: Vec<String> = 
            (self.token_count..token_counter)
            .collect::<Vec<u128>>()
            .iter()
            .map(|v| v.to_string())
            .collect();
        NftMint {
            owner_id: &minter,
            token_ids: &minted_vec.iter().map(|v| v.as_ref()).collect::<Vec<&str>>()[..],
            memo: None,
        }
        .emit();
        self.token_count = token_counter;
        self.nft_owners.insert(&minter, &user_type);

        let storage_added = env::storage_usage() - initial_storage;
        self.refund_deposit(storage_added, minter);
    }
}

impl Contract {

    pub fn internal_mint_without_refund(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> Token {
        if self.tokens.token_metadata_by_id.is_some() && token_metadata.is_none() {
            env::panic_str("Must provide metadata");
        }
        if self.tokens.owner_by_id.get(&token_id).is_some() {
            env::panic_str("token_id must be unique");
        }

        let owner_id: AccountId = token_owner_id;

        // Core behavior: every token must have an owner
        self.tokens.owner_by_id.insert(&token_id, &owner_id);

        // Metadata extension: Save metadata, keep variable around to return later.
        // Note that check above already panicked if metadata extension in use but no metadata
        // provided to call.
        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, token_metadata.as_ref().unwrap()));

        // Enumeration extension: Record tokens_per_owner for use with enumeration view methods.
        if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                UnorderedSet::new(InternalStorageKey::TokensPerOwner {
                    account_hash: env::sha256(owner_id.as_bytes()),
                })
            });
            token_ids.insert(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }

        // Approval Management extension: return empty HashMap as part of Token
        let approved_account_ids = if self.tokens.approvals_by_id.is_some() {
            Some(HashMap::new())
        } else {
            None
        };

        // Return any extra attached deposit not used for storage

        Token {
            token_id,
            owner_id,
            metadata: token_metadata,
            approved_account_ids,
        }
    }

    pub fn refund_deposit(&mut self, storage_used: u64, account_id: AccountId) {
        let required_cost = env::storage_byte_cost() * storage_used as u128;
        let attached_deposit = env::attached_deposit();
    
        require!(
            required_cost <= attached_deposit,
            format!("Must attach {} yoctoNEAR to cover storage", required_cost)
        );
    
        let refund = attached_deposit - required_cost;
        if refund > 1 {
            Promise::new(account_id).transfer(refund);
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
