use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PromiseResult};

use crate::ref_utils::{PoolInfo, SwapAction};

mod callbacks;
mod ref_utils;

near_sdk::setup_alloc!();

pub const GAS: Gas = 300_000_000_000_000;
pub const REFERRAL_ACCOUNT: &str = "kuznietsov.testnet";
pub const REF_EXCHANGE_ADDRESS: &str = "exchange.ref-dev.testnet";

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
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: String) -> U128;
}


#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen()]
impl Contract {

}

#[cfg(test)]
mod test {}
