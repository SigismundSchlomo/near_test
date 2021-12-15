//! This is a smart contract just to try some features of near_sdk


use std::intrinsics::unreachable;
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{
    AccountId,
    env,
    ext_contract,
    Gas,
    near_bindgen,
    Promise,
    PromiseOrValue,
    PromiseResult
};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::ValidAccountId;

near_sdk::setup_alloc!();

pub const GAS: Gas = 50_000_000_000_000;

// pub const REF_EXCHANGE_ADDRESS: AccountId = if cfg!(feature = "main_net") {
//     "v2.ref-finance.near".to_string()
// } else {
//     "ref-finance.testnet".to_string()
// };

pub const REF_EXCHANGE_ADDRESS: AccountId = "exchange.ref-dev.testnet".to_string();

#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    message: String,
}

#[ext_contract(ext_ref_finance)]
pub trait RefFinance {
    fn storage_deposit(&mut self, account_id: Option<ValidAccountId>, registration_only: Option<bool>) -> StorageBalance;
    fn get_whitelisted_tokens(&self) -> Vec<AccountId>;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn handle_get_whitelisted_tokens_result() -> Vec<AccountId>;
}

#[near_bindgen()]
impl Contract {

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_whitelisted_tokens(&self) -> Promise {
        ext_ref_finance::get_whitelisted_tokens(&REF_EXCHANGE_ADDRESS, 0, GAS).then(
            ext_self::handle_get_whitelisted_tokens_result(
                &env::current_account_id(),
                0,
                GAS,
            ),
        )
    }

    #[private]
    pub fn handle_get_whitelisted_tokens_result(&mut self) -> Vec<AccountId> {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(whitelist) = near_sdk::serde_json::from_slice::<Vec<AccountId>>(&val) {
                    whitelist
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}

