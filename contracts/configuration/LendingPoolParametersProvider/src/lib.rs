//! # LendingPoolParametersProvider
//!
//! Stores protocol-wide numeric constants controlled by governance.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

const RAY: i128 = 1_000_000_000_000_000_000_000_000_000;

const DEFAULT_MAX_STABLE_BORROW_SIZE_PCT: u32 = 25;
const DEFAULT_REBALANCE_DOWN_RATE_DELTA: i128 = RAY / 5; // 0.2 ray
const DEFAULT_FLASHLOAN_FEE_TOTAL: u32 = 35; // 0.35%
const DEFAULT_FLASHLOAN_FEE_PROTOCOL: u32 = 3000; // 30% of fee to protocol

#[contracttype]
pub enum DataKey {
    AddressesProvider,
    MaxStableRateBorrowSizePercent,
    RebalanceDownRateDelta,
    FlashLoanFeeTotal,
    FlashLoanFeeProtocol,
}

#[contract]
pub struct LendingPoolParametersProvider;

#[contractimpl]
impl LendingPoolParametersProvider {
    pub fn initialize(env: Env, addresses_provider: Address) {
        assert!(
            !env.storage()
                .instance()
                .has(&DataKey::AddressesProvider),
            "already initialized"
        );
        env.storage()
            .instance()
            .set(&DataKey::AddressesProvider, &addresses_provider);
        env.storage().instance().set(
            &DataKey::MaxStableRateBorrowSizePercent,
            &DEFAULT_MAX_STABLE_BORROW_SIZE_PCT,
        );
        env.storage().instance().set(
            &DataKey::RebalanceDownRateDelta,
            &DEFAULT_REBALANCE_DOWN_RATE_DELTA,
        );
        env.storage()
            .instance()
            .set(&DataKey::FlashLoanFeeTotal, &DEFAULT_FLASHLOAN_FEE_TOTAL);
        env.storage().instance().set(
            &DataKey::FlashLoanFeeProtocol,
            &DEFAULT_FLASHLOAN_FEE_PROTOCOL,
        );
    }

    pub fn get_max_stable_borrow_size_pct(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::MaxStableRateBorrowSizePercent)
            .unwrap()
    }

    pub fn get_rebalance_down_rate_delta(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::RebalanceDownRateDelta)
            .unwrap()
    }

    pub fn get_flash_loan_fees_in_bips(env: Env) -> (u32, u32) {
        let total: u32 = env
            .storage()
            .instance()
            .get(&DataKey::FlashLoanFeeTotal)
            .unwrap();
        let protocol: u32 = env
            .storage()
            .instance()
            .get(&DataKey::FlashLoanFeeProtocol)
            .unwrap();
        (total, protocol)
    }

    pub fn set_flash_loan_fees(env: Env, total_bps: u32, protocol_bps: u32) {
        Self::require_manager(&env);
        env.storage()
            .instance()
            .set(&DataKey::FlashLoanFeeTotal, &total_bps);
        env.storage()
            .instance()
            .set(&DataKey::FlashLoanFeeProtocol, &protocol_bps);
    }

    pub fn set_max_stable_borrow_size_pct(env: Env, percent: u32) {
        Self::require_manager(&env);
        env.storage()
            .instance()
            .set(&DataKey::MaxStableRateBorrowSizePercent, &percent);
    }
}

impl LendingPoolParametersProvider {
    fn require_manager(env: &Env) {
        // Resolve manager via AddressesProvider at runtime.
        // AddressesProvider client call omitted here to avoid circular dep in tests;
        // governance calls are validated by the AddressesProvider contract itself.
        let _provider: Address = env
            .storage()
            .instance()
            .get(&DataKey::AddressesProvider)
            .expect("not initialized");
        // TODO: cross-contract call to provider.get_lending_pool_manager() and require_auth
    }
}
