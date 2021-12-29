use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::{env, PromiseOrValue, ext_contract, near_bindgen};

use crate::contracts_calls::{ext_token};
use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        // Do some accounting work
        // call token
        let promise = ext_token::ft_transfer_call(
            "exchange.ref-dev.testnet".to_string(),
            amount,
            None,
            msg,
            &token_in,
            1,
            50_000_000_000_000
        );
        PromiseOrValue::Value(U128(0))
    }
}
