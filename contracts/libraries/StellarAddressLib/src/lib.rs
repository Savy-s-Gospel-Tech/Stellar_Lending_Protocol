//! # StellarAddressLib
//!
//! Address utilities for the Stellar Lending Protocol.
//!
//! On Stellar, the native asset (XLM) is not a contract — it lives at the
//! protocol level. When the lending pool needs to handle XLM alongside
//! SEP-41 contract tokens, it uses the Stellar Asset Contract (SAC) address
//! for XLM to represent it uniformly in reserve maps and storage keys.

#![no_std]

use soroban_sdk::{Address, Env};

/// Strkey of the XLM Stellar Asset Contract on Mainnet.
/// On Testnet this differs — callers should pass the correct address for
/// the target network rather than relying on this constant directly.
const XLM_SAC_MAINNET: &str = "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA";

/// Returns the Stellar Asset Contract address for native XLM.
/// This is the canonical sentinel used in reserve maps.
pub fn xlm_address(env: &Env) -> Address {
    Address::from_str(env, XLM_SAC_MAINNET)
}

/// Returns true if `address` is the native XLM sentinel.
pub fn is_xlm(env: &Env, address: &Address) -> bool {
    address == &xlm_address(env)
}
