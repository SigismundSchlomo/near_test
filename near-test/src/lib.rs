use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue, Promise};

use crate::utils::*;

mod utils;
mod token_receiver;
mod ref_utils;
mod storage_impl;

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
        Self::init(owner_id, token_total_supply )
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
