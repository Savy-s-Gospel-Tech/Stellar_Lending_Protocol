//! # FeeProvider
//!
//! Calculates fees charged by the protocol. Isolated so fee logic can be
//! upgraded by governance without touching LendingPool or Core.
//!
//! ## Default fees
//! Origination fee: 0.0025% (25 bps / 10000) of the loan amount.
//! Charged on every borrow, added to the user's debt.
//!
//! ## Formula
//! origination_fee = amount * origination_fee_percentage / WAD

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

const WAD: i128 = 1_000_000_000_000_000_000;
/// 0.0025 * WAD
const DEFAULT_ORIGINATION_FEE_WAD: i128 = 2_500_000_000_000_000;

#[contracttype]
pub enum DataKey {
    AddressesProvider,
    OriginationFeePercentage,
}

#[contract]
pub struct FeeProvider;

#[contractimpl]
impl FeeProvider {
    pub fn initialize(env: Env, addresses_provider: Address) {
        assert!(
            !env.storage()
                .instance()
                .has(&DataKey::AddressesProvider),
            "already initialized"
        );
        env.storage()
            .instance()
            .set(&DataKey::AddressesProvider, &addresses_provider);
        env.storage()
            .instance()
            .set(&DataKey::OriginationFeePercentage, &DEFAULT_ORIGINATION_FEE_WAD);
    }

    /// Calculate the origination fee for a loan of `amount`.
    ///
    /// # TODO (SLP-003 contributor issue)
    /// Replace the integer approximation below with a proper wad_mul call
    /// once WadRayMath (SLP-001/002) is merged. The current implementation
    /// uses integer division which loses sub-WAD precision on small amounts.
    /// See issue SLP-003 for acceptance criteria.
    pub fn calculate_loan_origination_fee(env: Env, _user: Address, amount: i128) -> i128 {
        let fee_pct: i128 = env
            .storage()
            .instance()
            .get(&DataKey::OriginationFeePercentage)
            .expect("not initialized");
        // Approximation: amount * fee_pct / WAD
        // TODO (SLP-003): replace with wad_mul(amount, fee_pct)
        amount * fee_pct / WAD
    }

    pub fn get_loan_origination_fee_pct(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::OriginationFeePercentage)
            .expect("not initialized")
    }

    pub fn set_loan_origination_fee_pct(env: Env, fee_wad: i128) {
        assert!(fee_wad >= 0, "fee must be non-negative");
        // TODO: require_auth manager via AddressesProvider
        env.storage()
            .instance()
            .set(&DataKey::OriginationFeePercentage, &fee_wad);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn origination_fee_on_1000_tokens() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(FeeProvider, ());
        let client = FeeProviderClient::new(&env, &id);
        let provider = Address::generate(&env);
        let user = Address::generate(&env);
        client.initialize(&provider);
        // 0.0025% of 1_000_000_000 = 1_000_000_000 * 2_500_000_000_000_000 / 1e18 = 2_500_000
        let fee = client.calculate_loan_origination_fee(&user, &1_000_000_000i128);
        assert_eq!(fee, 2_500_000);
    }

    #[test]
    fn default_fee_pct_is_25_bps() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(FeeProvider, ());
        let client = FeeProviderClient::new(&env, &id);
        client.initialize(&Address::generate(&env));
        assert_eq!(client.get_loan_origination_fee_pct(), DEFAULT_ORIGINATION_FEE_WAD);
    }
}
