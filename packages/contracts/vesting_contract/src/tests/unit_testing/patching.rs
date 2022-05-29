use super::*;
//monkey patch method implementations for unit tests

pub fn patch_schema() -> (PatchGuard, PatchGuard, PatchGuard) {
    let new_patch = guerrilla::patch9(Schema::new, |_0, _1, _2, _3, _4, _5, _6, _7, _8| Schema {
        category: _0,
        aloccated_quantity: 0,
        total_quantity: 0,
        initial_release: 0,
        cliff_release: 0,
        final_release: 0,
        initial_timestamp: 0,
        cliff_delta: 0,
        final_delta: 0,
        curve_type: CurveType::Linear { discrete_period: 0 },
    });
    // let change_patch = 0;
    // let curve_patch = 0;
    let change_patch = guerrilla::patch9(
        Schema::change_schema,
        |_0, _1, _2, _3, _4, _5, _6, _7, _8| {
            _0.category = "changed".to_string();
            _0.aloccated_quantity = 1;
            _0.total_quantity = 1;
            _0.initial_release = 1;
            _0.cliff_release = 1;
            _0.final_release = 1;
            _0.initial_timestamp = 1;
            _0.cliff_delta = 1;
            _0.final_delta = 1;
            _0.curve_type = CurveType::Linear { discrete_period: 1 };
        },
    );

    let curve_patch = guerrilla::patch4(CurveType::calculate_curve_return, |_0, _1, _2, _3| 42);

    (new_patch, change_patch, curve_patch)
}

pub fn patch_investment() -> (PatchGuard, PatchGuard) {
    let new_patch = guerrilla::patch3(Investment::new, |_0, _1, _2| Investment {
        account: _0,
        total_value: 0,
        withdrawn_value: 0,
        date_in: None,
    });

    let increase_withdraw_value_patch =
        guerrilla::patch2(Investment::increase_withdraw_value, |_0, _1| {
            _0.withdrawn_value += 1;
        });

    (new_patch, increase_withdraw_value_patch)
}
