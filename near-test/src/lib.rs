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

//TODO: Explore env::promise_results possibilities
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
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: String) -> U128;
}


#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen()]
impl Contract {
    pub fn get_whitelisted_tokens(&self) -> Promise {
        ext_ref_finance::get_whitelisted_tokens(
            &REF_EXCHANGE_ADDRESS.to_string(),
            0,
            20_000_000_000_000,
        )
        .then(ext_self::get_whitelisted_tokens_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000,
        ))
    }

    pub fn get_number_of_pools(&self) -> Promise {
        ext_ref_finance::get_number_of_pools(
            &REF_EXCHANGE_ADDRESS.to_string(),
            0,
            20_000_000_000_000,
        )
        .then(ext_self::get_number_of_pools_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000,
        ))
    }

    pub fn get_pools(&self, from_index: u64, limit: u64) -> Promise {
        ext_ref_finance::get_pools(
            from_index,
            limit,
            &REF_EXCHANGE_ADDRESS.to_string(),
            0,
            20_000_000_000_000,
        )
        .then(ext_self::get_pools_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000,
        ))
    }

    #[payable]
    pub fn swap(&mut self, actions: Vec<SwapAction>) -> Promise {
        ext_ref_finance::swap(
            actions,
            REFERRAL_ACCOUNT.to_string(),
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000,
        ).then(ext_self::swap_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000,
        ))
    }

    //TODO: Decide if I need it.

    // pub fn storage_deposit(
    //     &mut self,
    //     account_id: Option<ValidAccountId>,
    //     registration_only: Option<bool>,
    // ) -> Promise {
    //     let account_id = account_id
    //         .map(|a| a.into())
    //         .unwrap_or(env::predecessor_account_id());
    //     let registration_only = registration_only.unwrap_or(false);
    //     ext_ref_finance::storage_deposit(
    //         account_id,
    //         registration_only,
    //         &REF_EXCHANGE_ADDRESS.to_string(),
    //         env::attached_deposit(),
    //         20_000_000_000_000,
    //     )
    //     .then(ext_self::storage_deposit_callback(
    //         &env::current_account_id(),
    //         0,
    //         20_000_000_000_000,
    //     ))
    // }
}

#[cfg(test)]
mod test {}
