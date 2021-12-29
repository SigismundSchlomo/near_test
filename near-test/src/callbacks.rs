use near_sdk::{env, PromiseResult, ext_contract, near_bindgen};
use near_sdk::json_types::U128;

use crate::*;

#[ext_contract(ext_self)]
pub trait Callbacks {
    fn swap_callback() -> U128;
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn swap_callback(&mut self) -> U128 {
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