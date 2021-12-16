use near_sdk_sim::{init_simulator, to_yocto, STORAGE_AMOUNT, UserAccount, deploy, ContractAccount};
use near_test::ContractContract as TestContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NEAR_TEST_WASM_BYTES => "../res/near_test.wasm"
}

const CONTRACT_ID: &str = "near_test";

pub fn init() -> (UserAccount, ContractAccount<TestContract>) {
    let root = init_simulator(None);

    let contract = deploy!(
        contract: TestContract,
        contract_id: CONTRACT_ID,
        bytes: &NEAR_TEST_WASM_BYTES,
        signer_account: root
    );


    (root, contract)
}
