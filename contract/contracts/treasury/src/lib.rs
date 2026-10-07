#![no_std]
use soroban_sdk::{
    contract, contractimpl, 
    Env, Address, 
    symbol_short, Symbol,
    map, Map
};

const TREASURY_LOCK: Symbol = symbol_short!("TR_LOCK");

#[contract]
pub struct Treasury;

#[contractimpl]
impl Treasury {
    /// Initialize the treasury contract
    pub fn initialize(env: Env) {
        if env.storage().instance().has(&TREASURY_LOCK) {
            panic!("Treasury already initialized");
        }
        env.storage().instance().set(&TREASURY_LOCK, &false);
    }

    /// Deposit funds into the treasury
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 {
            panic!("Deposit amount must be positive");
        }
        Self::_enter_locked_section(&env);
        
        let balance_key = symbol_short!("balance");
        let current_balance: i128 = env.storage().persistent().get(&(from.clone(), balance_key.clone()))
            .unwrap_or(0);
        let new_balance = current_balance.checked_add(amount)
            .expect("Balance overflow");
        
        env.storage().persistent().set(&(from.clone(), balance_key), &new_balance);
        
        env.events().publish(
            (Symbol::new(&env, "Deposit"), from.clone()),
            (amount, new_balance)
        );
        Self::_exit_locked_section(&env);
    }

    /// Withdraw funds from the treasury
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        to.require_auth();
        if amount <= 0 {
            panic!("Withdrawal amount must be positive");
        }
        Self::_enter_locked_section(&env);
        
        let balance_key = symbol_short!("balance");
        let current_balance: i128 = env.storage().persistent().get(&(to.clone(), balance_key.clone()))
            .unwrap_or(0);
        
        if current_balance < amount {
            panic!("Insufficient balance");
        }
        
        let new_balance = current_balance - amount;
        env.storage().persistent().set(&(to.clone(), balance_key), &new_balance);
        
        env.events().publish(
            (Symbol::new(&env, "Withdraw"), to.clone()),
            (amount, new_balance)
        );
        Self::_exit_locked_section(&env);
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        let balance_key = symbol_short!("balance");
        env.storage().persistent().get(&(user.clone(), balance_key.clone()))
            .unwrap_or(0)
    }

    pub fn get_total_balance(env: Env) -> i128 {
        let total = symbol_short!("total");
        env.storage().instance().get(&total)
            .unwrap_or(0)
    }

    fn _enter_locked_section(env: &Env) {
        let lock_key = TREASURY_LOCK;
        let is_locked: bool = env.storage().instance().get(&lock_key)
            .unwrap_or(false);
        if is_locked {
            panic!("Reentrancy detected");
        }
        env.storage().instance().set(&lock_key, &true);
    }

    fn _exit_locked_section(env: &Env) {
        let lock_key = TREASURY_LOCK;
        env.storage().instance().set(&lock_key, &false);
    }

    pub fn update_treasury_balance(env: Env, amount: i128) {
        env.storage().instance().set(&Symbol::new(&env, "treasury_balance"), &amount);
    }

    pub fn update_user_liabilities(env: Env, amount: i128) {
        env.storage().instance().set(&Symbol::new(&env, "user_liabilities"), &amount);
    }

    pub fn check_reserve_ratio(env: Env) {
        let treasury: i128 = env.storage().instance().get(&Symbol::new(&env, "treasury_balance")).unwrap_or(0);
        let liabilities: i128 = env.storage().instance().get(&Symbol::new(&env, "user_liabilities")).unwrap_or(1); // avoid div by zero

        let ratio = (treasury * 100) / liabilities;

        if ratio < 110 {
            env.events().publish((Symbol::new(&env, "reserve_alert"),), (ratio,));
        }
        if ratio < 100 {
            env.storage().instance().set(&symbol_short!("paused"), &true);
            env.events().publish((Symbol::new(&env, "auto_pause"),), (ratio,));
        }
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&symbol_short!("paused")).unwrap_or(false)
    }
}
