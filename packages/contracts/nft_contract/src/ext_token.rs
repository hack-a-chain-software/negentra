use near_sdk::ext_contract;

#[ext_contract(ext_nep141)]
trait Nep141 {

    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: U128, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_total_supply(&self) -> U128;

    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128;
}