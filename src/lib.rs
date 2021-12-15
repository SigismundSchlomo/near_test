//! This is a smart contract just to try some features of near_sdk


use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{AccountId, env, near_bindgen, Promise, PromiseOrValue};
use near_sdk::collections::{LookupMap, Vector};

near_sdk::setup_alloc!();

const REF_EXCHANGE_ADDRESS: AccountId = if cfg!(feature = "main_net") {
    "ref-finance.near".to_string()
} else {
    "ref-finance.testnet".to_string()
};


#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    user_accounts_values: LookupMap<AccountId, u128>,
}

impl Contract {

    pub fn set_value(&mut self, &value: u128) {
        let sender_id = env::predecessor_account_id();
        self.user_accounts_values.insert(AccountId, value);
    }


}
