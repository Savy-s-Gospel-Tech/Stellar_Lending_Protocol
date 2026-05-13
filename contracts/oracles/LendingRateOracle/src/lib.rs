//! # LendingRateOracle
//!
//! Provides the average market lending rate (Mr) used to seed the stable
//! borrow rate model. Separate from PriceOracle — each can be upgraded
//! independently.
//!
//! v0: admin-fed. v1: oracle aggregator integration.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Price(Address),
    MarketLendingRate,
}

#[contract]
pub struct LendingRateOracle;

#[contractimpl]
impl LendingRateOracle {
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Set USD price for `asset` (7 decimals). Admin only.
    pub fn set_price(env: Env, asset: Address, price: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        assert!(price > 0, "price must be positive");
        env.storage()
            .instance()
            .set(&DataKey::Price(asset), &price);
    }

    /// Get USD price for `asset`. Panics if not set.
    pub fn get_price(env: Env, asset: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Price(asset))
            .expect("price not set for asset")
    }

    /// Set Mr — average market lending rate (ray). Updated by admin in v0.
    pub fn set_market_lending_rate(env: Env, rate: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::MarketLendingRate, &rate);
    }

    /// Get Mr. Returns 0 if not yet set.
    pub fn get_market_lending_rate(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::MarketLendingRate)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn set_and_get_market_lending_rate() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(LendingRateOracle, ());
        let client = LendingRateOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        let rate = 50_000_000_000_000_000_000_000_000i128; // 5% in RAY
        client.set_market_lending_rate(&rate);
        assert_eq!(client.get_market_lending_rate(), rate);
    }

    #[test]
    fn market_lending_rate_defaults_to_zero() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(LendingRateOracle, ());
        let client = LendingRateOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        assert_eq!(client.get_market_lending_rate(), 0);
    }

    #[test]
    #[should_panic(expected = "price not set for asset")]
    fn get_price_panics_when_not_set() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(LendingRateOracle, ());
        let client = LendingRateOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        client.get_price(&Address::generate(&env));
    }
}
