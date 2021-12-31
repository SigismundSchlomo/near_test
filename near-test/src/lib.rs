use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::callbacks::ext_self;
use crate::contracts_calls::ext_ref_finance;
use crate::ref_utils::SwapAction;

mod callbacks;
mod contracts_calls;
mod ref_utils;
mod storage_impl;
mod token_receiver;
mod utils;

pub const REFERRAL_ACCOUNT: &str = "kuznietsov.testnet";
pub const REF_EXCHANGE_ADDRESS: &str = "ref-finance-101.testnet";
pub const REF_FARMING_ADDRESS: &str = "v2.ref-farming.testnet";
const ONE_TK_IN_YOCTO: u128 = 10u128.pow(24); // Based on near. Symbolize one TK in yoctoTK

near_sdk::setup_alloc!();

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //FT has 24 decimals
    token: FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: ValidAccountId, token_total_supply: U128) -> Self {
        let mut this = Self {
            token: FungibleToken::new(b"t".to_vec()),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token
            .internal_deposit(owner_id.as_ref(), token_total_supply.into());
        this
    }

    #[init(ignore_state)]
    pub fn reinit(owner_id: ValidAccountId, token_total_supply: U128) -> Self {
        Self::init(owner_id, token_total_supply)
    }

    pub fn swap_single(
        &mut self,
        pool_id: U64, //TODO: Change to primitive ???
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
            20_000_000_000_000, //TODO: Calculate exact gas amount required to execute callback
        )
        .then(ext_self::swap_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000, //TODO: Calculate exact gas amount required to execute callback
        ))
    }

    pub fn swap(&mut self, actions: Vec<SwapAction>) -> Promise {
        ext_ref_finance::swap(
            actions,
            REFERRAL_ACCOUNT.to_string(),
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000,
        )
        .then(ext_self::swap_callback(
            &env::current_account_id(),
            0,
            20_000_000_000_000,
        ))
    }

    #[payable]
    pub fn add_liquidity(&mut self, pool_id: u64, amounts: Vec<U128>) -> Promise {
        ext_ref_finance::add_liquidity(
            pool_id,
            amounts,
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000,
        )
    }

    #[payable]
    pub fn create_position(
        &mut self,
        pool_id: u64,
        amount_in_pool: U128,
        token_in: AccountId,
        amount_in: U128,
        token_out: AccountId,
        min_amount_out: U128,
    ) -> Promise {
        let action = SwapAction {
            pool_id,
            token_in,
            amount_in: Some(amount_in),
            token_out,
            min_amount_out
        };
        ext_ref_finance::swap(
            vec![action],
            REFERRAL_ACCOUNT.to_string(),
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(), //Check if deposit works as expected
            20_000_000_000_000
        ).then(ext_self::create_position_callback(
            pool_id,
            amount_in_pool,
            REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000,
            &env::current_account_id(),
            0,
            50_000_000_000_000,
        ))
    }

    //User registration left on front end
    #[payable]
    pub fn stake_to_farm(&mut self, pool_id: u64, amount: U128) -> Promise{
        let token_id = format!(":{}", pool_id);
        let receiver_id = REF_FARMING_ADDRESS.to_string();
        let memo = None;
        let msg = "".to_string();
        ext_ref_finance::mft_transfer_call(
            token_id,
            receiver_id,
            amount,
            memo,
            msg,
            &REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(), //Check if deposit works as expected
            100_000_000_000_000
        )
    }

}

/// Internal methods
impl Contract {
    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

//
// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//
// }
