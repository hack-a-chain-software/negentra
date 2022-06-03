use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemType {
    pub total_supply: u64,
    pub minted_items: u64,
    pub supply_available: u64,
    pub metadata: Option<TokenMetadata>,
}

impl ItemType {
    pub fn mint_item_update_count(&mut self) {
        self.minted_items += 1;
        self.supply_available -= 1;
    }

    fn internal_change_supply(&mut self, new_total: u64) {
        assert!(
            self.minted_items <= new_total,
            "Cannot reduce total to number inferior to minted_items"
        );
        self.total_supply = new_total;
        self.supply_available = self.total_supply - self.minted_items;
    }

    fn update_metadata(&mut self, new_metadata: TokenMetadata) {
        self.metadata = Some(new_metadata);
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_new_item(
        &mut self,
        total_supply: u64,
        title: String,
        description: String,
        media: String,
        reference: String,
    ) -> bool {
        assert_one_yocto();
        self.only_owner();

        self.internal_create_new_item(total_supply, title, description, media, reference);

        true
    }

    #[payable]
    pub fn update_item(
        &mut self,
        item_id: u64,
        total_supply: u64,
        title: String,
        description: String,
        media: String,
        reference: String,
    ) -> bool {
        assert_one_yocto();
        self.only_owner();

        let mut old_item = self
            .item_types
            .get(&item_id)
            .expect("Error: No item found with this item_id");
        let new_metadata = TokenMetadata {
            title: Some(title),
            description: Some(description),
            media: Some(media),
            media_hash: None,
            reference: Some(reference),
            reference_hash: None,
            item_id,
        };

        if total_supply >= old_item.total_supply {
            self.item_amount_tree.update(
                &item_id,
                total_supply - old_item.total_supply,
                sum_tree::Operation::Sum,
            );
        } else {
            self.item_amount_tree.update(
                &item_id,
                old_item.total_supply - total_supply,
                sum_tree::Operation::Subtraction,
            );
        }

        old_item.internal_change_supply(total_supply);
        old_item.update_metadata(new_metadata);
        self.item_types.insert(&item_id, &old_item);

        true
    }
}

impl Contract {
    pub fn internal_create_new_item(
        &mut self,
        total_supply: u64,
        title: String,
        description: String,
        media: String,
        reference: String,
    ) -> ItemType {
        let item_id = self.item_count.clone();
        self.item_count += 1;

        let new_item = ItemType {
            total_supply,
            minted_items: 0,
            supply_available: total_supply,
            metadata: Some(TokenMetadata {
                title: Some(title),
                description: Some(description),
                media: Some(media),
                media_hash: None,
                item_id,

                reference: Some(reference),
                reference_hash: None,
            }),
        };

        self.item_types.insert(&item_id, &new_item);
        self.item_amount_tree.insert(&item_id, total_supply);

        new_item
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    // test suit create_new_item
    // method must:
    // (1) Assert that caller is owner;
    // (2) Asert that caller deposited one yocto
    // (3) Increment item counter
    // (4) Create new item in contract's memory
    #[test]
    fn test_create_new_item() {
        // (3) Increment item counter
        // (4) Create new item in contract's memory
        let context = get_context(
            vec![],
            false,
            1,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);

        let mut contract = init_contract();

        let initial_item_count = contract.item_count;

        let total_supply = 1000;
        let title = "a great title".to_string();
        let description = "description".to_string();
        let media = "media".to_string();
        let reference = "reference".to_string();

        contract.create_new_item(
            total_supply,
            title.clone(),
            description.clone(),
            media.clone(),
            reference.clone(),
        );

        // (3) Increment item counter
        assert_eq!(contract.item_count, initial_item_count + 1);

        // (4) Create new item in contract's memory
        let new_type = contract.item_types.get(&initial_item_count).unwrap();

        assert_eq!(new_type.total_supply, total_supply);
        assert_eq!(new_type.minted_items, 0);
        assert_eq!(new_type.supply_available, total_supply);
        assert_eq!(new_type.metadata.clone().unwrap().title, Some(title));
        assert_eq!(
            new_type.metadata.clone().unwrap().description,
            Some(description)
        );
        assert_eq!(new_type.metadata.clone().unwrap().media, Some(media));
        assert_eq!(
            new_type.metadata.clone().unwrap().item_id,
            initial_item_count
        );
        assert_eq!(
            new_type.metadata.clone().unwrap().reference,
            Some(reference)
        );
        assert_eq!(new_type.metadata.clone().unwrap().reference_hash, None);
    }

    #[test]
    fn test_update_item_decrease_supply() {
        let context = get_context(
            vec![],
            false,
            1,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);

        let mut contract = init_contract();

        contract.internal_create_new_item(
            1000,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );
        assert_eq!(contract.available_items(), 1000);

        let item = contract.item_types.get(&0).unwrap();
        assert_eq!(item.total_supply, 1000);
        assert_eq!(item.supply_available, 1000);

        contract.update_item(
            0,
            800,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );
        assert_eq!(contract.available_items(), 800);

        let updated_item = contract.item_types.get(&0).unwrap();
        assert_eq!(updated_item.total_supply, 800);
        assert_eq!(updated_item.supply_available, 800);
    }

    #[test]
    fn test_update_item_increase_supply() {
        let context = get_context(
            vec![],
            false,
            1,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);

        let mut contract = init_contract();

        contract.internal_create_new_item(
            1000,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );
        assert_eq!(contract.available_items(), 1000);

        let item = contract.item_types.get(&0).unwrap();
        assert_eq!(item.total_supply, 1000);
        assert_eq!(item.supply_available, 1000);

        contract.update_item(
            0,
            1200,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );
        assert_eq!(contract.available_items(), 1200);

        let updated_item = contract.item_types.get(&0).unwrap();
        assert_eq!(updated_item.total_supply, 1200);
        assert_eq!(updated_item.supply_available, 1200);
    }

    #[test]
    fn test_update_item_change_fields() {
        let context = get_context(
            vec![],
            false,
            1,
            0,
            OWNER_ACCOUNT.to_string(),
            0,
            10u64.pow(18),
        );
        testing_env!(context);

        let mut contract = init_contract();

        contract.internal_create_new_item(
            1000,
            "a great title".to_string(),
            "description".to_string(),
            "media".to_string(),
            "reference".to_string(),
        );

        let new_title = "new_title".to_string();
        let new_description = "new_description".to_string();
        let new_media = "new_media".to_string();
        let new_reference = "new_reference".to_string();

        contract.update_item(
            0,
            1000,
            new_title.clone(),
            new_description.clone(),
            new_media.clone(),
            new_reference.clone(),
        );

        let updated_item = contract.item_types.get(&0).unwrap();
        let metadata = updated_item.metadata.clone().unwrap();
        assert_eq!(metadata.title, Some(new_title));
        assert_eq!(metadata.description, Some(new_description));
        assert_eq!(metadata.media, Some(new_media));
        assert_eq!(metadata.reference, Some(new_reference));
    }
}
