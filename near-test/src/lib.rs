use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PromiseResult};

use crate::ref_utils::{PoolInfo, SwapAction};

mod callbacks;
mod ref_utils;

near_sdk::setup_alloc!();

pub const GAS: Gas = 300_000_000_000_000;
pub const REFERRAL_ACCOUNT: &str = "kuznietsov.testnet";
pub const REF_EXCHANGE_ADDRESS: &str = "exchange.ref-dev.testnet";

//TODO:Most integration functionality should be wrote into frontend code. For example in wrapper.
//TODO:Decide that should be added in smart contract code

#[ext_contract(ext_ref_finance)]
pub trait RefFinance {
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: String) -> U128;
}

#[ext_contract(ext_self)]
pub trait Callbacks {
    fn swap_single_callback() -> U128;
    // fn swap_sing_callback(#[callback] val: U128) -> U128; - this is much more simple but it very error prone will be replaced in near sdk soon
}

#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen()]
impl Contract {


    #[init(ignore_state)]
    pub fn reinit() -> Self {
        Self {}
    }

    /// Executes ref finances swap with exactly one action
    pub fn swap_single(
        &mut self,
        pool_id: U64,
        token_in: AccountId,
        amount_in: U128,
        token_out: AccountId,
        min_amount_out: U128,
    ) -> Promise {
        let action = SwapAction {
            pool_id: pool_id.0,
            token_in,
            amount_in: Some(amount_in),
            token_out,
            min_amount_out,
        };
        ext_ref_finance::swap(
            vec![action],
            REFERRAL_ACCOUNT.to_string(),
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000, //TODO: Calculate gas amount required to execute callback
        )
        .then(ext_self::swap_single_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000, //TODO: Calculate gas amount required to execute callback
        ))
    }
}

#[cfg(test)]
mod test {}
