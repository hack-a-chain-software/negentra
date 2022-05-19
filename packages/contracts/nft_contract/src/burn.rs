use crate::*;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn nft_burn(
        &mut self,
        sender_id: &AccountId,
        token_id: &TokenId,
    ) -> bool {
        assert_one_or_more_yocto();
        self.tokens.internal_transfer(
            sender_id,
            &"system".to_string(),
            token_id,
            None,
            None
        );
        let owner = self.tokens.owner_by_id.get(&token_id).unwrap();
        
        NftBurn { owner_id: &owner, token_ids: &[&token_id], memo: None, authorized_id: None }.emit();
        true
    }
    
}