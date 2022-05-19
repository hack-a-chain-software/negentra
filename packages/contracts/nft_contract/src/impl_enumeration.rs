use crate::*;
use modified_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;

/*
Reimplemente standard methods to overide normal Token objects with actual url and details
*/

impl Contract {
    /// Helper function used by a enumerations methods
    /// Note: this method is not exposed publicly to end users
    /// Modify to return token with correct information
    fn high_level_enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
        let metadata = self.token_metadata_by_id.as_ref().unwrap().get(&token_id);
        let approved_account_ids =
            Some(self.approvals_by_id.as_ref().unwrap().get(&token_id).unwrap_or_default());
        let royalty = self
            .royalties_by_id.as_ref()
            .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));

        /// Should return with item's metadata
        Token { token_id, owner_id, metadata, approved_account_ids, royalty }
    }
}

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(self) -> U128 {
        self.tokens.nft_total_supply()
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        
        // Get starting index, whether or not it was explicitly given.
        // Defaults to 0 based on the spec:
        // https://nomicon.io/Standards/NonFungibleToken/Enumeration.html#interface
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.owner_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        self.owner_by_id
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_id, owner_id)| self.enum_get_token(owner_id, token_id))
            .collect()

    }

    fn nft_supply_for_owner(self, account_id: ValidAccountId) -> U128 {
        self.tokens.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: ValidAccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        self.tokens.nft_tokens_for_owner(account_id, from_index, limit)
    }
}