//! # IFlashLoanReceiver — flash loan receiver interface
//!
//! Any contract that wants to receive a flash loan must implement this
//! interface. The lending pool calls `execute_operation` after transferring
//! the requested funds.
//!
//! The receiver must return `amount + fee` to the pool before
//! `execute_operation` returns, or the entire transaction reverts.

#![no_std]

use soroban_sdk::{Address, Bytes, Env};

pub trait IFlashLoanReceiver {
    /// Called by the pool after transferring `amount` of `asset`.
    ///
    /// # Arguments
    /// - `initiator` — address that called `flash_loan` on the pool
    /// - `asset`     — the borrowed asset
    /// - `amount`    — the borrowed amount
    /// - `fee`       — fee that must be repaid on top of `amount`
    /// - `params`    — arbitrary bytes passed through from the `flash_loan` call
    ///
    /// # Returns
    /// `true` on success. The pool reverts if `false` is returned or if
    /// `amount + fee` has not been returned to the pool.
    fn execute_operation(
        env: Env,
        initiator: Address,
        asset: Address,
        amount: i128,
        fee: i128,
        params: Bytes,
    ) -> bool;
}
