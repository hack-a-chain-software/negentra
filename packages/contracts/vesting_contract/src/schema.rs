use crate::errors::ERR_101;
use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Schema {
    pub category: String,
    pub aloccated_quantity: u128,
    pub total_quantity: u128,
    pub initial_release: u128, //releases should be a fraction
    pub cliff_release: u128,
    pub final_release: u128,
    pub initial_timestamp: u64,
    pub cliff_delta: u64,
    pub final_delta: u64,
    pub curve_type: CurveType,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum CurveType {
    Linear,
}

impl Schema {
    pub fn new(
        category: String,
        total_quantity: u128,
        initial_release: u128, //releases should be a fraction
        cliff_release: u128,
        final_release: u128,
        initial_timestamp: u64,
        cliff_delta: u64,
        final_delta: u64,
        curve_type: CurveType,
    ) -> Self {
        assert_eq!(
            (initial_release + cliff_release + final_release),
            FRACTION_BASE,
            "{}",
            ERR_101
        );

        Self {
            category,
            aloccated_quantity: 0,
            total_quantity,
            initial_release,
            cliff_release,
            final_release,
            initial_timestamp,
            cliff_delta,
            final_delta,
            curve_type,
        }
    }

    pub fn change_schema(
        &mut self,
        total_quantity: u128,
        initial_release: u128, //releases should be a fraction
        cliff_release: u128,
        final_release: u128,
        initial_timestamp: u64,
        cliff_delta: u64,
        final_delta: u64,
        curve_type: CurveType,
    ) {
        assert_eq!(
            (initial_release + cliff_release + final_release),
            FRACTION_BASE,
            "{}",
            ERR_101
        );

        self.total_quantity = total_quantity;
        self.initial_release = initial_release;
        self.cliff_release = cliff_release;
        self.final_release = final_release;
        self.initial_timestamp = initial_timestamp;
        self.cliff_delta = cliff_delta;
        self.final_delta = final_delta;
        self.curve_type = curve_type;
    }
}

//view_schema_data(id) → show schema data
