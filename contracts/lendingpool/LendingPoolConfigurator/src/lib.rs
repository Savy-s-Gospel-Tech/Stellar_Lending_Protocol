//! # LendingPoolConfigurator
//!
//! Admin-only contract for initialising and configuring reserves.
//!
//! ## Why it's separate from LendingPool
//! Configuration changes (adding reserves, changing LTV, enabling/disabling
//! borrowing) are governance actions — infrequent, high-stakes, and should
//! require a separate auth path from normal user actions. Keeping them in a
//! dedicated contract means:
//! - Governance can be upgraded independently
//! - LendingPool stays lean (no admin logic mixed with user logic)
//! - Easier to audit: all reserve config changes are in one place
//!
//! ## Responsibilities
//! - Initialise new reserves (deploys sToken, sets strategy, registers in Core)
//! - Enable/disable borrowing on a reserve
//! - Enable/disable a reserve as collateral
//! - Set LTV, liquidation threshold, liquidation bonus
//! - Activate/deactivate a reserve
//! - Freeze a reserve (no new deposits/borrows; repay/redeem still allowed)

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    AddressesProvider,
}

#[contract]
pub struct LendingPoolConfigurator;

#[contractimpl]
impl LendingPoolConfigurator {
    pub fn initialize(_env: Env, _addresses_provider: Address) {
        todo!("store addresses_provider; panic if already initialised")
    }

    /// Initialise a new reserve. Deploys an sToken for it and registers in Core.
    ///
    /// # TODO
    /// 1. require_auth: only LendingPoolManager (from AddressesProvider)
    /// 2. Assert reserve not already initialised
    /// 3. Deploy sToken contract for this asset
    /// 4. Cross-contract call: core.init_reserve(asset, ReserveData { ... })
    /// 5. Emit ReserveInitialized event
    pub fn init_reserve(
        _env: Env,
        _asset: Address,
        _decimals: u32,
        _interest_rate_strategy: Address,
        _s_token_name: soroban_sdk::String,
        _s_token_symbol: soroban_sdk::String,
    ) {
        todo!("require_auth manager; deploy sToken; register reserve in Core; emit event")
    }

    pub fn enable_borrowing_on_reserve(_env: Env, _asset: Address, _stable_rate_enabled: bool) {
        todo!("require_auth manager; core.set_reserve_borrowing_enabled(asset, true, stable_rate_enabled)")
    }

    pub fn disable_borrowing_on_reserve(_env: Env, _asset: Address) {
        todo!("require_auth manager; core.set_reserve_borrowing_enabled(asset, false, false)")
    }

    /// Enable an asset as collateral with risk parameters.
    ///
    /// # TODO
    /// 1. require_auth manager
    /// 2. Assert liquidation_threshold_bps >= ltv_bps
    ///    (if threshold < LTV, a position could be liquidatable immediately on deposit)
    /// 3. core.set_reserve_collateral_params(asset, ltv_bps, threshold_bps, bonus_bps)
    pub fn enable_reserve_as_collateral(
        _env: Env,
        _asset: Address,
        _ltv_bps: u32,
        _liquidation_threshold_bps: u32,
        _liquidation_bonus_bps: u32,
    ) {
        todo!("require_auth manager; assert threshold >= ltv; update core")
    }

    pub fn disable_reserve_as_collateral(_env: Env, _asset: Address) {
        todo!("require_auth manager; core.set_usage_as_collateral_enabled(asset, false)")
    }

    pub fn activate_reserve(_env: Env, _asset: Address) {
        todo!("require_auth manager; core.set_reserve_active(asset, true)")
    }

    /// Deactivate a reserve. Only possible if no outstanding borrows.
    pub fn deactivate_reserve(_env: Env, _asset: Address) {
        todo!(
            "require_auth manager
             assert total_borrows == 0 (cannot deactivate with outstanding debt)
             core.set_reserve_active(asset, false)"
        )
    }

    /// Freeze a reserve. No new deposits or borrows; repay and redeem still work.
    pub fn freeze_reserve(_env: Env, _asset: Address) {
        todo!("require_auth manager; core.set_reserve_frozen(asset, true)")
    }

    pub fn unfreeze_reserve(_env: Env, _asset: Address) {
        todo!("require_auth manager; core.set_reserve_frozen(asset, false)")
    }
}
