use crate::*;

/*
Implement functionality for owner to add new nft types here
*/

// #[near_bindgen]
// impl Contract {

//     pub fn initilize_random_generator(&mut self) -> bool {
//         let initial_len: u128 = self.random_minting.len().into();
//         let mut i: u128 = 1;
//         while i <= 50 {
//             if i + initial_len > 2331 {
//                 return true
//             } 
//             self.random_minting.push(&(&i + &initial_len));
//             i = i + 1
//         }
//         false
//     }
    

//     //add people to whitelist
//     #[payable]
//     pub fn add_to_whitelist(
//         &mut self,
//         whitelist_map: HashMap<AccountId, u128>
//     ) -> bool {
//         assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
//         assert_one_or_more_yocto();
//         for key in whitelist_map.keys() {
//             self.whitelist.insert(key, whitelist_map.get(key).unwrap());
//         }
//         true
//     }

//     //add people to whitelist
//     pub fn is_whitelist(
//         &self,
//         account_id: AccountId
//     ) -> u128 {
//         self.whitelist.get(&account_id).unwrap_or(0)
//     }

//     #[payable]
//     pub fn retrieve_funds(&mut self, quantity: U128) -> Promise {
//         assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
//         assert_one_or_more_yocto();

//         Promise::new(self.funds_beneficiary.clone()).transfer(quantity.0)
//     }

//     #[payable]
//     pub fn unlock_sales(&mut self, sales_lock: bool) -> bool {
//         assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
//         assert_one_or_more_yocto();

//         self.sales_locked = sales_lock;
//         true
//     }

//     #[payable]
//     pub fn unlock_whitelist(&mut self, whitelist_lock: bool) -> bool {
//         assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
//         assert_one_or_more_yocto();

//         self.only_whitelist = whitelist_lock;
//         true
//     }

//     #[payable]
//     pub fn change_mint_cost(&mut self, mint_cost: U128) -> bool {
//         assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized");
//         assert_one_or_more_yocto();

//         self.mint_cost = mint_cost.0;
//         true
//     }
    
// }