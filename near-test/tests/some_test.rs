use crate::common::utils::{init, register_account};
use near_sdk_sim::{call, to_yocto, view};

pub mod common;

#[test]
fn call_stake() {
    let (root, contract) = init();

    register_account(&root, &contract);

    let result = call!(root, contract.stake(), deposit = to_yocto("1"));

    println!("Gas burned: {:?}", result.gas_burnt());
    assert!(result.is_ok());

    let result: String = view!(contract.ft_total_supply()).unwrap_json();

    println!("Total stake: {:?}", result);
}
