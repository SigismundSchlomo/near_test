use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::{U128, ValidAccountId};
use near_sdk::{AccountId, PromiseOrValue, ext_contract};
use crate::ref_utils::SwapAction;

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
}

