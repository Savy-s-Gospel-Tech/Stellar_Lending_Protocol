//! # WalletBalanceProvider (contracts/misc/)
//!
//! Off-chain helper contract — not part of the core protocol.
//! Allows frontends and indexers to batch-query token balances and
//! sToken balances for a user across multiple assets in a single call.
//!


#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct BalanceResult {
    pub asset: Address,
    pub balance: i128,
}

#[contract]
pub struct WalletBalanceProvider;

#[contractimpl]
impl WalletBalanceProvider {
    /// Return the token balance of `user` for each asset in `assets`.
    ///
    /// # TODO
    /// For each asset in `assets`:
    ///   1. Call the SEP-41 `balance(user)` on the asset contract
    ///   2. Push BalanceResult { asset, balance } into result vec
    pub fn batch_balance_of(
        _env: Env,
        _user: Address,
        _assets: Vec<Address>,
    ) -> Vec<BalanceResult> {
        todo!("iterate assets, query SEP-41 balance for user, return vec")
    }

    /// Return the token balance of `user` for a single `asset`.
    ///
    /// # TODO: call SEP-41 balance(user) on asset contract
    pub fn balance_of(_env: Env, _user: Address, _asset: Address) -> i128 {
        todo!("call asset.balance(user)")
    }
}
