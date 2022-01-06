use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, PromiseResult};

use crate::contracts_calls::ext_ref_finance;
use crate::*;

#[ext_contract(ext_self)]
pub trait Callbacks {
    fn swap_callback() -> U128;
    fn create_position_callback(
        &mut self,
        pool_id: u64,
        amount_in_pool: U128,
        ref_finance_address: String,
        deposit: Balance,
        gas: u128,
    );
    fn exchange_callback_post_withdraw(
        &mut self,
        token_id: AccountId,
        sender_id: AccountId,
        amount: U128,
    );
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
                    env::panic_str("ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
        }
    }

    #[private]
    pub fn create_position_callback(
        &mut self,
        pool_id: u64,
        amount_in_pool: U128,
        ref_finance_address: String,
        deposit: Balance,
        gas: Gas,
    ) {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(exchanged) = near_sdk::serde_json::from_slice::<U128>(&val) {
                    ext_ref_finance::add_liquidity(
                        pool_id,
                        vec![amount_in_pool, exchanged],
                        validate_account_id(ref_finance_address),
                        deposit,
                        gas,
                    );
                } else {
                    env::panic_str("ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
        }
    }
}
