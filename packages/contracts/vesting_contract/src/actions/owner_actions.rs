use crate::errors::{ERR_301, ERR_302};
use crate::*;
use near_sdk::{env, near_bindgen};
//blockchain exposed

#[near_bindgen]
impl Contract {
    // ft on transfer is an onwner action, but it is called by the token contract
    pub fn ft_on_transfer(&mut self, sender_id: String, amount: U128, msg: String) -> U128 {
        // validar que o predecessor eh o contrato do tolen certo
        assert_eq!(env::predecessor_account_id(), self.token_contract, ERR);
        assert_eq!(sender_id, self.owner, "{}", ERR_001);

        // validar que o owner eh o owner -> signer tem que ser o owner
        //transformar o timestamp
        //new_schema()
    }
}

// let parsed_message: Value = serde_json::from_str(&msg).expect("1");
// let receiver = parsed_message["receiver"].as_str().expect("2");
// let burner = parsed_message["burner"].as_str().expect("3");
