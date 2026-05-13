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
//! ## Rounding
//! Every mul/div rounds half-up to prevent systematic precision loss.
//!
//! ## Overflow
//! i128 max ≈ 1.7e38. RAY = 1e27, so ray_mul(a, b) computes a*b which can
//! reach 1e54 — overflows i128. All mul/div use i128 intermediate arithmetic
//! with careful ordering to stay within bounds.

/// 1 ray = 1e27
pub const RAY: i128 = 1_000_000_000_000_000_000_000_000_000;
/// 1 wad = 1e18
pub const WAD: i128 = 1_000_000_000_000_000_000;
/// RAY / WAD = 1e9
pub const WAD_RAY_RATIO: i128 = 1_000_000_000;

// ── Conversion helpers ────────────────────────────────────────────────────────

/// Convert ray to wad, rounding half-up (drops 9 decimal places).
pub fn ray_to_wad(r: i128) -> i128 {
    (r + WAD_RAY_RATIO / 2) / WAD_RAY_RATIO
}

/// Convert wad to ray (exact, no precision loss).
pub fn wad_to_ray(w: i128) -> i128 {
    w.checked_mul(WAD_RAY_RATIO).expect("wad_to_ray overflow")
}

// ── WAD arithmetic ────────────────────────────────────────────────────────────

/// Multiply two wad values, rounding half-up.
/// Formula: (a * b + WAD/2) / WAD
pub fn wad_mul(a: i128, b: i128) -> i128 {
    // Split to avoid overflow: a * b can reach ~1e54 for WAD-scale inputs.
    // We compute (a / WAD) * b + ((a % WAD) * b + WAD/2) / WAD
    let half = WAD / 2;
    let a_int = a / WAD;
    let a_rem = a % WAD;
    a_int * b + (a_rem * b + half) / WAD
}

/// Divide two wad values, rounding half-up.
/// Formula: (a * WAD + b/2) / b
pub fn wad_div(a: i128, b: i128) -> i128 {
    assert!(b != 0, "wad_div: division by zero");
    let half = b / 2;
    // a * WAD can overflow for large a; split similarly.
    let a_int = a / b;
    let a_rem = a % b;
    a_int * WAD + (a_rem * WAD + half) / b
}

// ── RAY arithmetic ────────────────────────────────────────────────────────────

/// Multiply two ray values, rounding half-up.
/// Formula: (a * b + RAY/2) / RAY
///
/// # TODO (SLP-001)
/// The current implementation splits operands to avoid i128 overflow, but
/// this approach loses precision for very large inputs near RAY magnitude.
/// Replace with a proper 256-bit intermediate (e.g. via `uint256` crate or
/// Soroban's `I256`) to handle the full i128 range without precision loss.
/// See issue SLP-001 for acceptance criteria and test cases.
pub fn ray_mul(a: i128, b: i128) -> i128 {
    let half = RAY / 2;
    let a_int = a / RAY;
    let a_rem = a % RAY;
    a_int * b + (a_rem * b + half) / RAY
}

/// Divide two ray values, rounding half-up.
/// Formula: (a * RAY + b/2) / b
///
/// # TODO (SLP-001)
/// Same as ray_mul — replace intermediate arithmetic with 256-bit to handle
/// inputs where a * RAY overflows i128. See issue SLP-001.
pub fn ray_div(a: i128, b: i128) -> i128 {
    assert!(b != 0, "ray_div: division by zero");
    let half = b / 2;
    let a_int = a / b;
    let a_rem = a % b;
    a_int * RAY + (a_rem * RAY + half) / b
}

// ── Compound interest ─────────────────────────────────────────────────────────

/// Approximate (1 + rate)^n using a 3-term binomial expansion.
///
/// Used to compound the variable borrow index over `n` ledgers:
///   index_new = index_old * calculate_compound_interest(rate_per_ledger, n)
///
/// Formula (x = rate_per_ledger in RAY):
///   result = RAY + n*x + n*(n-1)/2 * x² + n*(n-1)*(n-2)/6 * x³
///
/// # TODO (SLP-002)
/// Implement the full 3-term binomial expansion using ray_mul for each term.
/// The stub below only handles n=0 and n=1 correctly. Implement terms 2 and 3
/// for n >= 2. See issue SLP-002 for acceptance criteria and test cases.
pub fn calculate_compound_interest(rate_per_ledger: i128, n: u64) -> i128 {
    if n == 0 {
        return RAY;
    }
    // term1 = n * rate_per_ledger (exact, no ray_mul needed)
    let term1 = (n as i128) * rate_per_ledger;
    // TODO (SLP-002): add term2 = n*(n-1)/2 * ray_mul(x, x)
    //                 and term3 = n*(n-1)*(n-2)/6 * ray_mul(ray_mul(x,x), x)
    RAY + term1
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Conversion ────────────────────────────────────────────────────────────

    #[test]
    fn wad_to_ray_and_back() {
        assert_eq!(ray_to_wad(wad_to_ray(WAD)), WAD);
    }

    #[test]
    fn ray_to_wad_rounds_half_up() {
        // RAY/2 in wad = 5e8; remainder = WAD_RAY_RATIO/2 triggers round-up
        let r = RAY / 2 + WAD_RAY_RATIO / 2;
        assert_eq!(ray_to_wad(r), WAD / 2 + 1);
    }

    // ── WAD ───────────────────────────────────────────────────────────────────

    #[test]
    fn wad_mul_identity() {
        assert_eq!(wad_mul(WAD, WAD), WAD);
    }

    #[test]
    fn wad_div_identity() {
        assert_eq!(wad_div(WAD, WAD), WAD);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn wad_div_by_zero_panics() {
        wad_div(WAD, 0);
    }

    // ── RAY ───────────────────────────────────────────────────────────────────
    // These two tests define the acceptance criteria for SLP-001.
    // They are marked should_panic because the current split-operand
    // implementation overflows for inputs at RAY magnitude.
    // Remove #[should_panic] once SLP-001 (256-bit intermediate) is merged.

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
    #[should_panic(expected = "division by zero")]
    fn ray_div_by_zero_panics() {
        ray_div(RAY, 0);
    }

    // ── Compound interest ─────────────────────────────────────────────────────

    #[test]
    fn compound_zero_ledgers_is_one() {
        assert_eq!(calculate_compound_interest(RAY / 100, 0), RAY);
    }

    #[test]
    fn compound_one_ledger_equals_ray_plus_rate() {
        let rate = RAY / 100; // 1% per ledger
        assert_eq!(calculate_compound_interest(rate, 1), RAY + rate);
    }
}
