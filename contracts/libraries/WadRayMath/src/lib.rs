//! # WadRayMath — fixed-point arithmetic
//!
//! All interest rate and index calculations in the protocol use fixed-point
//! arithmetic rather than floating point. Two precisions are used:
//!
//! | Unit | Value | Used for                              |
//! |------|-------|---------------------------------------|
//! | WAD  | 1e18  | token amounts, exchange rates         |
//! | RAY  | 1e27  | interest rates, cumulative indexes    |
//!
//! ## Why two precisions?
//! Interest rates are tiny numbers (e.g. 5% APY = 0.05). Storing them as
//! integers requires scaling up by a large factor to preserve precision.
//! RAY (1e27) gives 9 more decimal places than WAD, which matters when
//! compounding rates over millions of ledger steps.
//!
//! ## Rounding
//! Every mul/div rounds half-up. This prevents systematic precision loss
//! that would accumulate over time and disadvantage depositors or borrowers.
//!
//! ## Overflow
//! i128 max ≈ 1.7e38. RAY = 1e27, so ray_mul(a, b) computes a*b which can
//! reach 1e54 — overflows i128. Implementations must use i256 or u256
//! intermediate values. In Soroban, use `soroban_sdk::I256` or cast via u128
//! with careful bounds checking.

/// 1 ray = 1e27
pub const RAY: i128 = 1_000_000_000_000_000_000_000_000_000;
/// 1 wad = 1e18
pub const WAD: i128 = 1_000_000_000_000_000_000;
/// RAY / WAD = 1e9
pub const WAD_RAY_RATIO: i128 = 1_000_000_000;

/// Multiply two ray values, rounding half-up.
///
/// Formula: (a * b + RAY/2) / RAY
///
/// # TODO
/// Use 256-bit intermediate arithmetic to avoid overflow.
/// In Soroban: cast to i256 via `soroban_sdk::I256::from_i128(&env, a)`,
/// multiply, add RAY/2, divide by RAY, cast back.
pub fn ray_mul(_a: i128, _b: i128) -> i128 {
    todo!("(a * b + RAY/2) / RAY — use i256 intermediate to avoid overflow")
}

/// Divide two ray values, rounding half-up.
///
/// Formula: (a * RAY + b/2) / b
///
/// # TODO
/// Panic if b == 0. Use 256-bit intermediate for a * RAY.
pub fn ray_div(_a: i128, _b: i128) -> i128 {
    todo!("assert b != 0; (a * RAY + b/2) / b — use i256 intermediate")
}

/// Multiply two wad values, rounding half-up.
///
/// Formula: (a * b + WAD/2) / WAD
pub fn wad_mul(_a: i128, _b: i128) -> i128 {
    todo!("(a * b + WAD/2) / WAD — use i256 intermediate")
}

/// Divide two wad values, rounding half-up.
///
/// Formula: (a * WAD + b/2) / b
pub fn wad_div(_a: i128, _b: i128) -> i128 {
    todo!("assert b != 0; (a * WAD + b/2) / b — use i256 intermediate")
}

/// Convert ray to wad, rounding half-up.
///
/// Drops 9 decimal places of precision.
pub fn ray_to_wad(_r: i128) -> i128 {
    todo!("(_r + WAD_RAY_RATIO / 2) / WAD_RAY_RATIO")
}

/// Convert wad to ray (exact, no precision loss).
pub fn wad_to_ray(_w: i128) -> i128 {
    todo!("_w * WAD_RAY_RATIO — checked_mul to avoid overflow")
}

/// Approximate (1 + rate)^n using a 3-term binomial expansion.
///
/// Used to compound the variable borrow index over `n` ledgers:
///   index_new = index_old * calculate_compound_interest(rate_per_ledger, n)
///
/// Formula (x = rate_per_ledger in RAY):
///   result = 1 + n*x + n*(n-1)/2 * x² + n*(n-1)*(n-2)/6 * x³
///
/// # Why binomial approximation?
/// The exact formula requires exponentiation which is expensive on-chain.
/// Three terms give sufficient precision for Soroban's ~5-second ledger
/// intervals at realistic interest rates (< 200% APY).
///
/// # TODO
/// Implement using ray_mul for each term. Return RAY (= 1.0) when n == 0.
pub fn calculate_compound_interest(_rate_per_ledger: i128, _n: u64) -> i128 {
    todo!(
        "if n == 0: return RAY
         x = rate_per_ledger
         term1 = n * x
         term2 = n*(n-1)/2 * ray_mul(x, x)
         term3 = n*(n-1)*(n-2)/6 * ray_mul(ray_mul(x,x), x)
         return RAY + term1 + term2 + term3"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn ray_mul_half_times_two() {
        assert_eq!(ray_mul(RAY / 2, 2 * RAY), RAY);
    }

    #[test]
    #[should_panic]
    fn ray_div_one_by_two() {
        assert_eq!(ray_div(RAY, 2 * RAY), RAY / 2);
    }

    #[test]
    #[should_panic]
    fn compound_zero_ledgers_is_one() {
        assert_eq!(calculate_compound_interest(RAY / 100, 0), RAY);
    }
}
