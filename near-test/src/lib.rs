//! This is a smart contract just to try some features of near_sdk


use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{
    AccountId,
    env,
    ext_contract,
    Gas,
    near_bindgen,
    Promise,
    PromiseOrValue,
    PromiseResult,
};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::ValidAccountId;

near_sdk::setup_alloc!();

pub const GAS: Gas = 50_000_000_000_000;

// pub const REF_EXCHANGE_ADDRESS: &str = if cfg!(feature = "main_net") {
//     "v2.ref-finance.near".to_string()
// } else {
//     "ref-finance.testnet".to_string()
// };

pub const REF_EXCHANGE_ADDRESS: &str = "exchange.ref-dev.testnet";

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
        ext_ref_finance::get_whitelisted_tokens(REF_EXCHANGE_ADDRESS.to_string(), 0, GAS).then(
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
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::serde::de::Unexpected::Str;

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "some.cool.testnet".to_string(),
            signer_account_id: "some.signer.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2, 3],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![1, 2, 3],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 10,
        }
    }

    #[test]
    fn set_message() {
        let context = get_context("hello.testnet".to_string(), 0);
        testing_env!(context);
        let mut contract = Contract { message: String::new() };
        let message = "Hello".to_string();
        contract.set_message(message.clone());
        assert_eq!(contract.message, message, "Expected message to contain \"Hello\"");
    }

    #[test]
    fn get_message() {
        let context = get_context("hello.testnet".to_string(), 0);
        testing_env!(context);
        let mut contract = Contract { message: String::new() };
        let message = "Hello".to_string();
        contract.set_message(message.clone());
        assert_eq!(contract.get_message(), message, "Expected to return \"Hello\"")
    }
}
