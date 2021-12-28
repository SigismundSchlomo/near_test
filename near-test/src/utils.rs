use near_sdk::{AccountId, Balance};
use near_sdk::json_types::ValidAccountId;
use near_sdk::env;

pub fn validate_account_id(account_id: AccountId) -> ValidAccountId {
    if let Ok(valid_id) = ValidAccountId::try_from(account_id) {
        valid_id
    } else {
        env::panic(b"ERR_INVALID_ACCOUNT")
    }
}

//floating point problem
// pub fn convert_near_to_yocto(amount: f64) -> Balance {
//     let one_near = 10u128.pow(24) as f64;
//     amount * one_near
// }
