//! # PriceOracle
//!
//! Provides USD asset prices to the protocol. Separate from LendingRateOracle.
//!
//! ## Why two oracles?
//! Two oracles serve different purposes:
//! - PriceOracle         → asset USD prices, used for health factor + liquidation math
//! - LendingRateOracle   → market lending rate Mr, used only for stable rate Rs calculation
//!
//! Keeping them separate means each can be upgraded independently and sourced
//! from different providers (e.g. Reflector for prices, admin-fed for Mr in v0).
//!
//! ## v0 → v1 path
//! v0: admin-fed (centralised push). Admin calls set_asset_price() manually.
//! v1: integrate Reflector on-chain oracle aggregator (Stellar's native oracle).
//!
//! ## Price precision
//! All prices in USD with 7 decimal places (Stellar convention).
//! Example: $1.00 = 10_000_000

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    AssetPrice(Address),
}

#[contract]
pub struct PriceOracle;

#[contractimpl]
impl PriceOracle {
    pub fn initialize(_env: Env, _admin: Address) {
        todo!("store admin; panic if already initialised")
    }

    /// Set USD price for `asset` (7 decimals). Admin only in v0.
    pub fn set_asset_price(_env: Env, _asset: Address, _price: i128) {
        todo!("require_auth admin; assert price > 0; persist AssetPrice(asset)")
    }

    /// Get USD price for `asset`. Called by LendingPoolDataProvider for health factor.
    pub fn get_asset_price(_env: Env, _asset: Address) -> i128 {
        todo!("return AssetPrice(asset); panic with clear message if not set")
    }
}
