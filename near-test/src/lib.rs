use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, U64};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, Gas};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use crate::account::Account;

use crate::callbacks::ext_self;
use crate::contracts_calls::{ext_ref_farming, ext_ref_finance, SeedId};
use crate::ref_utils::SwapAction;
use crate::utils::validate_account_id;

mod account;
mod callbacks;
mod contracts_calls;
mod ref_utils;
mod storage_impl;
mod token_receiver;
mod utils;


#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    AccountTokens { account_id: AccountId },
    WhitelistedTokens,
}

pub const REFERRAL_ACCOUNT: &str = "kuznietsov.testnet";
pub const REF_EXCHANGE_ADDRESS: &str = "ref-finance-101.testnet";
pub const REF_FARMING_ADDRESS: &str = "v2.ref-farming.testnet";
const ONE_TK_IN_YOCTO: u128 = 10u128.pow(24); // Based on near. Symbolize one TK in yoctoTK

//TODO: Add logic to work with whitelisted tokens
//TODO: ADD file with error constance like in ref finance
//TODO: Use require instead of assert


//Some crazy constants
pub const TWO_HUNDREDS_TGAS: Gas = Gas(200_000_000_000_000);
pub const HUNDRED_TGAS: Gas = Gas(100_000_000_000_000);
pub const FIFTY_TGAS: Gas = Gas(50_000_000_000_000);
pub const TWENTY_TGAS: Gas = Gas(20_000_000_000_000);

near_sdk::setup_alloc!();

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //FT has 24 decimals
    token: FungibleToken,
    owner_id: AccountId,
    accounts: LookupMap<AccountId, Account>,
    whitelisted_tokens: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId, token_total_supply: U128) -> Self {
        let mut this = Self {
            token: FungibleToken::new(b"t".to_vec()),
            owner_id: owner_id.clone(),
            accounts: LookupMap::new(StorageKey::Accounts),
            whitelisted_tokens: UnorderedSet::new(StorageKey::WhitelistedTokens)
        };
        this.token.internal_register_account(&owner_id);
        this.token
            .internal_deposit(&owner_id, token_total_supply.into());
        this
    }

    #[init(ignore_state)]
    pub fn reinit(owner_id: AccountId, token_total_supply: U128) -> Self {
        Self::init(owner_id, token_total_supply)
    }

    pub fn swap_single(
        &mut self,
        pool_id: u64,
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
            min_amount_out,
        };
        ext_ref_finance::swap(
            vec![action],
            REFERRAL_ACCOUNT.to_string(),
            validate_account_id(REF_EXCHANGE_ADDRESS.to_string()),
            env::attached_deposit(),
            TWENTY_TGAS
        )
        .then(ext_self::swap_callback(
            env::current_account_id(),
            0,
            TWENTY_TGAS
        ))
    }

    #[payable]
    pub fn swap(&mut self, actions: Vec<SwapAction>) -> Promise {
        ext_ref_finance::swap(
            actions,
            REFERRAL_ACCOUNT.to_string(),
            validate_account_id(REF_EXCHANGE_ADDRESS.to_string()),
            env::attached_deposit(),
            TWENTY_TGAS
        )
        .then(ext_self::swap_callback(
            env::current_account_id(),
            0,
            TWENTY_TGAS
        ))
    }

    #[payable]
    pub fn add_liquidity(&mut self, pool_id: u64, amounts: Vec<U128>) -> Promise {
        ext_ref_finance::add_liquidity(
            pool_id,
            amounts,
            validate_account_id(REF_EXCHANGE_ADDRESS.to_string()),
            env::attached_deposit(),
            TWENTY_TGAS
        )
    }

    //Add liquidity is self balancing, but we should expect that amounts will be same as returned from swap
    //amount in will be divided in half. One half will be exchanged nad other half will be added to the pool with exchanged tokens
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
            min_amount_out,
        };
        ext_ref_finance::swap(
            vec![action],
            REFERRAL_ACCOUNT.to_string(),
            validate_account_id(REF_EXCHANGE_ADDRESS.to_string()),
            env::attached_deposit(), //Check if deposit works as expected
            TWENTY_TGAS
        )
        .then(ext_self::create_position_callback(
            pool_id,
            amount_in_pool,
            REF_EXCHANGE_ADDRESS.to_string(),
            env::attached_deposit(),
            20_000_000_000_000,
            env::current_account_id(),
            0,
            FIFTY_TGAS
        ))
    }

    //User registration left on front end
    #[payable]
    pub fn stake_to_farm(&mut self, pool_id: u64, amount: U128) -> Promise {
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
            validate_account_id(REF_EXCHANGE_ADDRESS.to_string()),
            env::attached_deposit(), //Check if deposit works as expected
            HUNDRED_TGAS
        )
    }

    #[payable]
    pub fn unstake_from_farm(&mut self, pool_id: u64, amount: U128) -> Promise {
        let seed_id: SeedId = format!("{}@{}", REF_EXCHANGE_ADDRESS, pool_id);
        ext_ref_farming::withdraw_seed(
            seed_id,
            amount,
            validate_account_id(REF_FARMING_ADDRESS.to_string()),
            env::attached_deposit(),
            TWO_HUNDREDS_TGAS
        )
    }

    pub fn claim_reward(&mut self, pool_id: u64) -> Promise {
        let seed_id: SeedId = format!("{}@{}", REF_EXCHANGE_ADDRESS, pool_id);
        ext_ref_farming::claim_reward_by_seed(
            seed_id,
            validate_account_id(REF_FARMING_ADDRESS.to_string()),
            env::attached_deposit(),
            TWO_HUNDREDS_TGAS
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
