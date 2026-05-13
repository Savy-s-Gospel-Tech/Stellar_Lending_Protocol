# Build Plan

This document defines the implementation order for the Stellar Lending Protocol.
Contracts are ordered by dependency — each contract can only be implemented after
the ones it depends on are complete and passing tests.

**Legend:** ✅ done · ⚠️ partial · 🔨 open

---

## Phase 1 — Foundation ✅

| # | Contract | Location | Status | Notes |
|---|---|---|---|---|
| 1 | WadRayMath | `contracts/libraries/WadRayMath` | ⚠️ | wad_* done; ray_mul/div need 256-bit (SLP-001); compound terms 2+3 open (SLP-002) |
| 2 | CoreLibrary | `contracts/libraries/CoreLibrary` | ✅ | Structs complete |
| 3 | StellarAddressLib | `contracts/libraries/StellarAddressLib` | ✅ | XLM SAC sentinel |

---

## Phase 2 — Infrastructure ✅

| # | Contract | Location | Status | Notes |
|---|---|---|---|---|
| 4 | AddressStorage | `contracts/configuration/AddressStorage` | ✅ | |
| 5 | UintStorage | `contracts/configuration/UintStorage` | ✅ | |
| 6 | LendingPoolAddressesProvider | `contracts/configuration/LendingPoolAddressesProvider` | ✅ | All 11 pairs, 4 tests |
| 7 | LendingPoolParametersProvider | `contracts/configuration/LendingPoolParametersProvider` | ✅ | Defaults set |

---

## Phase 3 — Oracles and Fees ✅

| # | Contract | Location | Status | Notes |
|---|---|---|---|---|
| 8 | PriceOracle | `contracts/oracles/PriceOracle` | ✅ | Admin-fed v0, 3 tests |
| 9 | LendingRateOracle | `contracts/oracles/LendingRateOracle` | ✅ | Mr default 0, 3 tests |
| 10 | FeeProvider | `contracts/fees/FeeProvider` | ⚠️ | Integer approx; wad_mul upgrade open as SLP-003 |

---

## Phase 4 — Interest Rate Model 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 11 | DefaultReserveInterestRateStrategy | `contracts/lendingpool/DefaultReserveInterestRateStrategy` | High | WadRayMath (SLP-001) |

**Issue:** SLP-007. Two-slope model: `Rv`, `Rs`, `Rl` from utilisation `U`. See `scripts/issues/issue-07.md`.

---

## Phase 5 — Token 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 12 | SToken | `contracts/tokenization/SToken` | High | — |

**Issue:** SLP-005. Full SEP-41: mint, burn, transfer, allowance. Pool-only mint/burn. See `scripts/issues/issue-05.md`.

---

## Phase 6 — Core State Layer 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 13 | LendingPoolCore | `contracts/lendingpool/LendingPoolCore` | High | WadRayMath, CoreLibrary, InterestRateStrategy |

**Issue:** SLP-006. Reserve storage, `update_cumulative_indexes`, `update_interest_rates`, balance mutations. See `scripts/issues/issue-06.md`.

---

## Phase 7 — Data and Configuration 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 14 | LendingPoolDataProvider | `contracts/lendingpool/LendingPoolDataProvider` | High | PoolCore, PriceOracle, WadRayMath |
| 15 | LendingPoolConfigurator | `contracts/lendingpool/LendingPoolConfigurator` | Medium | PoolCore, AddressesProvider |

---

## Phase 8 — Main Pool 🔨

| # | Action | Location | Complexity | Depends on |
|---|---|---|---|---|
| 16 | `deposit` + `withdraw` | `contracts/lendingpool/LendingPool` | High | PoolCore, SToken, DataProvider |
| 17 | `borrow` + `repay` | `contracts/lendingpool/LendingPool` | High | PoolCore, DataProvider, FeeProvider |
| 18 | `swap_borrow_rate_mode` | `contracts/lendingpool/LendingPool` | Medium | PoolCore |

**First working milestone:** deposit → earn interest → withdraw (actions 16 complete).

---

## Phase 9 — Liquidation 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 19 | LendingPoolLiquidationManager | `contracts/lendingpool/LendingPoolLiquidationManager` | High | DataProvider, PriceOracle, PoolCore |

---

## Phase 10 — Flash Loans 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 20 | FlashLoanReceiverBase | `contracts/flashloan/base/FlashLoanReceiverBase` | Medium | — |
| 21 | `flash_loan` | `contracts/lendingpool/LendingPool` | High | PoolCore, FeeProvider, FlashLoanReceiverBase |

---

## Phase 11 — Mocks (parallel, any time) 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 22 | MintableSEP41 | `contracts/mocks/tokens/MintableSEP41` | Trivial | — |
| 23 | MockFlashLoanReceiver | `contracts/mocks/flashloan/FlashLoanReceiver` | Trivial | FlashLoanReceiverBase |
| 24 | MockPoolCore | `contracts/mocks/upgradeability/MockPoolCore` | Medium | PoolCore interface |

---

## Phase 12 — Misc Utilities 🔨

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 25 | OracleAggregator | `contracts/misc/OracleAggregator` | Medium | PriceOracle |
| 26 | WalletBalanceProvider | `contracts/misc/WalletBalanceProvider` | Trivial | SToken |
| 27 | TokenDistributor | `contracts/fees/TokenDistributor` | Medium | FeeProvider |

---

## Open Contributor Issues

| Issue | Contract | Complexity | Blocked by |
|---|---|---|---|
| SLP-001 | WadRayMath — `ray_mul`/`ray_div` 256-bit | Medium | — |
| SLP-002 | WadRayMath — `calculate_compound_interest` terms 2+3 | Medium | SLP-001 |
| SLP-003 | FeeProvider — replace integer div with `wad_mul` | Trivial | SLP-002 |
| SLP-005 | SToken — full SEP-41 | High | — |
| SLP-006 | LendingPoolCore | High | SLP-001, SLP-002 |
| SLP-007 | DefaultReserveInterestRateStrategy | High | SLP-001 |

---

## Milestones

| Milestone | Requires | Status |
|---|---|---|
| **M0 — Apply to Drips Wave** | Phases 1–3 done | ✅ Ready |
| **M1 — Deposit/Withdraw** | Phases 4–8 (action 16) | 🔨 |
| **M2 — Borrow/Repay** | Phase 8 (action 17) | 🔨 |
| **M3 — Liquidations** | Phase 9 | 🔨 |
| **M4 — Flash Loans** | Phase 10 | 🔨 |
