//! # PriceOracle
//!
//! Admin-fed USD price oracle (v0). Prices are pushed by the admin and read
//! by LendingPoolDataProvider for health factor and liquidation calculations.
//!
//! v1 will integrate Reflector (Stellar's native on-chain oracle aggregator).
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
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Set USD price for `asset` (7 decimals). Admin only.
    pub fn set_asset_price(env: Env, asset: Address, price: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        assert!(price > 0, "price must be positive");
        env.storage()
            .instance()
            .set(&DataKey::AssetPrice(asset), &price);
    }

    /// Get USD price for `asset`. Called by LendingPoolDataProvider.
    pub fn get_asset_price(env: Env, asset: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::AssetPrice(asset))
            .expect("price not set for asset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn set_and_get_price() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        let admin = Address::generate(&env);
        let asset = Address::generate(&env);
        client.initialize(&admin);
        client.set_asset_price(&asset, &10_000_000);
        assert_eq!(client.get_asset_price(&asset), 10_000_000);
    }

    #[test]
    #[should_panic(expected = "price not set for asset")]
    fn get_price_panics_when_not_set() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        client.get_asset_price(&Address::generate(&env));
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn double_initialize_panics() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(PriceOracle, ());
        let client = PriceOracleClient::new(&env, &id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin);
    }
}
