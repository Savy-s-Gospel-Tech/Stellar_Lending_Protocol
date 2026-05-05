//! # StellarAddressLib
//!
//! Address utilities for the Stellar Lending Protocol.
//!
//! On Stellar, the native asset (XLM) is not a contract — it lives at the
//! protocol level. When the lending pool needs to handle XLM alongside
//! SEP-41 contract tokens, it uses a sentinel address to represent XLM
//! uniformly in reserve maps and storage keys.
//!
//! This library provides that sentinel and the check to identify it.

#![no_std]

use soroban_sdk::{Address, Env};

/// Returns the sentinel address that represents native XLM in reserve maps.
///
/// # TODO
/// Decide on a canonical sentinel. Options:
///   a) Use the Stellar Asset Contract address for XLM on the target network
///   b) Use a well-known fixed address (e.g. all-zeros)
/// The Stellar Asset Contract approach is preferred — it means XLM can be
/// handled via the same SEP-41 interface as every other asset.
pub fn xlm_address(_env: &Env) -> Address {
    todo!("return the Stellar Asset Contract address for native XLM")
}

/// Returns true if `address` is the native XLM sentinel.
pub fn is_xlm(_env: &Env, _address: &Address) -> bool {
    todo!("return address == xlm_address(env)")
}
