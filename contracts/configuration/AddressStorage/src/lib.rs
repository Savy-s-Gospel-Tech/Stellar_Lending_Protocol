//! # AddressStorage
//!
//! Storage utility for addresses used by LendingPoolAddressesProvider.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Address(Symbol),
}

#[contract]
pub struct AddressStorage;

#[contractimpl]
impl AddressStorage {
    /// Get address by key. Panics if not set.
    pub fn get_address(env: Env, key: Symbol) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Address(key))
            .expect("address not set")
    }

    /// Set address by key.
    pub fn set_address(env: Env, key: Symbol, address: Address) {
        env.storage()
            .instance()
            .set(&DataKey::Address(key), &address);
    }
}
