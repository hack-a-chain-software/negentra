use crate::*;
use near_sdk::PromiseOrValue;
use std::convert::TryFrom;
use sum_tree::Operation;

#[near_bindgen]
impl Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(env::predecessor_account_id(), "", "");
        assert!(amount.0 >= self.mint_cost, "");

        let refund = amount.0 - self.mint_cost;

        PromiseOrValue::Value(U128(refund))
    }
    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.

    fn nft_mint(&mut self) -> Token {
        let account_id: AccountId = env::predecessor_account_id();

        let random_seed: u64 = (*env::random_seed().get(0).unwrap() + 1).into();
        let total = self.item_amount_tree.root().unwrap_or(0);

        let drawn_number = (u64::MAX / random_seed) % total; // hash random seed with total

        let id = self.item_amount_tree.find(drawn_number).unwrap();
        let mut item = self.item_types.get(&id).unwrap();

        self.item_amount_tree.update(&id, 1, Operation::Subtraction);
        item.mint_item_update_count();

        let token_id = format!("{} #{}", id, item.minted_items);

        self.tokens.internal_mint(
            token_id,
            ValidAccountId::try_from(account_id.clone()).unwrap(),
            item.metadata,
            self.mint_cost,
        )
    }
}
