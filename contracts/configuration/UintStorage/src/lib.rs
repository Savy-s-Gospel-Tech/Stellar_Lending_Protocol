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
    /// Get uint by key
    pub fn get_uint(_env: Env, _key: Symbol) -> u128 {
        todo!("return uint from storage by key")
    }

    /// Set uint by key (internal function)
    pub fn set_uint(_env: Env, _key: Symbol, _value: u128) {
        todo!("store uint by key")
    }
}
