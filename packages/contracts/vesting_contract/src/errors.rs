// Schema erros
// PRIMEIRO NUMERO EH A SECAO
// SEGUNDO NUMERO EH A

pub(crate) const ERR_001: &str = "Vesting: Contract: function is private to owner";

pub(crate) const ERR_101: &str = "Vesting: Schema: Cannot create schema:  sum of 
initial_release + cliff_release + final_release  MUST be equal to FRACTION_BASE "; //padrao constante de string

pub(crate) const ERR_102: &str =
    "Vesting: Schema: new_schema: There is already a Schema with this category 
    (the category is the schema id)";
