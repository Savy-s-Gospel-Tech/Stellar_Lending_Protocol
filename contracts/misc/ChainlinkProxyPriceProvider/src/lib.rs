//! # OracleAggregator (contracts/misc/)
//!
//! Aggregates prices from multiple oracle sources and exposes a single
//! `get_asset_price` interface to the rest of the protocol.
//!
//! ## v0: admin-fed
//! The PriceOracle contract (contracts/oracles/PriceOracle) is used directly.
//! An admin sets prices manually. Suitable for testnet.
//!
//! ## v1: Reflector integration
//! This contract will wrap Reflector oracle feeds. Reflector is a
//! decentralised price feed network native to Stellar, operated by a DAO
//! of trusted node operators. See: https://reflector.network
//!
//! For each asset, a Reflector feed address is registered here. On price
//! queries, this contract calls the feed and returns the latest price.
//! A fallback to the admin-fed oracle is used if the feed is stale.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    /// Maps asset address → oracle feed address.
    PriceFeed(Address),
    /// Fallback oracle address (used if feed is stale or unregistered).
    FallbackOracle,
}

#[contract]
pub struct OracleAggregator;

#[contractimpl]
impl OracleAggregator {
    /// # TODO: store admin; panic if already initialised
    pub fn initialize(_env: Env, _admin: Address, _fallback_oracle: Address) {
        todo!("store admin and fallback_oracle")
    }

    /// Register a price feed for an asset.
    ///
    /// # TODO
    /// 1. `admin.require_auth()`
    /// 2. Store DataKey::PriceFeed(asset) → feed
    pub fn set_asset_source(_env: Env, _asset: Address, _feed: Address) {
        todo!("require_auth admin; store feed for asset")
    }

    /// Return the USD price of `asset` with 8 decimal precision.
    ///
    /// # TODO
    /// 1. Load feed from DataKey::PriceFeed(asset)
    /// 2. If no feed registered, call fallback_oracle.get_asset_price(asset)
    /// 3. Otherwise call feed.latest_answer()
    /// 4. Return price
    pub fn get_asset_price(_env: Env, _asset: Address) -> i128 {
        todo!("load feed; call feed.latest_answer() or fallback; return price")
    }

    pub fn get_source_of_asset(_env: Env, _asset: Address) -> Address {
        todo!("return DataKey::PriceFeed(asset)")
    }
}
