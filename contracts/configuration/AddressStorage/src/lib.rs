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
    /// Get address by key
    pub fn get_address(_env: Env, _key: Symbol) -> Address {
        todo!("return address from storage by key")
    }

    /// Set address by key (internal function)
    pub fn set_address(_env: Env, _key: Symbol, _address: Address) {
        todo!("store address by key")
    }
}
