use crate::*;

#[near_bindgen]
impl Contract {

    //calculates the payout for a token given the passed in balance. This is a view method
    pub fn nft_payout(&self, token_id: TokenId, balance: U128, max_len_payout: u32) -> Payout {
        self.tokens.nft_payout(token_id, balance, max_len_payout)
    }

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance. 
    #[payable]
    pub fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout { 
        self.tokens.nft_transfer_payout(
            receiver_id,
            token_id,
            approval_id,
            memo,
            balance,
            max_len_payout,
        )
    }

}