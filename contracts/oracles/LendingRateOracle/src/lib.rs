//! # Price Oracle
//!
//! Provides USD prices for reserve assets.
//!
//! v0: admin-fed (centralised push). Prices are set manually by the admin.
//! v1: integrate Reflector on-chain oracle aggregator (planned).
//!

//!
//! ## Price precision
//! All prices are expressed in USD with 7 decimal places (Stellar convention).
//! Example: $1.00 = 10_000_000

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Price(Address),
    /// Mr — average market lending rate (ray), used for stable rate model
    MarketLendingRate,
}

#[contract]
pub struct Oracle;

#[contractimpl]
impl Oracle {
    pub fn initialize(_env: Env, _admin: Address) {
        todo!("store admin; panic if already initialised")
    }

    /// Set USD price for `asset` (7 decimals).
    pub fn set_price(_env: Env, _asset: Address, _price: i128) {
        todo!("require_auth admin; assert price > 0; persist")
    }

    /// Get USD price for `asset`. Panics if not set.
    pub fn get_price(_env: Env, _asset: Address) -> i128 {
        todo!("return price from storage; panic with descriptive message if missing")
    }

    /// Set Mr — average market lending rate (ray).
    /// Updated daily by admin in v0; oracle aggregator in v1.
    pub fn set_market_lending_rate(_env: Env, _rate: i128) {
        todo!("require_auth admin; persist MarketLendingRate")
    }

    pub fn get_market_lending_rate(_env: Env) -> i128 {
        todo!("return MarketLendingRate; default to 0 if not set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    #[should_panic]
    fn get_price_unimplemented() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(Oracle, ());
        let client = OracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        client.get_price(&Address::generate(&env));
    }
}
