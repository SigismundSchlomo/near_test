use crate::ref_utils::SwapAction;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{ext_contract, AccountId, PromiseOrValue};

//The seed format is contract_address@pool_id
pub(crate) type SeedId = String;

#[ext_contract(ext_token)]
pub trait ExtTokens {
    fn ft_transfer_call(
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ref_finance)]
pub trait ExtRefFinance {
    fn storage_deposit(
        &mut self,
        account_id: Option<ValidAccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: String) -> U128;
    fn add_liquidity(&mut self, pool_id: u64, amounts: Vec<U128>);
    fn mft_transfer_call(
        &mut self,
        token_id: String,
        receiver_id: String,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ref_farming)]
pub trait ExtRefFarming {
    fn withdraw_seed(&mut self, seed_id: SeedId, amount: U128);
}
