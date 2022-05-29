use super::*;
use rand;
use rand::prelude::{Rng};

// mock the context for testing, notice "signer_account_id" that was accessed above from env::
pub fn get_context(
    input: Vec<u8>,
    is_view: bool,
    attached_deposit: u128,
    account_balance: u128,
    signer_id: AccountId,
    block_timestamp: u64,
) -> VMContext {
    VMContext {
        current_account_id: CONTRACT_ACCOUNT.to_string(),
        signer_account_id: signer_id.clone(),
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id: signer_id.clone(),
        input,
        block_index: 0,
        block_timestamp,
        account_balance,
        account_locked_balance: 0,
        storage_usage: 0,
        attached_deposit,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view,
        output_data_receivers: vec![],
        epoch_height: 19,
    }
}

pub fn init_contract() -> Contract {
    let mut generator = rand::thread_rng();
    let random: u128 = generator.gen();
    Contract {
        owner: OWNER_ACCOUNT.to_string(),
        token_contract: TOKEN_ACCOUNT.to_string(),
        schemas: LookupMap::new(random.to_string().into_bytes()),
        investments: LookupMap::new((random + 1).to_string().into_bytes()),
    }
}

pub fn init_schema() -> Schema {
    Schema{
        category: "category".to_string(),
        aloccated_quantity: 0,
        total_quantity: 0,
        initial_release: 0,
        cliff_release: 0,
        final_release: 0,
        initial_timestamp: 0,
        cliff_delta: 0,
        final_delta: 0,
        curve_type: CurveType::Linear { discrete_period: 0 },
    }
}