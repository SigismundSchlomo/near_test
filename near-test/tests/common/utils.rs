use near_sdk::json_types::ValidAccountId;
use near_sdk_sim::{call, deploy, init_simulator, ContractAccount, UserAccount};
use near_test::ContractContract as TestContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NEAR_TEST_WASM_BYTES => "../res/near_test.wasm"
}

pub fn validate_account(account_id: String) -> ValidAccountId {
    ValidAccountId::try_from(account_id).unwrap()
}

const CONTRACT_ID: &str = "near_test";
const TOTAL_SUPPLY: u128 = 20_000_000_000_000_000_000_000_000_000; //20_000 TK
const TOTAL_STAKE: u128 = 25_000_000_000_000_000_000_000_000_000; //25_000 NEAR
const OWNER_ID: &str = "owner";

pub fn init() -> (UserAccount, ContractAccount<TestContract>) {
    let root = init_simulator(None);

    let contract = deploy!(
        contract: TestContract,
        contract_id: CONTRACT_ID,
        bytes: &NEAR_TEST_WASM_BYTES,
        signer_account: root,
        init_method: init(validate_account(OWNER_ID.into()), TOTAL_SUPPLY.into(), TOTAL_STAKE.into())
    );

    (root, contract)
}

pub fn register_account(root: &UserAccount, contract: &ContractAccount<TestContract>) {
    let result = call!(
        root,
        contract.storage_deposit(None, None),
        deposit = 1250000000000000000000 //minimal storage
    );
    println!("{:?}", result);
}
