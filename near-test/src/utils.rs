use near_sdk::{AccountId, Balance};
use near_sdk::env;

pub fn validate_account_id(string: String) -> AccountId {
    if let Ok(account_ud) = AccountId::try_from(string) {
        account_ud
    } else {
        env::panic_str("ACCOUNT_ID_IS_INVALID")
    }
}