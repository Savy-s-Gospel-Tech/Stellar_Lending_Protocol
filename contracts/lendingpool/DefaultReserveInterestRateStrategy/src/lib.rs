//! # DefaultReserveInterestRateStrategy
//!
//! Implements the two-slope interest rate model. Each reserve has its own
//! instance of this contract with its own tuned parameters.
//!
//! ## How it works
//!
//! The interest rate is a function of utilisation (U):
//!   U = total_borrows / total_liquidity
//!
//! When utilisation is low, rates are cheap to attract borrowers.
//! When utilisation is high (liquidity is scarce), rates spike to incentivise
//! repayment and attract new deposits.
//!
//! ## Formulas (all values in RAY = 1e27)
//!
//! Utilisation:
//!   U = (total_variable_borrows + total_stable_borrows) / total_liquidity
//!
//! Variable borrow rate (Rv):
//!   if U <= U_optimal:
//!     Rv = base_rate + (U / U_optimal) * slope1
//!   else:
//!     Rv = base_rate + slope1 + ((U - U_optimal) / (1 - U_optimal)) * slope2
//!
//! Stable borrow rate (Rs):
//!   Same two-slope formula but uses market_lending_rate (from LendingRateOracle)
//!   as the base instead of base_rate.
//!
//! Overall borrow rate (Ro):
//!   Ro = (total_variable_borrows * Rv + total_stable_borrows * avg_stable_rate)
//!        / total_borrows
//!
//! Liquidity rate (Rl) — earned by depositors:
//!   Rl = Ro * U

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env};

/// 1 ray = 1e27
pub const RAY: i128 = 1_000_000_000_000_000_000_000_000_000;

#[contracttype]
#[derive(Clone)]
pub struct StrategyParams {
    /// Base variable borrow rate at U=0 (RAY).
    pub base_variable_rate: i128,
    /// Target utilisation rate (RAY). e.g. 0.8 * RAY = 80%.
    pub optimal_utilisation: i128,
    /// Rate slope below U_optimal (RAY).
    pub slope1: i128,
    /// Rate slope above U_optimal (RAY). Much steeper to deter over-utilisation.
    pub slope2: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct RateCalcInput {
    /// Total tokens deposited in the reserve.
    pub total_liquidity: i128,
    /// Total outstanding variable-rate debt.
    pub total_variable_borrows: i128,
    /// Total outstanding stable-rate debt.
    pub total_stable_borrows: i128,
    /// Weighted average stable borrow rate across all stable positions (RAY).
    pub avg_stable_rate: i128,
    /// Market lending rate from LendingRateOracle (RAY). Used as base for Rs.
    pub market_lending_rate: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct RateCalcOutput {
    /// Variable borrow rate (RAY).
    pub variable_rate: i128,
    /// Stable borrow rate for new positions (RAY).
    pub stable_rate: i128,
    /// Liquidity rate earned by depositors (RAY).
    pub liquidity_rate: i128,
}

#[contract]
pub struct InterestRateStrategy;

#[contractimpl]
impl InterestRateStrategy {
    /// Calculate current rates given reserve state. Pure function — no storage.
    ///
    /// # TODO
    /// 1. total_borrows = total_variable_borrows + total_stable_borrows
    /// 2. U = if total_liquidity == 0 then 0 else total_borrows * RAY / total_liquidity
    /// 3. Apply two-slope formula for Rv (using base_variable_rate as base)
    /// 4. Apply two-slope formula for Rs (using market_lending_rate as base)
    /// 5. Ro = if total_borrows == 0 then 0
    ///         else (total_variable_borrows * Rv + total_stable_borrows * avg_stable_rate)
    ///              / total_borrows
    /// 6. Rl = Ro * U / RAY
    /// 7. Return RateCalcOutput { variable_rate: Rv, stable_rate: Rs, liquidity_rate: Rl }
    pub fn calculate_rates(
        _env: Env,
        _params: StrategyParams,
        _input: RateCalcInput,
    ) -> RateCalcOutput {
        todo!("implement two-slope model — see module doc for formulas")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    fn default_params() -> StrategyParams {
        StrategyParams {
            base_variable_rate: RAY / 100,     // 1%
            optimal_utilisation: RAY * 8 / 10, // 80%
            slope1: RAY * 4 / 100,             // 4%
            slope2: RAY * 75 / 100,            // 75%
        }
    }

    /// At U=0: Rv == base_rate, Rl == 0
    #[test]
    #[should_panic]
    fn rate_at_zero_utilisation() {
        let env = Env::default();
        let id = env.register(InterestRateStrategy, ());
        let client = InterestRateStrategyClient::new(&env, &id);
        let out = client.calculate_rates(
            &default_params(),
            &RateCalcInput {
                total_liquidity: 1_000_000,
                total_variable_borrows: 0,
                total_stable_borrows: 0,
                avg_stable_rate: 0,
                market_lending_rate: RAY * 9 / 100,
            },
        );
        assert_eq!(out.variable_rate, RAY / 100);
        assert_eq!(out.liquidity_rate, 0);
    }

    /// At U=U_optimal: Rv == base_rate + slope1
    #[test]
    #[should_panic]
    fn rate_at_optimal_utilisation() {
        let env = Env::default();
        let id = env.register(InterestRateStrategy, ());
        let client = InterestRateStrategyClient::new(&env, &id);
        let params = default_params();
        let out = client.calculate_rates(
            &params,
            &RateCalcInput {
                total_liquidity: 1_000_000,
                total_variable_borrows: 800_000,
                total_stable_borrows: 0,
                avg_stable_rate: 0,
                market_lending_rate: RAY * 9 / 100,
            },
        );
        assert_eq!(out.variable_rate, params.base_variable_rate + params.slope1);
    }
}
