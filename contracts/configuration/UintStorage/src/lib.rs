//! # UintStorage
//!
//! Storage utility for uint values.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Uint(Symbol),
}

#[contract]
pub struct UintStorage;

#[contractimpl]
impl UintStorage {
    /// Get uint by key. Panics if not set.
    pub fn get_uint(env: Env, key: Symbol) -> u128 {
        env.storage()
            .instance()
            .get(&DataKey::Uint(key))
            .expect("uint not set")
    }

    /// Set uint by key.
    pub fn set_uint(env: Env, key: Symbol, value: u128) {
        env.storage()
            .instance()
            .set(&DataKey::Uint(key), &value);
    }
}
