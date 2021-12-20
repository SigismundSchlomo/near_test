//! This is a smart contract just to try some features of near_sdk

use std::intrinsics::unreachable;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Gas, Promise, PromiseOrValue, PromiseResult,
};
use crate::ref_utils::PoolInfo;

mod ref_utils;

near_sdk::setup_alloc!();

pub const GAS: Gas = 300_000_000_000_000;

// pub const REF_EXCHANGE_ADDRESS: &str = if cfg!(feature = "main_net") {
//     "v2.ref-finance.near".to_string()
// } else {
//     "ref-finance.testnet".to_string()
// };

pub const REF_EXCHANGE_ADDRESS: &str = "exchange.ref-dev.testnet";
//TODO: Move some code to other modules
//TODO: Explore env::promise_results possibilities
//TODO: Write some more pool related and swap related code
//TODO: explore available tokens - get whitelisted is very useful for this one
//TODO: Discuss about fat frontend strategy to simplify smart contract development and make whole project cheaper

//TODO: Move ref finance related code to the separate module
#[ext_contract(ext_ref_finance)]
pub trait RefFinance {
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
    fn get_whitelisted_tokens(&self) -> Vec<AccountId>;
    fn get_number_of_pools(&self) -> u64;
    fn get_pools(&self, from_index: u64, limit: u64) -> Vec<PoolInfo>;
}

// Если информация не будет нужна в смарт контракте - гораздо дешевле запрашивать информацию на фронтенд напрямую из контрактов ref finance
#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn get_whitelisted_tokens_callback() -> Vec<AccountId>;
    fn get_number_of_pools_callback() -> u64;
    fn get_pools_callback() -> Vec<PoolInfo>;
}

#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen()]
impl Contract {
    pub fn get_whitelisted_tokens(&self) -> Promise {
        ext_ref_finance::get_whitelisted_tokens(&REF_EXCHANGE_ADDRESS.to_string(), 0, 20_000_000_000_000)
            .then(ext_self::get_whitelisted_tokens_callback(
                &env::current_account_id(),
                0,
                20_000_000_000_000,
            ))
    }

    pub fn get_number_of_pools(&self) -> Promise {
        ext_ref_finance::get_number_of_pools(&REF_EXCHANGE_ADDRESS.to_string(), 0, 20_000_000_000_000)
            .then(ext_self::get_number_of_pools_callback(
                &env::current_account_id(),
                0,
                20_000_000_000_000,
            ))
    }

    pub fn get_pools(&self, from_index: u64, limit: u64) -> Promise {
        ext_ref_finance::get_pools(from_index, limit, &REF_EXCHANGE_ADDRESS.to_string(), 0, 20_000_000_000_000)
            .then(ext_self::get_pools_callback(
                &env::current_account_id(),
                0,
                20_000_000_000_000,
            ))
    }

    //TODO: Move callbacks in separate module

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

    //TODO: Try to user callback attribute
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

    pub fn get_pools_callback(&mut self) -> Vec<PoolInfo> {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(results) = near_sdk::serde_json::from_slice::<Vec<PoolInfo>>(&val) {
                    results
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED")
        }
    }
}

#[cfg(test)]
mod test {

}
