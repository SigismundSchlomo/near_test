use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

use crate::utils::*;

mod utils;
const ONE_TK_IN_YOCTO: u128 = 1_000_000_000_000_000_000_000_000; //10u128.pow(24). Based on near. Symbolize one TK in yoctoTK

near_sdk::setup_alloc!();

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //FT has 24 decimals
    token: FungibleToken,
    total_stake: Balance,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: ValidAccountId, token_total_supply: U128, total_stake: U128) -> Self {
        let mut this = Self {
            token: FungibleToken::new(b"t".to_vec()),
            total_stake: total_stake.into(),
        };
        this.token.internal_register_account(owner_id.as_ref());
        this.token
            .internal_deposit(owner_id.as_ref(), token_total_supply.into());
        this
    }

    #[init(ignore_state)]
    pub fn reinit(owner_id: ValidAccountId, token_total_supply: U128, total_stake: U128) -> Self {
        Self::init(owner_id, token_total_supply, total_stake)
    }

    // Should I stake near or our token?
    #[payable]
    pub fn stake(&mut self) {
        let token_price = self.get_token_price();
        let tokens = env::attached_deposit() / token_price; // attached deposit is delivered in yoctoNEAR
        self.token
            .internal_deposit(&env::predecessor_account_id(), tokens);
        self.increment_stake(tokens);
    }

    pub fn unstake(&mut self, tokens: U128) {
        let user_balance: u128 = self
            .token
            .ft_balance_of(validate_account_id(env::predecessor_account_id()))
            .into();

        let tokens: u128 = tokens.into();

        assert!(tokens <= user_balance, "Sender has not enough tokens");
        let token_price = self.get_token_price();
        let deposit = if let Some(deposit) = token_price.checked_mul(tokens) {
            deposit
        } else {
            env::panic(b"ERR_DEPOSIT_OVERFLOW");
        };
        assert!(
            deposit <= self.total_stake,
            "Total stake is less then deposit"
        );

        self.token
            .internal_withdraw(&env::predecessor_account_id(), tokens);
        self.decrement_stake(deposit.into());
        self.token.internal_transfer(
            &env::current_account_id(),
            &env::predecessor_account_id(),
            deposit,
            None,
        );
    }
}

/// Internal methods
impl Contract {
    fn get_token_price(&self) -> Balance {
        if self.token.total_supply > 0 {
            self.total_stake / self.token.total_supply
        } else {
            ONE_TK_IN_YOCTO
        }
    }

    fn decrement_stake(&mut self, amount: u128) {
        if let Some(stake) = self.total_stake.checked_sub(amount) {
            self.total_stake = stake;
        } else {
            env::panic(b"ERR_TOTAL_STAKE_OVERFLOW_OCCURED")
        }
    }

    fn increment_stake(&mut self, amount: u128) {
        if let Some(stake) = self.total_stake.checked_add(amount) {
            self.total_stake = stake;
        } else {
            env::panic(b"ERR_TOTAL_STAKE_OVERFLOW_OCCURED");
        }
    }

    //TODO: Build in methods. Research why we need them
    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};
    use std::ops::Add;

    use super::*;

    const TOTAL_SUPPLY: Balance = 20_000_000_000_000_000_000_000_000_000; //20_000 TK
    const TOTAL_STAKE: Balance = 25_000_000_000_000_000_000_000_000_000; //25_000 NEAR

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
        assert_eq!(contract.total_stake, TOTAL_STAKE);
    }

    #[test]
    fn test_reinit() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::reinit(accounts(1), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
        assert_eq!(contract.total_stake, TOTAL_STAKE);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_stake_increment() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(1), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        let tokens_to_add = 1_000_000_000_000_000_000_000_000;
        contract.increment_stake(tokens_to_add);
        assert_eq!(contract.total_stake, TOTAL_STAKE + tokens_to_add);
    }

    #[test]
    fn test_stake_decrement() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(1), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        let tokens_to_sub = 1_000_000_000_000_000_000_000_000;
        contract.decrement_stake(tokens_to_sub);
        assert_eq!(contract.total_stake, TOTAL_STAKE - tokens_to_sub);
    }

    #[test]
    fn test_get_token_price() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::init(accounts(1), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        let token_price = TOTAL_STAKE / TOTAL_SUPPLY;
        assert_eq!(contract.get_token_price(), token_price);
    }

    #[test]
    fn test_stake() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = Contract::init(accounts(2), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        let attached_deposit = 1_000_000_000_000_000_000_000_000; // 1 NEAR in yoctoNEAR
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(attached_deposit)
            .predecessor_account_id(accounts(1))
            .build());
        contract.stake();

        let tokens = attached_deposit / contract.get_token_price();
        assert_eq!(contract.total_stake, TOTAL_STAKE + tokens);
        assert_eq!(contract.token.total_supply, TOTAL_SUPPLY + tokens);
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract =
            Contract::init(accounts(2).into(), TOTAL_SUPPLY.into(), TOTAL_STAKE.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(
            contract.ft_balance_of(accounts(2)).0,
            (TOTAL_SUPPLY - transfer_amount)
        );
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
