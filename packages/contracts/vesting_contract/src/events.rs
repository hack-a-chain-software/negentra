use near_sdk::{log};
use serde_json::{json};

const STANDARD_NAME: &str = "HaC_vesting";
const STANDARD_VERSION: &str = "1.0.0";

fn log_basic_event_format(standard: &str, version: &str, event_type: &str, data_vec: Vec<&str>) {
    log!("EVENT_JSON:{}", &json!({
        "standard": standard, 
        "version": version, 
        "event": event_type,
        "data": data_vec
    }).to_string())
}

pub fn event_create_schema(category: &str, total_quantity: &str) {
    let event_type = "create_schema";
    let event_data = &json!({
        "category": category,
        "total_quantity": total_quantity,
    }).to_string();
    log_basic_event_format(STANDARD_NAME, STANDARD_VERSION, event_type, vec!(event_data));
}