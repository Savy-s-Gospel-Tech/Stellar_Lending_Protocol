//! # LendingPoolParametersProvider
//!
//! Stores protocol-wide numeric constants that are controlled by governance.
//!
//! ## Why it's separate
//! These parameters (flash loan fee, max stable borrow size, rebalance delta)
//! are governance-controlled. Isolating them means governance can update a
//! single small contract to change protocol economics without touching Core
//! or Pool. It also makes the values easy to find and audit.
//!
//! ## Parameters
//!
//! | Constant                    | Default | Meaning                                      |
//! |-----------------------------|---------|----------------------------------------------|
//! | MAX_STABLE_RATE_BORROW_SIZE | 25 %    | Max % of available liquidity borrowable at stable rate in one tx |
//! | REBALANCE_DOWN_RATE_DELTA   | 0.2 ray | How far above current Rs a user's rate must be before rebalance-down triggers |
//! | FLASHLOAN_FEE_TOTAL         | 35 bps  | Total flash loan fee (0.35 %)                |
//! | FLASHLOAN_FEE_PROTOCOL      | 3000    | Share of flash loan fee going to protocol (30 %) |

#![no_std]
#![allow(dead_code)] // RAY constant used once functions are implemented

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// 1 ray = 1e27
const RAY: i128 = 1_000_000_000_000_000_000_000_000_000;

#[contracttype]
pub enum DataKey {
    AddressesProvider,
    MaxStableRateBorrowSizePercent,
    RebalanceDownRateDelta,
    FlashLoanFeeTotal,
    FlashLoanFeeProtocol,
}

#[contract]
pub struct LendingPoolParametersProvider;

#[contractimpl]
impl LendingPoolParametersProvider {
    pub fn initialize(_env: Env, _addresses_provider: Address) {
        todo!(
            "store addresses_provider
             set defaults:
               MAX_STABLE_RATE_BORROW_SIZE_PERCENT = 25
               REBALANCE_DOWN_RATE_DELTA = RAY / 5   (0.2 ray)
               FLASHLOAN_FEE_TOTAL = 35               (0.35 %)
               FLASHLOAN_FEE_PROTOCOL = 3000          (30 % of fee goes to protocol)"
        )
    }

    /// Max % of available liquidity that can be borrowed at stable rate in one tx.
    /// Prevents rate manipulation: a large stable borrow would spike Rs, then
    /// the attacker could immediately redeem at the inflated rate.
    pub fn get_max_stable_borrow_size_pct(_env: Env) -> u32 {
        todo!("return MAX_STABLE_RATE_BORROW_SIZE_PERCENT")
    }

    /// Rate delta above current Rs that triggers a rebalance-down.
    /// If a user's locked-in stable rate is this much above the current stable
    /// rate, anyone can call rebalance to bring it down to the current rate.
    pub fn get_rebalance_down_rate_delta(_env: Env) -> i128 {
        todo!("return REBALANCE_DOWN_RATE_DELTA (ray)")
    }

    /// Returns (total_fee_bps, protocol_fee_bps) for flash loans.
    pub fn get_flash_loan_fees_in_bips(_env: Env) -> (u32, u32) {
        todo!("return (FLASHLOAN_FEE_TOTAL, FLASHLOAN_FEE_PROTOCOL)")
    }

    // ── Governance setters ────────────────────────────────────────────────────

    pub fn set_flash_loan_fees(_env: Env, _total_bps: u32, _protocol_bps: u32) {
        todo!("require_auth manager from AddressesProvider; update both fee params")
    }

    pub fn set_max_stable_borrow_size_pct(_env: Env, _percent: u32) {
        todo!("require_auth manager; update MAX_STABLE_RATE_BORROW_SIZE_PERCENT")
    }
}
