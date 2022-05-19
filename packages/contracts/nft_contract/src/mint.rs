use crate::*;
use near_sdk::PromiseOrValue;

#[near_bindgen]
impl Contract {

    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        PromiseOrValue::Value(U128(1))
    }
    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.

    //minter must be whitelisted possibility to mint multiple nfts in batch
    #[payable]
    pub fn nft_mint(
        &mut self,
        quantity: U128
    ) -> Vec<Token> {
        let account_id: AccountId = env::predecessor_account_id();
        let allowance: u128 = self.whitelist.get(&account_id).unwrap_or(0);

        assert!(!self.sales_locked, "sales locked");
        if self.only_whitelist {
            assert!(&allowance >= &quantity.0, "Whitelist error: this account has no allowance for minitng this amount of NFTs");
            self.whitelist.insert(&account_id, &(allowance - quantity.0));
        }
        
        let mut return_vector = Vec::new();

        let initial_storage_usage = env::storage_usage();

        let mut i: u128 = 0;
        let mut random_seed: u64 = (*env::random_seed().get(0).unwrap()).into();
        random_seed = random_seed + 1;
        let mut random_range: u64;
        let mut current_id;
        while i < quantity.0 {
            random_range = (u64::MAX / random_seed) % self.random_minting.len();
            current_id = self.random_minting.swap_remove(random_range);
            return_vector.push( 
                self.tokens.internal_mint( 
                    current_id.to_string(), 
                    account_id.clone().try_into().unwrap(), 
                    Some(TokenMetadata {
                        title: Some(format!("Tokonami #{}", &current_id)),
                        description: Some("2331 TOKONAMI Ready for the Revolution".to_string()),
                        media: Some(format!("{}/{}.png", self.url_media_base, &current_id)),
                        media_hash: None,
                        copies: None,
                        issued_at: None,
                        expires_at: None,
                        starts_at: None,
                        updated_at: None,
                        extra: None,
                        reference: Some(format!("{}/{}.json", self.url_reference_base, &current_id)),
                        reference_hash: None,

                        // special metadata
                        nft_type: Some((&current_id % 3 + 1).to_string())
                    }),
                    self.mint_cost,
                    self.perpetual_royalties.clone()
                )
            );
            i = i + 1;
        }
        refund_deposit_mint(env::storage_usage() - initial_storage_usage, self.mint_cost * quantity.0);
        return_vector
    }
    
}