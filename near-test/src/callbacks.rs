use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PromiseResult};

#[near_bindgen]
impl Contract {
    #[private]
    pub fn swap_single_callback(&mut self) -> U128 {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(amount) = near_sdk::serde_json::from_slice::<U128>(&val) {
                    amount
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}
