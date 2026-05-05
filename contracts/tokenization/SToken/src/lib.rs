//! # sToken — interest-bearing receipt token
//!
//! When a user deposits an asset into the lending pool, they receive sTokens
//! in return at a 1:1 ratio. The sToken represents their claim on the
//! underlying asset plus accrued interest.
//!
//! ## How interest accrues
//! sToken balances grow over time via the reserve's cumulative liquidity
//! index (Ci). The real balance of a user is:
//!
//!   real_balance = principal * (Ci_now / Ci_at_deposit)
//!
//! In v0, balances are stored as principals and the exchange rate is applied
//! at read time. In v1, the sToken contract will hold the index snapshot
//! per user and compute the scaled balance on every `balance()` call.
//!
//! ## SEP-41 compliance
//! sToken implements the Stellar SEP-41 token interface so it is compatible
//! with wallets, explorers, and other DeFi contracts on Stellar.
//!
//! ## Mint / Burn
//! Only the lending pool contract can mint or burn sTokens. This is enforced
//! via `require_auth` against the stored pool address.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
pub enum DataKey {
    /// The lending pool address — only this address can mint/burn.
    Pool,
    TotalSupply,
    Balance(Address),
}

#[contract]
pub struct SToken;

#[contractimpl]
impl SToken {
    /// Initialize the sToken for a specific reserve.
    ///
    /// # Arguments
    /// - `pool` — the LendingPool contract address (sole minter/burner)
    ///
    /// # TODO
    /// 1. Panic if already initialised (check for existing DataKey::Pool)
    /// 2. Store pool address under DataKey::Pool
    /// 3. Set DataKey::TotalSupply = 0
    pub fn initialize(_env: Env, _pool: Address) {
        todo!("store pool; set total_supply = 0; panic if already initialised")
    }

    // ── SEP-41 read interface ─────────────────────────────────────────────────

    pub fn name(_env: Env) -> String {
        todo!("return token name, e.g. 'Stellar Lending sUSDC'")
    }

    pub fn symbol(_env: Env) -> String {
        todo!("return symbol, e.g. 'sUSDC'")
    }

    pub fn decimals(_env: Env) -> u32 {
        todo!("return decimals matching the underlying asset (e.g. 7 for XLM, 6 for USDC)")
    }

    pub fn total_supply(_env: Env) -> i128 {
        todo!("return DataKey::TotalSupply")
    }

    /// Returns the current balance of `account`.
    ///
    /// v0: returns the stored principal balance (1:1 with deposit amount).
    /// v1: apply the exchange rate — real_balance = principal * (Ci_now / Ci_snapshot).
    pub fn balance(_env: Env, _account: Address) -> i128 {
        todo!("return DataKey::Balance(account); return 0 if not found")
    }

    pub fn allowance(_env: Env, _from: Address, _spender: Address) -> i128 {
        todo!("return stored allowance for (from, spender)")
    }

    pub fn approve(_env: Env, _from: Address, _spender: Address, _amount: i128, _expiry: u32) {
        todo!("require_auth from; store allowance; emit approve event")
    }

    // ── SEP-41 transfer ───────────────────────────────────────────────────────

    /// Transfer sTokens between accounts.
    ///
    /// # TODO
    /// 1. `from.require_auth()`
    /// 2. Assert `balance(from) >= amount`
    /// 3. Deduct from sender, add to recipient
    /// 4. Emit transfer event
    ///
    /// Note: if the sender has an active borrow using this deposit as
    /// collateral, transferring sTokens could drop their health factor below 1.
    /// The health factor check is enforced by the LendingPool, not here.
    pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {
        todo!("require_auth from; check balance; update balances; emit event")
    }

    pub fn transfer_from(
        _env: Env,
        _spender: Address,
        _from: Address,
        _to: Address,
        _amount: i128,
    ) {
        todo!("require_auth spender; check allowance; update balances and allowance; emit event")
    }

    pub fn burn(_env: Env, _from: Address, _amount: i128) {
        todo!("require_auth from; decrease balance and total_supply; emit burn event")
    }

    pub fn burn_from(_env: Env, _spender: Address, _from: Address, _amount: i128) {
        todo!(
            "require_auth spender; check allowance; decrease balance and total_supply; emit event"
        )
    }

    // ── Mint / Burn (pool only) ───────────────────────────────────────────────

    /// Mint `amount` sTokens to `to`. Called by LendingPool on deposit.
    ///
    /// # TODO
    /// 1. `pool.require_auth()` — load pool from DataKey::Pool
    /// 2. `DataKey::Balance(to) += amount`
    /// 3. `DataKey::TotalSupply += amount`
    pub fn mint(_env: Env, _to: Address, _amount: i128) {
        todo!("require_auth pool; increase Balance(to) and TotalSupply")
    }

    /// Burn `amount` sTokens from `from`. Called by LendingPool on withdraw/liquidation.
    ///
    /// # TODO
    /// 1. `pool.require_auth()`
    /// 2. Assert `balance(from) >= amount`
    /// 3. `DataKey::Balance(from) -= amount`
    /// 4. `DataKey::TotalSupply -= amount`
    pub fn pool_burn(_env: Env, _from: Address, _amount: i128) {
        todo!(
            "require_auth pool; assert sufficient balance; decrease Balance(from) and TotalSupply"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    #[should_panic]
    fn mint_unimplemented() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(SToken, ());
        let client = STokenClient::new(&env, &id);
        let pool = Address::generate(&env);
        let user = Address::generate(&env);
        client.initialize(&pool);
        client.mint(&user, &1000);
        assert_eq!(client.balance(&user), 1000);
    }
}
