pub fn create_investment_id(category: String, account: String) -> String {
    format!("{}{}{}", category, "@", account)
}
