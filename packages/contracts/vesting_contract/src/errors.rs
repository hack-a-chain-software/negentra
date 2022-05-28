// Schema erros
// PRIMEIRO NUMERO EH A SECAO
// SEGUNDO NUMERO EH A

pub(crate) const ERR_001: &str = "Vesting: Contract: function is private to owner";

pub(crate) const ERR_002: &str =
    "Vesting: Contract: new_schema: There is already a Schema with this category 
    (the category is the schema id)";

pub(crate) const ERR_003: &str =
    "Vesting: Contract: new_investment: There is already an Investment with this ID 
    (it uses the same schema and same acconunt)";

pub(crate) const ERR_004: &str =
    "Vesting: Contract: new_investment: The allocated amount for this investment 
is greater than the amount of tokens available on that  schema category:  
(schema.aloccated_quantity + total_value) MUST be SMALLER then or EQUAL to schema.total_value ";

pub(crate) const ERR_005: &str =
    "Vesting: Contract: calculate_avalibe_withdraw: Schema does not exist ";

pub(crate) const ERR_006: &str =
    "Vesting: Contract: calculate_avalibe_withdraw: Investment does not exist ";

pub(crate) const ERR_101: &str = "Vesting: Schema: Cannot create schema:  sum of 
initial_release + cliff_release + final_release  MUST be equal to FRACTION_BASE "; //padrao constante de string

pub(crate) const ERR_102: &str = "Vesting: Schema: CurveType: Return formula for this 
curve type variant has not been implemented yet"; //padrao constante de string

pub(crate) const ERR_201: &str = "Vesting: Schema: Cannot withdraw value:  sum of 
self.withdrawn_value + value_to_withdraw  MUST be SMALLER or EQUAL to self.total_value "; //padrao constante de string
