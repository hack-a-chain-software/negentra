use crate::*;
use modified_contract_standards::non_fungible_token::utils::refund_deposit;
use near_sdk::PromiseOrValue;
use std::convert::{TryFrom, TryInto};
use sum_tree::Operation;

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    #[payable]
    pub fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            env::predecessor_account_id(),
            self.mint_token,
            "Contract only accepts calls from {}",
            self.mint_token
        );
        assert!(
            amount.0 >= self.mint_cost,
            "Must deposit at least {} to mint nfts",
            self.mint_cost
        );

        let initial_storage = env::storage_usage();
        self.nft_mint();
        let final_storage = env::storage_usage();

        refund_deposit(final_storage - initial_storage);

        let refund = amount.0 - self.mint_cost;
        PromiseOrValue::Value(U128(refund))
    }
}

impl Contract {
    fn nft_mint(&mut self) -> Token {
        let total = self.available_items();
        assert!(total > 0, "There are no available items to mint");

        let random_seed = *env::random_seed().get(0).unwrap();
        let hash = env::keccak256(&[random_seed, (total % 256) as u8]);
        let bytes = (hash[0..8]).try_into().unwrap();

        let drawn_number = (u64::MAX / u64::from_be_bytes(bytes)) % total;

        let id = self.item_amount_tree.find(drawn_number).unwrap();
        let mut item = self.item_types.get(&id).unwrap();

        self.item_amount_tree.update(&id, 1, Operation::Subtraction);
        item.mint_item_update_count();
        self.item_types.insert(&id, &item);

        let token_id = format!("{} #{}", id, item.minted_items);

        self.tokens.internal_mint(
            token_id,
            ValidAccountId::try_from(env::predecessor_account_id().clone()).unwrap(),
            item.metadata,
            self.mint_cost,
        )
    }

    pub fn available_items(&self) -> u64 {
        self.item_amount_tree.root().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn create_item(contract: &mut Contract) {
        contract.internal_create_new_item(
            1000,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "There are no available items to mint")]
    fn test_mint_no_items() {
        let context = get_context(
            vec![],
            false,
            0,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);

        let mut contract = init_contract();

        contract.nft_mint();
    }

    #[test]
    fn test_mint_update_supply() {
        let context = get_context(vec![], false, 1, 0, "".to_string(), 0, 10u64.pow(18));
        testing_env!(context);

        let mut contract = init_contract();
        create_item(&mut contract);

        contract.nft_mint();
        let item = contract.item_types.get(&0).unwrap();

        assert_eq!(item.minted_items, 1);
        assert_eq!(item.supply_available, 999);
        assert_eq!(contract.available_items(), 999);
    }

    #[test]
    #[should_panic(expected = "Contract only accepts calls from")]
    fn test_transfer_reject_cross_calls() {
        let context = get_context(vec![], false, 1, 0, USER_ACCOUNT.to_string(), 0, 1u64 << 5);
        testing_env!(context);

        let mut contract = init_contract();

        contract.ft_on_transfer(
            ValidAccountId::try_from(USER_ACCOUNT).unwrap(),
            U128(1000),
            "".to_string(),
        );
    }

    #[test]
    #[should_panic(expected = "Must deposit at least")]
    fn test_transfer_min_deposit() {
        let context = get_context(vec![], false, 1, 0, "".to_string(), 0, 10u64.pow(18));
        testing_env!(context);

        let mut contract = init_contract();

        contract.ft_on_transfer(
            ValidAccountId::try_from(USER_ACCOUNT).unwrap(),
            U128(999),
            "".to_string(),
        );
    }

    #[test]
    fn test_transfer_mint_refund() {
        testing_env!(get_context(
            vec![],
            false,
            5980000000000000000000,
            0,
            TOKEN_ACCOUNT.to_string(),
            0,
            1u64 << 50,
        ));

        let mut contract = init_contract();
        create_item(&mut contract);

        let refund = contract.ft_on_transfer(
            ValidAccountId::try_from(USER_ACCOUNT).unwrap(),
            U128(1001),
            "".to_string(),
        );

        match refund {
            PromiseOrValue::Promise(_) => panic!(),
            PromiseOrValue::Value(U128(v)) => assert_eq!(v, 1),
        }
    }

    #[should_panic(expected = "Must attach")]
    #[test]
    fn test_attach_deposit() {
        testing_env!(get_context(
            vec![],
            false,
            10,
            0,
            TOKEN_ACCOUNT.to_string(),
            0,
            1u64 << 50,
        ));

        let mut contract = init_contract();
        create_item(&mut contract);

        contract.ft_on_transfer(
            ValidAccountId::try_from(USER_ACCOUNT).unwrap(),
            U128(1001),
            "".to_string(),
        );
    }
}
