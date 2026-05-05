//! # PoolCore — state storage and token custody
//!
//! PoolCore holds all protocol state and all deposited tokens. It is the
//! "treasury" of the lending protocol — every asset deposited by users
//! lives in this contract's balance.
//!
//! ## Separation of concerns
//! LendingPool handles validation and business logic.
//! PoolCore handles state storage and token transfers.
//!
//! This separation means the business logic (LendingPool) can be upgraded
//! without moving funds. Only LendingPool can call state-mutating functions
//! on PoolCore — this is enforced via `require_auth`.
//!
//! ## Index cumulation
//! Before any balance change, `update_cumulative_indexes` must be called.
//! This brings the liquidity index (Ci) and variable borrow index (Bvc)
//! up to date for the current ledger, so that interest accrues correctly.
//!
//! ## Interest rate update
//! After any balance change, `update_interest_rates` must be called.
//! This recalculates utilisation and the three rates (Rl, Rs, Rv) and
//! persists them to the reserve.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
pub enum DataKey {
    /// The LendingPool address — only this can call mutating functions.
    LendingPool,
    /// ReserveData keyed by asset address.
    Reserve(Address),
    /// UserReserveData keyed by (user, asset).
    UserReserve(Address, Address),
    /// Ordered list of all reserve asset addresses.
    ReserveList,
}

/// Full state of a lending reserve. One per supported asset.
///
/// All rate fields are in RAY precision (1e27).
/// All balance fields are in the asset's native units.
#[contracttype]
#[derive(Clone)]
pub struct ReserveData {
    // ── Aggregate balances ────────────────────────────────────────────────────
    /// Total tokens deposited (available liquidity + total borrows).
    pub total_liquidity: i128,
    /// Total outstanding variable-rate debt (principal, before interest).
    pub total_variable_borrows: i128,
    /// Total outstanding stable-rate debt (principal, before interest).
    pub total_stable_borrows: i128,
    /// Weighted average stable borrow rate across all stable positions (RAY).
    pub avg_stable_borrow_rate: i128,

    // ── Cumulative indexes (RAY, initialised to 1 RAY) ────────────────────────
    /// Ci — grows each ledger by the liquidity rate.
    /// Depositor real balance = sToken_balance * (Ci_now / Ci_at_deposit).
    pub liquidity_index: i128,
    /// Bvc — compounds each ledger by the variable borrow rate.
    /// Variable borrower real debt = principal * (Bvc_now / Bvc_at_borrow).
    pub variable_borrow_index: i128,

    // ── Current rates (RAY) ───────────────────────────────────────────────────
    pub current_liquidity_rate: i128,
    pub current_variable_borrow_rate: i128,
    pub current_stable_borrow_rate: i128,

    // ── Timestamps ───────────────────────────────────────────────────────────
    /// Ledger number of last index update.
    pub last_update_ledger: u32,

    // ── Contract addresses ────────────────────────────────────────────────────
    pub s_token_address: Address,
    pub interest_rate_strategy: Address,

    // ── Risk parameters ───────────────────────────────────────────────────────
    /// Max loan-to-value in basis points (e.g. 7500 = 75%).
    pub ltv_bps: u32,
    /// Liquidation threshold in basis points (e.g. 8000 = 80%).
    pub liquidation_threshold_bps: u32,
    /// Liquidation bonus in basis points (e.g. 10500 = 105%).
    pub liquidation_bonus_bps: u32,
    pub decimals: u32,

    // ── Feature flags ─────────────────────────────────────────────────────────
    pub borrowing_enabled: bool,
    pub stable_rate_borrowing_enabled: bool,
    pub usage_as_collateral_enabled: bool,
    pub is_active: bool,
    pub is_frozen: bool,
}

/// Per-user state for a single reserve. One per (user, asset) pair.
#[contracttype]
#[derive(Clone)]
pub struct UserReserveData {
    /// Variable borrow index snapshot at time of last borrow (RAY).
    pub variable_borrow_index: i128,
    /// Principal debt at time of last borrow/repay.
    pub principal_borrow_balance: i128,
    /// Locked-in stable rate for this user's stable borrow (RAY). 0 if none.
    pub stable_borrow_rate: i128,
    /// Origination fee owed (paid on repay).
    pub origination_fee: i128,
    /// Whether this user's deposit is being used as collateral.
    pub use_as_collateral: bool,
    /// Ledger of last interaction (used for stable interest accrual).
    pub last_update_ledger: u32,
}

#[contract]
pub struct LendingPoolCore;

#[contractimpl]
impl LendingPoolCore {
    /// Initialize PoolCore with the LendingPool address.
    ///
    /// # TODO
    /// 1. Panic if already initialised
    /// 2. Store `lending_pool` under DataKey::LendingPool
    /// 3. Store empty Vec under DataKey::ReserveList
    pub fn initialize(_env: Env, _lending_pool: Address) {
        todo!("store lending_pool; init empty reserve list; panic if already initialised")
    }

    // ── Reserve management ────────────────────────────────────────────────────

    /// Register a new reserve. Called by PoolConfigurator.
    ///
    /// # TODO
    /// 1. `lending_pool.require_auth()`
    /// 2. Panic if reserve already exists
    /// 3. Store `reserve` under DataKey::Reserve(asset)
    /// 4. Append `asset` to DataKey::ReserveList
    pub fn init_reserve(_env: Env, _asset: Address, _reserve: ReserveData) {
        todo!("require_auth pool; panic if exists; store reserve; append to list")
    }

    pub fn get_reserve(_env: Env, _asset: Address) -> ReserveData {
        todo!("return DataKey::Reserve(asset); panic if not found")
    }

    pub fn get_reserve_list(_env: Env) -> Vec<Address> {
        todo!("return DataKey::ReserveList")
    }

    // ── Index cumulation ──────────────────────────────────────────────────────

    /// Bring Ci and Bvc up to date for the current ledger.
    /// Must be called before any balance change on a reserve.
    ///
    /// # Formulas
    /// ```
    /// delta_ledgers = current_ledger - last_update_ledger
    ///
    /// // Liquidity index: simple interest (linear)
    /// Ci_new = Ci_old * (1 + Rl * delta_ledgers / LEDGERS_PER_YEAR)
    ///        = ray_mul(Ci_old, RAY + ray_mul(Rl, delta_ledgers / LEDGERS_PER_YEAR))
    ///
    /// // Variable borrow index: compound interest
    /// Bvc_new = Bvc_old * calculate_compound_interest(Rv, delta_ledgers)
    /// ```
    ///
    /// # TODO
    /// 1. `lending_pool.require_auth()`
    /// 2. Load reserve; compute delta_ledgers
    /// 3. If delta_ledgers == 0, return early
    /// 4. Update liquidity_index and variable_borrow_index
    /// 5. Set last_update_ledger = current_ledger
    /// 6. Persist updated reserve
    pub fn update_cumulative_indexes(_env: Env, _asset: Address) {
        todo!("require_auth pool; compute delta; update Ci and Bvc; persist")
    }

    // ── Interest rate update ──────────────────────────────────────────────────

    /// Recalculate utilisation and all three rates. Call after every balance change.
    ///
    /// # Formula
    /// ```
    /// available_liquidity = total_liquidity - total_variable_borrows - total_stable_borrows
    /// (Rl, Rs, Rv) = InterestRateStrategy::calculate_interest_rates(
    ///     available_liquidity, total_stable_borrows, total_variable_borrows, avg_stable_rate
    /// )
    /// ```
    ///
    /// # TODO
    /// 1. `lending_pool.require_auth()`
    /// 2. Load reserve; compute available_liquidity
    /// 3. Cross-contract call to interest_rate_strategy.calculate_interest_rates(...)
    /// 4. Persist new rates to reserve
    pub fn update_interest_rates(_env: Env, _asset: Address) {
        todo!("require_auth pool; compute available liquidity; call strategy; persist rates")
    }

    // ── Liquidity mutations ───────────────────────────────────────────────────

    pub fn increase_total_liquidity(_env: Env, _asset: Address, _amount: i128) {
        todo!("require_auth pool; total_liquidity += amount; persist")
    }

    pub fn decrease_total_liquidity(_env: Env, _asset: Address, _amount: i128) {
        todo!("require_auth pool; assert total_liquidity >= amount; total_liquidity -= amount; persist")
    }

    // ── Borrow mutations ──────────────────────────────────────────────────────

    pub fn increase_variable_borrows(_env: Env, _asset: Address, _amount: i128) {
        todo!("require_auth pool; total_variable_borrows += amount; persist")
    }

    pub fn decrease_variable_borrows(_env: Env, _asset: Address, _amount: i128) {
        todo!("require_auth pool; assert sufficient; total_variable_borrows -= amount; persist")
    }

    /// Increase stable borrows and update the weighted average stable rate.
    ///
    /// # Formula
    /// ```
    /// Rsa_new = (Bs_old * Rsa_old + amount * rate) / (Bs_old + amount)
    /// ```
    pub fn increase_stable_borrows(_env: Env, _asset: Address, _amount: i128, _rate: i128) {
        todo!(
            "require_auth pool
             Rsa_new = (total_stable * avg_stable_rate + amount * rate) / (total_stable + amount)
             total_stable_borrows += amount
             avg_stable_borrow_rate = Rsa_new
             persist"
        )
    }

    /// Decrease stable borrows and update the weighted average stable rate.
    ///
    /// # Formula
    /// ```
    /// if Bs_old - amount == 0: Rsa_new = 0
    /// else: Rsa_new = (Bs_old * Rsa_old - amount * rate) / (Bs_old - amount)
    /// ```
    pub fn decrease_stable_borrows(_env: Env, _asset: Address, _amount: i128, _rate: i128) {
        todo!(
            "require_auth pool
             if total_stable - amount == 0: avg_stable_borrow_rate = 0
             else: Rsa_new = (total_stable * avg_stable_rate - amount * rate) / (total_stable - amount)
             total_stable_borrows -= amount
             persist"
        )
    }

    // ── User reserve mutations ────────────────────────────────────────────────

    pub fn set_user_reserve(_env: Env, _user: Address, _asset: Address, _data: UserReserveData) {
        todo!("require_auth pool; persist DataKey::UserReserve(user, asset)")
    }

    pub fn get_user_reserve(_env: Env, _user: Address, _asset: Address) -> UserReserveData {
        todo!("return DataKey::UserReserve(user, asset); panic if not found")
    }
}
