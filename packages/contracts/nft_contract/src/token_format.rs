use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemType {
    pub total_supply: U128,
    pub minted_items: U128,
    pub supply_available: U128,
    pub random_minting_locations: HashMap<u64, bool>,
    pub metadata: Option<TokenMetadata>,
}

impl ItemType {

    fn mint_item_update_count(&mut self) {
        self.minted_items = U128(self.minted_items.0 + 1);
        self.supply_available = U128(self.supply_available.0 - 1);
    }

    fn internal_change_supply(&mut self, new_total: U128) {
        assert!(self.minted_items.0 <= new_total.0, "Cannot reduce total to number inferior to minted_items");
        self.total_supply = new_total;
        self.supply_available = U128(self.total_supply.0 - self.minted_items.0);
    }

    fn update_metadata(&mut self, new_metadata: TokenMetadata) {
        self.metadata = Some(new_metadata);
    }

}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn create_new_item(&mut self, total_supply: U128, title: String, description: String,
                           media: String, reference: String) -> bool {
        
        assert_one_yocto();
        self.only_owner();
        let item_id = self.item_count.clone();
        self.item_count += 1;
        let new_item = ItemType {
            total_supply,
            minted_items: 0,
            supply_available: total_supply,
            random_minting_locations: HashMap::new(),
            metadata: Some(TokenMetadata {
                title: Some(title),
                description: Some(description),
                media: Some(media),
                media_hash: None,
                copies: Some(total_supply),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: Some(reference),
                reference_hash: None,
                item_id
            })
        }

        let random_minting_next_index = self.random_minting.len() as u128;
        let i = 0;
        while i < total_supply.0 {
            self.random_minting.push(&item_id);
            new_item.random_minting_locations.insert(&(random_minting_next_index + i), true);
            i += 1;
        }

        self.item_types.insert(&item_id, &new_item);

    }

    #[payable]
    pub fn update_item(&mut self, item_id: U128, total_supply: U128, title: String, description: String,
                        media: String, reference: String) -> bool {
        
        assert_one_yocto();
        self.only_owner();

        let mut old_item = self.item_types.get(&item_id.0).expect("Error: No item found with this item_id");
        
        let new_metadata = TokenMetadata {
            title: Some(title),
            description: Some(description),
            media: Some(media),
            media_hash: None,
            copies: Some(total_supply),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: Some(reference),
            reference_hash: None,
            item_id
        }

        
        if total_supply >= old_item.total_supply {
            let new_items = total_supply.0 - old_item.total_supply.0;
            let random_minting_next_index = self.random_minting.len() as u128;
            let i = 0;
            while i < new_items {
                self.random_minting.push(&item_id);
                old_item.random_minting_locations.insert(&(random_minting_next_index + i), true);
                i += 1;
            }
        } else {
            //find first n occurences in random_mintint
            //use remove reorder function
            let items_to_remove = old_item.total_supply.0 - total_supply.0
            let mut keys_iterator = old_item.random_minting_locations.keys();
            let i = 0
            while i < items_to_remove {
                self.vec_remove_store_loc(keys_iterator.next().unwrap());
            }
        }

        old_item.internal_change_supply(total_supply)
        old_item.update_metadata(new_metadata);
        self.item_types.insert(&item_id.0, &old_item);

    }

}

impl Contract {

    fn vec_remove_store_loc(&mut self, loc: u64) {
        let last_item = self.random_minting.len() - 1;
        let removed = self.random_minting.swap_remove(loc);
        
        //if last element was removed, will return None in get
        //handle None by doing nothing and Some by replacing
        //locations inside item Struct
        if let Some(item_id) = self.random_minting.get(loc) {
            let mut item_struct = self.item_types.get(&item_id).unwrap();
            item_struct.random_minting_locations.remove(&last_item);
            item_struct.random_minting_locations.insert(loc, true);
        }

    }

}