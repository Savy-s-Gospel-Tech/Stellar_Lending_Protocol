//! # FeeProvider
//!
//! Calculates fees charged by the protocol.
//!
//! ## Why it's separate
//! Fee logic is governance-controlled and may evolve (e.g. discounts for stakers,
//! volume-based tiers). Isolating it means the fee model can be upgraded without
//! touching LendingPool or Core. LendingPool calls FeeProvider to get the fee
//! amount before executing a borrow.
//!
//! ## Default fees
//! Origination fee: 0.0025% (25 basis points / 10000) of the loan amount.
//! Charged on every borrow, added to the user's debt.
//!
//! ## Formula
//! origination_fee = amount * origination_fee_percentage / WAD
//! where origination_fee_percentage = 0.0025 * 1e18 (wad)

#![no_std]
#![allow(dead_code)] // constants used once functions are implemented

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Default origination fee: 25 bps = 0.0025 * WAD
const DEFAULT_ORIGINATION_FEE_WAD: i128 = 2_500_000_000_000_000; // 0.0025 * 1e18

#[contracttype]
pub enum DataKey {
    AddressesProvider,
    OriginationFeePercentage, // wad
}

#[contract]
pub struct FeeProvider;

#[contractimpl]
impl FeeProvider {
    pub fn initialize(_env: Env, _addresses_provider: Address) {
        todo!(
            "store addresses_provider
             set origination_fee_percentage = DEFAULT_ORIGINATION_FEE_WAD"
        )
    }

    /// Calculate the origination fee for a loan of `amount`.
    /// `_user` is reserved for future discount logic (e.g. staking tiers).
    pub fn calculate_loan_origination_fee(_env: Env, _user: Address, _amount: i128) -> i128 {
        todo!("wad_mul(amount, origination_fee_percentage)")
    }

    pub fn get_loan_origination_fee_pct(_env: Env) -> i128 {
        todo!("return origination_fee_percentage (wad)")
    }

    /// Governance: update the origination fee percentage.
    pub fn set_loan_origination_fee_pct(_env: Env, _fee_wad: i128) {
        todo!("require_auth manager from AddressesProvider; update fee")
    }
}
