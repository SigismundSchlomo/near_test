use near_contract_standards::storage_management::StorageBalance;
use near_sdk::{AccountId, env, PromiseResult};
use near_sdk::json_types::U128;
use crate::{Contract, PoolInfo};

// Если информация не будет нужна в смарт контракте - гораздо дешевле запрашивать информацию на фронтенд напрямую из контрактов ref finance
#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn get_whitelisted_tokens_callback() -> Vec<AccountId>;
    fn get_number_of_pools_callback() -> u64;
    fn get_pools_callback() -> Vec<PoolInfo>;
    fn storage_deposit_callback() -> StorageBalance;
    fn swap_callback() -> U128;
}

//TODO: Try callback attribute
#[near_bindgen]
impl Contract {

    #[private]
    pub fn get_whitelisted_tokens_callback(&mut self) -> Vec<AccountId> {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(whitelist) = near_sdk::serde_json::from_slice::<Vec<AccountId>>(&val) {
                    whitelist
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    #[private]
    pub fn get_number_of_pools_callback(&mut self) -> u64 {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(number) = near_sdk::serde_json::from_slice::<u64>(&val) {
                    number
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    #[private]
    pub fn get_pools_callback(&mut self) -> Vec<PoolInfo> {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(results) = near_sdk::serde_json::from_slice::<Vec<PoolInfo>>(&val) {
                    results
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
    // TODO: Decide if i need it

    // #[private]
    // pub fn storage_deposit_callback(&mut self) -> StorageBalance {
    //     match env::promise_result(0) {
    //         PromiseResult::NotReady => unreachable!(),
    //         PromiseResult::Successful(val) => {
    //             if let Ok(storage_balance) = near_sdk::serde_json::from_slice::<StorageBalance>(&val) {
    //                 storage_balance
    //             } else {
    //                 env::panic(b"ERR_WRONG_VALRECEIVED")
    //             }
    //         },
    //         PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED")
    //     }
    // }

    #[private]
    pub fn swap_callback() -> U128 {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(swap_result) = near_sdk::serde_json::from_slice::<U128>(&val) {
                    swap_result
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED")
        }
    }
}
