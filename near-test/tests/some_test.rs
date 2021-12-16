use near_sdk::serde_json::json;
use near_sdk_sim::{DEFAULT_GAS, view, call};

use crate::common::utils::init;

pub mod common;

#[test]
fn simulate_some_view() {
    let (root, contract) = init();

    let actual: String = view!(
        contract.get_message()
    ).unwrap_json();

    assert_eq!("".to_string(), actual);
}

#[test]
fn simulate_some_change() {
    let (root, contract) = init();
    let result = call!(
        root,
        contract.set_message("Hello".to_string()),
        deposit = 1
    );

    assert!(result.is_ok());
    let after: String = view!(
        contract.get_message()
    ).unwrap_json();

    assert_eq!("Hello".to_string(), after);
}
