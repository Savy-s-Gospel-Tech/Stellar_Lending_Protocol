//! # FlashLoanReceiverBase — base contract for flash loan receivers
//!
//! Provides the boilerplate wiring that every flash loan receiver needs:
//! storing the pool address and transferring funds back after execution.
//!
//! Extend this contract to build your own flash loan receiver. Override
//! `execute_operation` with your custom logic (arbitrage, collateral swap, etc.).

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env};

#[contracttype]
pub enum DataKey {
    /// The LendingPool address — used to transfer funds back.
    LendingPool,
}

#[contract]
pub struct FlashLoanReceiverBase;

#[contractimpl]
impl FlashLoanReceiverBase {
    /// Store the pool address. Call this in your receiver's `initialize`.
    ///
    /// # TODO: store lending_pool under DataKey::LendingPool
    pub fn initialize(_env: Env, _lending_pool: Address) {
        todo!("store lending_pool")
    }

    pub fn get_lending_pool(_env: Env) -> Address {
        todo!("return DataKey::LendingPool")
    }

    /// Transfer `amount` of `asset` back to the lending pool.
    /// Call this at the end of your `execute_operation` implementation.
    ///
    /// # TODO
    /// 1. Load pool address from DataKey::LendingPool
    /// 2. Call SEP-41 `transfer(env, this_contract, pool, amount)` on `asset`
    pub fn transfer_to_pool(_env: Env, _asset: Address, _amount: i128) {
        todo!("load pool address; call asset.transfer(self, pool, amount)")
    }

    /// Override this in your receiver contract with your flash loan logic.
    ///
    /// # TODO
    /// 1. Use the borrowed funds
    /// 2. Call `transfer_to_pool(asset, amount + fee)` before returning
    /// 3. Return true
    pub fn execute_operation(
        _env: Env,
        _initiator: Address,
        _asset: Address,
        _amount: i128,
        _fee: i128,
        _params: Bytes,
    ) -> bool {
        todo!("implement flash loan logic; call transfer_to_pool(asset, amount + fee); return true")
    }
}
