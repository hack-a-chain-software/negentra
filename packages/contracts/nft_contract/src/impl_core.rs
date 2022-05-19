use crate::*;
use modified_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use modified_contract_standards::non_fungible_token::core::NonFungibleTokenResolver;

/*
Reimplemente standard methods to overide normal Token objects with actual url and details
*/

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        self.tokens.nft_transfer(receiver_id, token_id, approval_id, memo)
    }

    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        self.tokens.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    }

    fn nft_token(self, token_id: TokenId) -> Option<Token> {
        /// change function to actually return token with correct data
        let owner_id = self.owner_by_id.get(&token_id)?;
        let metadata = self.token_metadata_by_id.and_then(|by_id| by_id.get(&token_id));
        let approved_account_ids = self
            .approvals_by_id
            .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));
        let royalty = self
            .royalties_by_id
            .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));
        Some(Token { token_id, owner_id, metadata, approved_account_ids, royalty: royalty })
    }

    fn mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: ValidAccountId,
        token_metadata: Option<TokenMetadata>,
    ) -> Token {
        panic!("Mint function is not available");
        self.tokens.mint(token_id, token_owner_id, token_metadata)
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        self.tokens.nft_resolve_transfer(
            previous_owner_id,
            receiver_id,
            token_id,
            approved_account_ids,
        )
    }
}