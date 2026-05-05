//! # LendingPoolAddressesProvider
//!
//! Central registry of all protocol contract addresses.
//!
//! ## Why it exists
//! Every contract in the protocol needs to call other contracts. Hard-coding
//! addresses would make upgrades impossible. Instead, every contract looks up
//! its dependencies here at runtime. To upgrade any component, the admin
//! updates one entry in this registry — all other contracts automatically
//! use the new address on the next call.
//!
//! ## Entries managed
//! - LENDING_POOL
//! - LENDING_POOL_CORE
//! - LENDING_POOL_CONFIGURATOR
//! - LENDING_POOL_PARAMETERS_PROVIDER
//! - LENDING_POOL_MANAGER          (admin/governance address)
//! - LENDING_POOL_LIQUIDATION_MANAGER
//! - DATA_PROVIDER
//! - PRICE_ORACLE
//! - LENDING_RATE_ORACLE
//! - FEE_PROVIDER
//! - TOKEN_DISTRIBUTOR

#![no_std]
#![allow(dead_code)] // constants used once functions are implemented

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

#[contractimpl]
impl LendingPoolAddressesProvider {
    pub fn initialize(_env: Env, _owner: Address) {
        todo!("store owner; panic if already initialised")
    }

    // ── Getters / Setters ─────────────────────────────────────────────────────

    pub fn get_lending_pool(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL)")
    }
    pub fn set_lending_pool(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL)")
    }

    pub fn get_lending_pool_core(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL_CORE)")
    }
    pub fn set_lending_pool_core(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL_CORE)")
    }

    pub fn get_lending_pool_configurator(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL_CONFIGURATOR)")
    }
    pub fn set_lending_pool_configurator(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL_CONFIGURATOR)")
    }

    pub fn get_pool_parameters_provider(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL_PARAMETERS_PROVIDER)")
    }
    pub fn set_pool_parameters_provider(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL_PARAMETERS_PROVIDER)")
    }

    pub fn get_lending_pool_manager(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL_MANAGER)")
    }
    pub fn set_lending_pool_manager(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL_MANAGER)")
    }

    pub fn get_pool_liquidation_manager(_env: Env) -> Address {
        todo!("return Entry(LENDING_POOL_LIQUIDATION_MANAGER)")
    }
    pub fn set_pool_liquidation_manager(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_POOL_LIQUIDATION_MANAGER)")
    }

    pub fn get_lending_pool_data_provider(_env: Env) -> Address {
        todo!("return Entry(DATA_PROVIDER)")
    }
    pub fn set_lending_pool_data_provider(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(DATA_PROVIDER)")
    }

    pub fn get_price_oracle(_env: Env) -> Address {
        todo!("return Entry(PRICE_ORACLE)")
    }
    pub fn set_price_oracle(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(PRICE_ORACLE)")
    }

    pub fn get_lending_rate_oracle(_env: Env) -> Address {
        todo!("return Entry(LENDING_RATE_ORACLE)")
    }
    pub fn set_lending_rate_oracle(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(LENDING_RATE_ORACLE)")
    }

    pub fn get_fee_provider(_env: Env) -> Address {
        todo!("return Entry(FEE_PROVIDER)")
    }
    pub fn set_fee_provider(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(FEE_PROVIDER)")
    }

    pub fn get_token_distributor(_env: Env) -> Address {
        todo!("return Entry(TOKEN_DISTRIBUTOR)")
    }
    pub fn set_token_distributor(_env: Env, _addr: Address) {
        todo!("require_auth owner; set Entry(TOKEN_DISTRIBUTOR)")
    }
}
