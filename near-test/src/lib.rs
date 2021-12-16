//! This is a smart contract just to try some features of near_sdk


use near_contract_standards::storage_management::StorageBalance;
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::{AccountId, Balance, env, ext_contract, Gas, near_bindgen, Promise, PromiseOrValue, PromiseResult};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk_sim::config::ActionCosts::deploy_contract;

near_sdk::setup_alloc!();

const CODE: &[u8] = include_bytes!("../res/fun_token.wasm");
const TOKEN_ADDRESS: &str = env!("TOKEN_ADDRESS");
pub const XCC_GAS: Gas = 20000000000000;

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn get_ft_total_supply_with_caller_id(&self, caller_id: AccountId) -> (AccountId, U128);
    fn get_ft_balance_of_with_caller_id(&self, caller_id: AccountId) -> (AccountId, U128);
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn get_token_price_callback(&mut self) -> u128;
    fn get_user_balance_callback(&mut self);
}

struct UsersMeta {
    /// token on the moment of user's request
    token_price: u128,
    /// users balance on the moment of user's request
    token_balance: Balance,

}

#[near_bindgen()]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub total_deposit: Balance,
    pub token_account_id: AccountId,
    users_metas: LookupMap<AccountId, UsersMeta>,
}

#[near_bindgen()]
impl Contract {

    #[init]
    pub fn new() -> Self {
        deploy_token();
        Self {
            //possibly non persistent
            total_deposit: 0,
            token_account_id: TOKEN_ADDRESS.to_string(),
            users_metas: LookupMap::new("UsersMetaHashTable"),
        }
    }

    fn deploy_token(&self) -> Promise {
        Promise::new(self.token_account_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(3_000_000_000_000_000_000_000_000)
            .deploy_contract(CODE.to_vec())
    }

    #[payable]
    pub fn stake(&self) {
        // 1. getTokenPrice
        ext_fungible_token::get_ft_total_supply_with_caller_id(
            env::predecessor_account_id(),
            &self.token_account_id,
            0,
            XCC_GAS
        ).then(ext_self::get_token_price_callback(
            &env::current_account_id(),
            0,
            XCC_GAS
            // 2. getToken for user
        )).then(ext_fungible_token::get_ft_balance_of_with_caller_id(
            env::predecessor_account_id(),
            &self.token_account_id,
            0,
            XCC_GAS
        )).then(ext_self::get_user_balance_callback(
            &env::current_account_id(),
            0,
            XCC_GAS
        ));
        // 3. mint tokens
        // 4. add to total stake
    }

    #[private]
    pub fn get_user_balance_callback(&mut self) {
        match env::promise_result(1) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok((account_id, account_balance)) = near_sdk::serde_json::from_slice::<(AccountId, U128)>(&val) {
                    let mut users_meta = self.users_metas.get(&account_id).unwrap();
                    users_meta.token_balance = account_balance.into();
                    self.users_metas.insert(&account_id, &users_meta);
                } else {
                    env::panic(b"ERR_BALANCE_WRONG_VAL_RECEIVED");
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_BALANCE_CALL_FAILER")
        }
    }

    #[private]
    pub fn get_token_price_callback(&mut self) {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok((account_id, total_supply)) = near_sdk::serde_json::from_slice::<(AccountId, U128)>(&val) {
                    let mut users_meta = self.users_metas.get(&account_id).unwrap();
                    let token_price = self.get_token_price(total_supply);
                    users_meta.token_price = token_price.into();
                   self.users_metas.insert(&account_id, &users_meta);
                } else {
                    env::panic(b"ERR_TOKEN_PRICE_WRONG_VAL_RECEIVED");
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_TOKEN_PRICE_CALL_FAILED"),
        }
    }

    fn get_token_price(&self, val: U128) -> u128 {
        let total_supply: Balance = val.into();
        if total_supply > 0 {
            self.total_deposit / total_supply
        } else {
            1
        }
    }
}

