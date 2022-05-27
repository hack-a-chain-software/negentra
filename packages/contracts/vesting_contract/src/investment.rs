struct Investment {
    account: AccountId,
    total_value: U128,
    withdrawn_value: U128,
    date_in: U128,
}

// new_investment → init investment, must deposit their tokens on
// initialization (INTERNAL, USES FT ON TRANSFER), Create the investment id using the SchemaId+AccountId
//view_investment_data(id) → returns all data, and the available withdraw
