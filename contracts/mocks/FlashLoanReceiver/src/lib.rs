//! # MockFlashLoanReceiver (contracts/mocks/)
//!
//! A mock implementation of the flash loan receiver interface, used for
//! testing the LendingPool's flash loan functionality.
//!
//! This is NOT a production contract.
//! contract. Real flash loan receivers are built by external protocol users.
//!
//! ## Flash loan flow
//! 1. Caller invokes `LendingPool::flash_loan(receiver, asset, amount, params)`
//! 2. Pool transfers `amount` of `asset` to `receiver`
//! 3. Pool calls `receiver.execute_operation(asset, amount, fee, params)`
//! 4. Receiver executes arbitrary logic with the funds
//! 5. Receiver approves pool to pull back `amount + fee`
//! 6. Pool asserts balance_after >= balance_before + fee; reverts if not

#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Bytes, Env};

#[contract]
pub struct MockFlashLoanReceiver;

#[contractimpl]
impl MockFlashLoanReceiver {
    /// Called by the LendingPool after transferring flash loan funds.
    ///
    /// # Arguments
    /// - `initiator` — address that initiated the flash loan
    /// - `asset`     — the borrowed asset
    /// - `amount`    — the borrowed amount
    /// - `fee`       — fee that must be repaid on top of `amount`
    /// - `params`    — arbitrary bytes passed through from `flash_loan`
    ///
    /// # Returns
    /// `true` on success. Pool reverts if `false` or funds not returned.
    ///
    /// # TODO
    /// 1. Implement test logic (e.g. verify funds arrived, do a no-op)
    /// 2. Approve the pool to pull back `amount + fee`
    /// 3. Return `true`
    pub fn execute_operation(
        _env: Env,
        _initiator: Address,
        _asset: Address,
        _amount: i128,
        _fee: i128,
        _params: Bytes,
    ) -> bool {
        todo!("mock: approve pool to pull amount + fee, return true")
    }
}
