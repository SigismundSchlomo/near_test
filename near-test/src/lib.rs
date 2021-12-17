//! This is a smart contract just to try some features of near_sdk
//! There are some terrible decisions are made
//!
//! TODO: Check if used account_id is always valid

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise,
    PromiseResult,
};

near_sdk::setup_alloc!();

//Do we need to register an account?

const CODE: &[u8] = include_bytes!("../../res/fun_token.wasm");
const TOKEN_ADDRESS: &str = env!("TOKEN_ADDRESS");
pub const XCC_GAS: Gas = 20000000000000;

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    /// Retrieves token total supply from token smart contract and keeps user id to save it latter.
    fn get_ft_total_supply_with_caller_id(&self, caller_id: AccountId) -> (AccountId, U128);
    fn get_ft_balance_of_with_caller_id(&self, caller_id: AccountId) -> (AccountId, U128);
    fn internal_deposit(&mut self, account_id: AccountId, amount: Balance);
    fn internal_withdraw(&mut self, account_id: AccountId, amount: Balance);
}

#[ext_contract(ext_self)]
trait ExtSelf {
    /// Retrieves token total supply and calculates token price. Saves last in user's meta
    fn get_token_price_callback(&mut self) -> u128;
    /// Retrieves user's balance and saves to user's meta
    fn get_user_balance_callback(&mut self);
    /// Adds tokens to the contract
    fn add_to_total_deposit(&mut self, amount: Balance);
}

struct UsersMeta {
    /// token on the moment of user's request
    token_price: u128,
    /// token amount in user's request
    token_amount: Balance,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub total_deposit: Balance,
    pub token_account_id: AccountId,
    users_metas: LookupMap<AccountId, UsersMeta>,
}

fn calculate_tokens_amount(value: Balance, token_price: u128) -> u128 {
    value.checked_div(token_price).unwrap_or(0)
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            //possibly non persistent
            total_deposit: 0,
            token_account_id: TOKEN_ADDRESS.to_string(),
            users_metas: LookupMap::new("UsersMetaHashTable"),
        }
    }

    // TODO: fix
    #[private]
    pub fn deploy_token(&self) -> Promise {
        Promise::new(self.token_account_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(3_000_000_000_000_000_000_000_000)
            .deploy_contract(CODE.to_vec())
    }

    #[payable]
    pub fn stake(&mut self, amount: Balance) -> Promise {
        self.cache_token_amount(env::predecessor_account_id(), amount);
        // getTokenPrice
        ext_fungible_token::get_ft_total_supply_with_caller_id(
            env::predecessor_account_id(),
            &self.token_account_id,
            0,
            XCC_GAS,
        )
        .then(ext_self::get_token_price_callback(
            &env::current_account_id(),
            0,
            XCC_GAS,
        ))
        // mint tokens
        .then(ext_fungible_token::internal_deposit(
            env::predecessor_account_id(),
            calculate_tokens_amount(
                // Is env::predecessor_account_id returns expected value? Could return env::current_account_id because this is a callback...
                self.get_cached_token_amount(env::predecessor_account_id()),
                self.get_cached_token_price_of(env::predecessor_account_id()),
            ),
            &self.token_account_id,
            0,
            XCC_GAS,
        ))
        // add to total stake
        .then(ext_self::add_to_total_deposit(
            calculate_tokens_amount(
                self.get_cached_token_amount(env::predecessor_account_id()),
                self.get_cached_token_price_of(env::predecessor_account_id()),
            ),
            &self.token_account_id,
            0,
            XCC_GAS,
        ))
        // .then(ext_fungible_token::internal_withdraw(
        //     env::predecessor_account_id(),
        //     calculate_tokens_amount(
        //         amount,
        //         self.get_cached_token_price_of(env::predecessor_account_id()),
        //     ),
        //     &self.token_account_id,
        //     0,
        //     XCC_GAS
        // ));
    }

    // #[payable]
    // pub fn unstake(&self)

    fn cache_token_amount(&mut self, &account_id: AccountId, amount: Balance) {
        if let Some(mut meta) = self.users_metas.get(account_id) {
            meta.token_amount = amount
        } else {
            env::panic(b"ERR_AMOUNT_OWERFLOW")
        }
    }

    fn get_cached_token_amount(&self, &account_id: AccountId) -> Balance {
        if let Some(meta) = self.users_metas.get(account_id) {
            meta.token_amount
        } else {
            env::panic(b"ERR_RETRIVING_TOKEN_AMOUNT")
        }
    }

    fn get_cached_token_price_of(&self, &account_id: AccountId) -> u128 {
        if let Some(UsersMeta) = self.users_metas.get(&account_id) {
            return UsersMeta.token_price;
        } else {
            1
        }
    }

    #[private]
    pub fn add_to_total_deposit(&mut self, amount: Balance) {
        if let Some(deposit) = self.total_deposit.checked_add(amount) {
            self.total_deposit = deposit
        } else {
            env::panic(b"ERR_ADDING_TOTAL_DEPOSIT")
        }
    }

    //TODO: move boilerplate
    #[private]
    pub fn get_token_price_callback(&mut self) {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok((account_id, total_supply)) =
                    near_sdk::serde_json::from_slice::<(AccountId, U128)>(&val)
                {
                    // Unsafe!!! Move to the separate function and make safe
                    let mut users_meta = self.users_metas.get(&account_id).unwrap();
                    let token_price = self.get_token_price(total_supply);
                    users_meta.token_price = token_price.into();
                    self.users_metas.insert(&account_id, &users_meta);
                } else {
                    env::panic(b"ERR_TOKEN_PRICE_WRONG_VAL_RECEIVED");
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_TOKEN_PRICE_CALL_FAILED"),
        }
    }

    fn get_token_price(&self, val: U128) -> u128 {
        let total_supply: Balance = val.into();
        if total_supply > 0 {
            self.total_deposit.checked_div(total_supply).unwrap_or(1)
        } else {
            1
        }
    }
}
