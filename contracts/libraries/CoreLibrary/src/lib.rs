//! # CoreLibrary — shared data structures
//!
//! Defines the two central data structures that every contract in the protocol
//! reads and writes: `ReserveData` and `UserReserveData`.
//!
//! ## ReserveData
//! One instance per supported asset (e.g. USDC, XLM). Tracks the pool's
//! aggregate state: total liquidity, total borrows, cumulative interest
//! indexes, and configuration parameters.
//!
//! ## UserReserveData
//! One instance per (user, asset) pair. Tracks an individual user's deposit
//! and borrow position within a specific reserve.
//!
//! ## Cumulative indexes
//! Interest accrual is tracked via two monotonically increasing indexes:
//!
//! - `liquidity_index` (Ci): starts at 1 RAY. Grows each ledger by the
//!   liquidity rate. A depositor's real balance = principal * (Ci_now / Ci_at_deposit).
//!
//! - `variable_borrow_index` (Bvc): starts at 1 RAY. Compounds each ledger
//!   by the variable borrow rate. A variable borrower's real debt =
//!   principal * (Bvc_now / Bvc_at_borrow).
//!
//! This design means interest accrues lazily — no per-user loop needed.

#![no_std]

use soroban_sdk::{contracttype, Address};

/// Aggregate state of a single lending reserve.
/// Stored in PoolCore keyed by asset address.
#[contracttype]
#[derive(Clone)]
pub struct ReserveData {
    // ── Balances ──────────────────────────────────────────────────────────────
    /// Total tokens deposited (available + borrowed).
    pub total_liquidity: i128,
    /// Total outstanding variable-rate debt (principal only).
    pub total_variable_borrows: i128,
    /// Total outstanding stable-rate debt (principal only).
    pub total_stable_borrows: i128,
    /// Weighted average stable borrow rate across all stable borrowers (RAY).
    /// Updated on every stable borrow/repay.
    pub avg_stable_borrow_rate: i128,

    // ── Cumulative indexes (RAY, start at 1 RAY) ──────────────────────────────
    /// Ci — cumulative liquidity index. Tracks depositor interest over time.
    pub liquidity_index: i128,
    /// Bvc — cumulative variable borrow index. Tracks variable borrower debt.
    pub variable_borrow_index: i128,

    // ── Current rates (RAY) ───────────────────────────────────────────────────
    pub current_liquidity_rate: i128,
    pub current_variable_borrow_rate: i128,
    pub current_stable_borrow_rate: i128,

    // ── Timestamps ───────────────────────────────────────────────────────────
    /// Ledger number of last state update. Used to compute time elapsed.
    pub last_update_ledger: u32,

    // ── Contract addresses ────────────────────────────────────────────────────
    /// The sToken contract for this reserve.
    pub s_token_address: Address,
    /// The interest rate strategy contract for this reserve.
    pub interest_rate_strategy: Address,

    // ── Risk parameters (set by PoolConfigurator) ─────────────────────────────
    /// Max loan-to-value ratio in basis points (e.g. 7500 = 75%).
    pub ltv_bps: u32,
    /// Liquidation threshold in basis points (e.g. 8000 = 80%).
    /// Position is liquidatable when collateral_value * threshold < debt_value.
    pub liquidation_threshold_bps: u32,
    /// Liquidation bonus in basis points (e.g. 10500 = 105%, i.e. 5% bonus).
    pub liquidation_bonus_bps: u32,
    /// Token decimals (e.g. 7 for XLM, 6 for USDC on Stellar).
    pub decimals: u32,

    // ── Feature flags ─────────────────────────────────────────────────────────
    pub borrowing_enabled: bool,
    pub stable_rate_borrowing_enabled: bool,
    pub usage_as_collateral_enabled: bool,
    pub is_active: bool,
    pub is_frozen: bool,
}

/// Per-user state for a single reserve.
/// Stored in PoolCore keyed by (user_address, asset_address).
#[contracttype]
#[derive(Clone)]
pub struct UserReserveData {
    // ── Deposit position ──────────────────────────────────────────────────────
    /// Liquidity index snapshot at time of last deposit/withdraw.
    /// Real balance = sToken_balance * (Ci_now / liquidity_index_snapshot).
    pub liquidity_index_snapshot: i128,

    // ── Borrow position ───────────────────────────────────────────────────────
    /// Principal debt at time of last borrow/repay (before interest).
    pub principal_borrow_balance: i128,
    /// Variable borrow index snapshot at time of last borrow.
    /// Real debt = principal * (Bvc_now / variable_borrow_index_snapshot).
    pub variable_borrow_index_snapshot: i128,
    /// Locked-in stable rate for this user's stable-rate borrow (RAY).
    /// Zero if user has no stable-rate borrow.
    pub stable_borrow_rate: i128,
    /// Origination fee owed (accrued at borrow time, paid on repay).
    pub origination_fee: i128,
    /// Which rate mode this user's borrow is in.
    pub interest_rate_mode: InterestRateMode,

    // ── Collateral flag ───────────────────────────────────────────────────────
    /// Whether this deposit is being used as collateral for borrows.
    pub use_as_collateral: bool,
}

/// Borrow rate mode for a user's position.
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum InterestRateMode {
    /// No active borrow.
    None = 0,
    /// Rate is locked in at borrow time and does not change.
    Stable = 1,
    /// Rate tracks the market and changes every ledger.
    Variable = 2,
}
