//! # Interfaces — cross-contract trait definitions
//!
//! Every contract in the protocol that is called by another contract has a
//! trait defined here. This serves three purposes:
//!
//! 1. Documents the expected public API of each contract
//! 2. Enables mock implementations in tests
//! 3. Makes the dependency graph explicit and auditable
//!
//! In Soroban, cross-contract calls use generated client types (not traits
//! at runtime), but defining these traits makes the protocol's interface
//! contract clear to contributors and auditors.

#![no_std]

use soroban_sdk::{Address, Env};

// ---------------------------------------------------------------------------
// IAddressesProvider
// Implemented by: contracts/configuration/LendingPoolAddressesProvider
// ---------------------------------------------------------------------------
/// Central registry. All contracts resolve each other's addresses through here.
/// The provider address itself is the only hardcoded address in the system.
pub trait IAddressesProvider {
    fn get_lending_pool(env: &Env) -> Address;
    fn get_pool_core(env: &Env) -> Address;
    fn get_pool_configurator(env: &Env) -> Address;
    fn get_pool_data_provider(env: &Env) -> Address;
    fn get_liquidation_manager(env: &Env) -> Address;
    fn get_price_oracle(env: &Env) -> Address;
    fn get_lending_rate_oracle(env: &Env) -> Address;
    fn get_fee_provider(env: &Env) -> Address;
}

// ---------------------------------------------------------------------------
// IFeeProvider
// Implemented by: contracts/fees/FeeProvider
// ---------------------------------------------------------------------------
/// Calculates protocol fees for borrows and flash loans.
pub trait IFeeProvider {
    /// Calculate the origination fee for a borrow of `amount`.
    /// Returns the fee amount in the same units as `amount`.
    fn calculate_loan_origination_fee(env: &Env, user: Address, amount: i128) -> i128;

    /// Return the flash loan fee in basis points (e.g. 9 = 0.09%).
    fn get_flash_loan_fee_in_bps(env: &Env) -> u32;
}

// ---------------------------------------------------------------------------
// ILendingRateOracle
// Implemented by: contracts/oracles/LendingRateOracle
// ---------------------------------------------------------------------------
/// Provides market lending rates used to seed the stable borrow rate.
/// The stable borrow rate for a new position is set to the market rate
/// at time of borrowing, plus a protocol-defined spread.
pub trait ILendingRateOracle {
    /// Return the current market borrow rate for `asset` in RAY precision.
    fn get_market_borrow_rate(env: &Env, asset: Address) -> i128;

    /// Set the market borrow rate for `asset` (admin only).
    fn set_market_borrow_rate(env: &Env, asset: Address, rate: i128);
}

// ---------------------------------------------------------------------------
// IPriceOracle
// Implemented by: contracts/oracles/PriceOracle (v0, admin-fed)
//                 contracts/misc/OracleAggregator (v1, Reflector-backed)
// ---------------------------------------------------------------------------
/// Returns USD prices for assets. Used by the pool to calculate collateral
/// values, health factors, and liquidation amounts.
pub trait IPriceOracle {
    /// Return the USD price of `asset` with 8 decimal precision.
    /// e.g. $1.00 = 100_000_000
    fn get_asset_price(env: &Env, asset: Address) -> i128;

    /// Set the USD price of `asset` (admin only, v0).
    fn set_asset_price(env: &Env, asset: Address, price: i128);
}

// ---------------------------------------------------------------------------
// IInterestRateStrategy
// Implemented by: contracts/lendingpool/DefaultReserveInterestRateStrategy
// ---------------------------------------------------------------------------
/// Calculates interest rates for a reserve based on current utilisation.
/// Each reserve has its own strategy contract with its own parameters.
pub trait IInterestRateStrategy {
    /// Calculate the three current rates for a reserve.
    ///
    /// # Arguments
    /// - `available_liquidity`      — tokens currently available to borrow
    /// - `total_borrows_stable`     — total stable-rate debt outstanding
    /// - `total_borrows_variable`   — total variable-rate debt outstanding
    /// - `avg_stable_borrow_rate`   — weighted average stable rate (RAY)
    ///
    /// # Returns
    /// `(liquidity_rate, stable_borrow_rate, variable_borrow_rate)` all in RAY
    fn calculate_interest_rates(
        env: &Env,
        available_liquidity: i128,
        total_borrows_stable: i128,
        total_borrows_variable: i128,
        avg_stable_borrow_rate: i128,
    ) -> (i128, i128, i128);
}

// ---------------------------------------------------------------------------
// IPriceFeed
// Implemented by: Reflector oracle adapters (v1)
// ---------------------------------------------------------------------------
/// Minimal interface for an on-chain price feed (e.g. Reflector node).
/// The OracleAggregator calls this to get prices from individual feeds.
pub trait IPriceFeed {
    /// Return the latest price with 8 decimal precision.
    fn latest_answer(env: &Env) -> i128;
}

// ---------------------------------------------------------------------------
// IFlashLoanReceiver
// Implemented by: any contract that wants to receive flash loans
// ---------------------------------------------------------------------------
/// Any contract that borrows via flash loan must implement this interface.
/// The pool calls `execute_operation` after transferring funds.
pub trait IFlashLoanReceiver {
    /// Called by the pool after transferring `amount` of `asset`.
    ///
    /// The receiver must return `amount + fee` to the pool before this
    /// function returns, or the entire transaction reverts.
    ///
    /// # Returns
    /// `true` on success. Returning `false` causes the pool to revert.
    fn execute_operation(
        env: Env,
        initiator: Address,
        asset: Address,
        amount: i128,
        fee: i128,
        params: soroban_sdk::Bytes,
    ) -> bool;
}
