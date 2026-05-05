//! # LendingPoolDataProvider
//!
//! Read-only aggregation layer. Reads from PoolCore and the price oracle,
//! computes derived values, and returns them to LendingPool.
//!
//! ## Why it's separate from LendingPool
//! LendingPool needs complex multi-asset calculations (health factor, max
//! borrowable, average LTV) that require iterating over all user positions
//! and calling the oracle for each. Putting this logic in LendingPool would
//! bloat it. DataProvider is a stateless view contract — it reads, computes,
//! and returns. It can be upgraded without touching Core or LendingPool.
//!
//! ## Responsibilities
//! - Calculate health factor for a user across all reserves
//! - Calculate total collateral value in USD (weighted by LTV)
//! - Calculate total borrow value in USD
//! - Calculate max borrowable amount for a user
//! - Calculate average LTV and average liquidation threshold

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct UserAccountData {
    /// Total collateral value in USD (8 decimal precision).
    pub total_collateral_usd: i128,
    /// Total borrow value in USD (8 decimal precision).
    pub total_borrows_usd: i128,
    /// Total origination fees in USD.
    pub total_fees_usd: i128,
    /// How much more the user can borrow in USD.
    pub available_borrows_usd: i128,
    /// Weighted average liquidation threshold across collateral (basis points).
    pub current_liquidation_threshold: u32,
    /// Weighted average LTV across collateral (basis points).
    pub ltv: u32,
    /// Health factor in RAY. < 1 RAY means the position is liquidatable.
    pub health_factor: i128,
}

#[contracttype]
pub enum DataKey {
    Core,
    Oracle,
}

#[contract]
pub struct LendingPoolDataProvider;

#[contractimpl]
impl LendingPoolDataProvider {
    pub fn initialize(_env: Env, _core: Address, _oracle: Address) {
        todo!("store core and oracle addresses; panic if already initialised")
    }

    /// Returns full account summary for a user across all reserves.
    /// Called by LendingPool before every borrow and withdraw.
    ///
    /// # TODO
    /// 1. Cross-contract call: core.get_reserve_list() — iterate all assets
    /// 2. For each asset where user has a position:
    ///    a. core.get_user_reserve_data(user, asset)
    ///    b. oracle.get_asset_price(asset)
    ///    c. If collateral_enabled: add to total_collateral_usd weighted by LTV
    ///    d. If has borrow: compute compounded balance, add to total_borrows_usd
    /// 3. health_factor = (total_collateral_usd * avg_liq_threshold / 10_000) * RAY
    ///                    / (total_borrows_usd + total_fees_usd)
    ///    If total_borrows == 0: health_factor = i128::MAX
    /// 4. available_borrows_usd = total_collateral_usd * avg_ltv / 10_000
    ///                            - total_borrows_usd
    pub fn get_user_account_data(_env: Env, _user: Address) -> UserAccountData {
        todo!("iterate reserves → sum collateral and debt → compute health factor and available borrows")
    }

    /// Calculate health factor from pre-computed USD totals.
    /// Pure function — used internally and by LendingPool for quick checks.
    ///
    /// # Formula
    /// Hf = (total_collateral_usd * liquidation_threshold_bps / 10_000) * RAY
    ///      / (total_borrows_usd + total_fees_usd)
    ///
    /// Returns i128::MAX if total_borrows_usd + total_fees_usd == 0.
    pub fn calc_health_factor(
        _env: Env,
        _total_collateral_usd: i128,
        _total_borrows_usd: i128,
        _total_fees_usd: i128,
        _liquidation_threshold_bps: u32,
    ) -> i128 {
        todo!(
            "if total_borrows + total_fees == 0: return i128::MAX
             (total_collateral * liquidation_threshold_bps / 10_000) * RAY
             / (total_borrows + total_fees)"
        )
    }

    /// Returns the compounded borrow balance for a user on a specific reserve.
    ///
    /// Variable rate: Bxc = principal * (Bvc_current / Bvc_at_borrow)
    /// Stable rate:   Bxc = principal * calculate_compound_interest(Rs, delta_ledgers)
    pub fn get_compounded_borrow_balance(_env: Env, _user: Address, _asset: Address) -> i128 {
        todo!(
            "load user_reserve and reserve from core
             if variable: principal * variable_borrow_index / user_snapshot
             if stable:   principal * calculate_compound_interest(stable_rate, delta_ledgers)"
        )
    }
}
