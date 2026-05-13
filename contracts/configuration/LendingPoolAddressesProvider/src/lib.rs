//! # LendingPoolAddressesProvider
//!
//! Central registry of all protocol contract addresses.
//!
//! Every contract in the protocol resolves its dependencies here at runtime.
//! To upgrade any component, the admin updates one entry — all other contracts
//! automatically use the new address on the next call.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const LENDING_POOL: Symbol = symbol_short!("LP");
const LENDING_POOL_CORE: Symbol = symbol_short!("LP_CORE");
const LENDING_POOL_CONFIGURATOR: Symbol = symbol_short!("LP_CFG");
const LENDING_POOL_PARAMETERS_PROVIDER: Symbol = symbol_short!("LP_PARAMS");
const LENDING_POOL_MANAGER: Symbol = symbol_short!("LP_MGR");
const LENDING_POOL_LIQUIDATION_MANAGER: Symbol = symbol_short!("LP_LIQ");
const DATA_PROVIDER: Symbol = symbol_short!("DATA_PRV");
const PRICE_ORACLE: Symbol = symbol_short!("PRICE_ORC");
const LENDING_RATE_ORACLE: Symbol = symbol_short!("RATE_ORC");
const FEE_PROVIDER: Symbol = symbol_short!("FEE_PRV");
const TOKEN_DISTRIBUTOR: Symbol = symbol_short!("TOKEN_DST");

#[contracttype]
pub enum DataKey {
    Owner,
    Entry(Symbol),
}

#[contract]
pub struct LendingPoolAddressesProvider;

impl LendingPoolAddressesProvider {
    fn require_owner(env: &Env) {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("not initialized");
        owner.require_auth();
    }

    fn get_entry(env: &Env, key: Symbol) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Entry(key))
            .expect("address not set")
    }

    fn set_entry(env: &Env, key: Symbol, addr: Address) {
        env.storage().instance().set(&DataKey::Entry(key), &addr);
    }
}

#[contractimpl]
impl LendingPoolAddressesProvider {
    pub fn initialize(env: Env, owner: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Owner),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Owner, &owner);
    }

    pub fn get_lending_pool(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL)
    }
    pub fn set_lending_pool(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL, addr);
    }

    pub fn get_lending_pool_core(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL_CORE)
    }
    pub fn set_lending_pool_core(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL_CORE, addr);
    }

    pub fn get_lending_pool_configurator(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL_CONFIGURATOR)
    }
    pub fn set_lending_pool_configurator(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL_CONFIGURATOR, addr);
    }

    pub fn get_pool_parameters_provider(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL_PARAMETERS_PROVIDER)
    }
    pub fn set_pool_parameters_provider(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL_PARAMETERS_PROVIDER, addr);
    }

    pub fn get_lending_pool_manager(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL_MANAGER)
    }
    pub fn set_lending_pool_manager(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL_MANAGER, addr);
    }

    pub fn get_pool_liquidation_manager(env: Env) -> Address {
        Self::get_entry(&env, LENDING_POOL_LIQUIDATION_MANAGER)
    }
    pub fn set_pool_liquidation_manager(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_POOL_LIQUIDATION_MANAGER, addr);
    }

    pub fn get_lending_pool_data_provider(env: Env) -> Address {
        Self::get_entry(&env, DATA_PROVIDER)
    }
    pub fn set_lending_pool_data_provider(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, DATA_PROVIDER, addr);
    }

    pub fn get_price_oracle(env: Env) -> Address {
        Self::get_entry(&env, PRICE_ORACLE)
    }
    pub fn set_price_oracle(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, PRICE_ORACLE, addr);
    }

    pub fn get_lending_rate_oracle(env: Env) -> Address {
        Self::get_entry(&env, LENDING_RATE_ORACLE)
    }
    pub fn set_lending_rate_oracle(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, LENDING_RATE_ORACLE, addr);
    }

    pub fn get_fee_provider(env: Env) -> Address {
        Self::get_entry(&env, FEE_PROVIDER)
    }
    pub fn set_fee_provider(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, FEE_PROVIDER, addr);
    }

    pub fn get_token_distributor(env: Env) -> Address {
        Self::get_entry(&env, TOKEN_DISTRIBUTOR)
    }
    pub fn set_token_distributor(env: Env, addr: Address) {
        Self::require_owner(&env);
        Self::set_entry(&env, TOKEN_DISTRIBUTOR, addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, LendingPoolAddressesProviderClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(LendingPoolAddressesProvider, ());
        let client = LendingPoolAddressesProviderClient::new(&env, &id);
        (env, client)
    }

    #[test]
    fn set_and_get_lending_pool() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        let pool = Address::generate(&env);
        client.initialize(&owner);
        client.set_lending_pool(&pool);
        assert_eq!(client.get_lending_pool(), pool);
    }

    #[test]
    fn set_and_get_price_oracle() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        let oracle = Address::generate(&env);
        client.initialize(&owner);
        client.set_price_oracle(&oracle);
        assert_eq!(client.get_price_oracle(), oracle);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn double_initialize_panics() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        client.initialize(&owner);
        client.initialize(&owner);
    }

    #[test]
    #[should_panic(expected = "address not set")]
    fn getter_panics_when_not_set() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        client.initialize(&owner);
        client.get_lending_pool();
    }
}
