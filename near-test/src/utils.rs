use std::env;
use near_sdk::AccountId;
use near_sdk::json_types::ValidAccountId;

pub fn validate_account_id(account_id: AccountId) -> ValidAccountId {
    if let Ok(valid_id) = ValidAccountId::try_from(account_id) {
        valid_id
    } else {
        env::panic(b"ERR_INVALID_ACCOUNT")
    }
}
