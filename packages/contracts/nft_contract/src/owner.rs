use crate::*;

#[near_bindgen]
impl Contract {
    fn assert_owner() {
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
    }

    #[payable]
    pub fn change_mint_cost(&mut self, mint_cost: U128) bool {
        self.assert_owner();
        assert_one_or_more_yocto!();

        self.mint_cost = mint_cost.0;
        true
    }

    #[payable]
    pub fn change_mint_token(&mut self, mint_token: AccountId) bool {
        self.assert_owner();
        assert_one_or_more_yocto!();

        self.mint_token = mint_token;
        true
    }
    
    #[payable]
    pub fn change_perpetual_royalties(&mut self, perpetual_royalties: HashMap<AccountId, u128>) bool {
        self.assert_owner();
        assert_one_or_more_yocto!();

        self.perpetual_royalties = perpetual_royalties
        true
    }

    pub fn idk(&mut self) {
        
    }
}
