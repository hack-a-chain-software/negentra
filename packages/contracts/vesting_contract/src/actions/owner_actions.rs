//blockchain exposed



#[near_bindgen]

    // validar que o predecessor eh o contrato do tolen certo 
    // validar que o owner eh o owner -> signer tem que ser o owner 
    // 
    //new_schema()


//not blockchain exposed
pub fn ft_on_transfer(sender_id: String,
    amount: U128,
    msg: String
)U128 {}