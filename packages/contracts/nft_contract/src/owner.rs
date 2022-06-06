use crate::*;
use near_sdk::serde::Serialize;

#[derive(Serialize)]
pub struct ContractView {
    pub item_count: u64,
    pub perpetual_royalties: HashMap<AccountId, u128>,
    pub mint_token: AccountId,
    pub mint_cost: u128,
}

impl ContractView {
    pub fn from(contract: &Contract) -> Self {
        ContractView {
            item_count: contract.item_count,
            perpetual_royalties: contract.perpetual_royalties.clone(),
            mint_token: contract.mint_token.clone(),
            mint_cost: contract.mint_cost,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn change_mint_cost(&mut self, mint_cost: U128) -> bool {
        self.only_owner();
        assert_one_or_more_yocto();

        self.mint_cost = mint_cost.0;
        true
    }

    #[payable]
    pub fn change_mint_token(&mut self, mint_token: AccountId) -> bool {
        self.only_owner();
        assert_one_or_more_yocto();

        self.mint_token = mint_token;
        true
    }

    #[payable]
    pub fn change_perpetual_royalties(
        &mut self,
        perpetual_royalties: HashMap<AccountId, u128>,
    ) -> bool {
        self.only_owner();
        assert_one_or_more_yocto();

        self.perpetual_royalties = perpetual_royalties;
        true
    }

    pub fn view(&self) -> ContractView {
        self.only_owner();

        ContractView::from(self)
    }
}
