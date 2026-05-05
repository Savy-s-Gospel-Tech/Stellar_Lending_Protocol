# Build Plan

This document defines the implementation order for the Stellar Lending Protocol.
Contracts are ordered by dependency — each contract can only be implemented after
the ones it depends on are complete and passing tests.

---

## Phase 1 — Foundation

Nothing else in the protocol works without these.

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 1 | WadRayMath | `contracts/libraries/WadRayMath` | Medium | — |
| 2 | CoreLibrary | `contracts/libraries/CoreLibrary` | Medium | WadRayMath |
| 3 | StellarAddressLib | `contracts/libraries/StellarAddressLib` | Trivial | — |

**Why first:** Every interest calculation in the protocol uses `ray_mul`, `ray_div`,
and `calculate_compound_interest` from WadRayMath. CoreLibrary defines the
`ReserveData` and `UserReserveData` structs that PoolCore stores and every other
contract reads.

---

## Phase 2 — Infrastructure

The registry and storage layer. Required before any cross-contract call can be made.

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 4 | AddressStorage | `contracts/configuration/AddressStorage` | Trivial | — |
| 5 | UintStorage | `contracts/configuration/UintStorage` | Trivial | — |
| 6 | LendingPoolAddressesProvider | `contracts/configuration/LendingPoolAddressesProvider` | Medium | AddressStorage |
| 7 | LendingPoolParametersProvider | `contracts/configuration/LendingPoolParametersProvider` | Trivial | AddressesProvider |

**Why second:** All contracts resolve each other's addresses through
`LendingPoolAddressesProvider`. No integration test can run until this is working.

---

## Phase 3 — Oracles and Fees

Required before any borrow position can be validated.

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 8 | PriceOracle | `contracts/oracles/PriceOracle` | Trivial | — |
| 9 | LendingRateOracle | `contracts/oracles/LendingRateOracle` | Trivial | — |
| 10 | FeeProvider | `contracts/fees/FeeProvider` | Trivial | ParametersProvider |

**Why third:** `LendingPoolDataProvider` needs `PriceOracle` to calculate collateral
values and health factors. `LendingPool::borrow` needs `FeeProvider` to calculate
the origination fee before executing.

---

## Phase 4 — Interest Rate Model

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 11 | DefaultReserveInterestRateStrategy | `contracts/lendingpool/DefaultReserveInterestRateStrategy` | High | WadRayMath |

**Why fourth:** `LendingPoolCore::update_interest_rates` calls this contract after
every deposit, borrow, repay, and withdraw. It must be deployed per-reserve before
any reserve can be initialised.

---

## Phase 5 — Token

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 12 | SToken | `contracts/tokenization/SToken` | High | — |

**Why fifth:** `LendingPool::deposit` mints sTokens and `LendingPool::withdraw`
burns them. The full SEP-41 interface (mint, burn, transfer, balance, allowance)
must be implemented and tested before the deposit/withdraw flow can work end-to-end.

---

## Phase 6 — Core State Layer

The most complex contract. Holds all protocol state and all deposited funds.

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 13 | LendingPoolCore | `contracts/lendingpool/LendingPoolCore` | High | WadRayMath, CoreLibrary, InterestRateStrategy |

**Why sixth:** Every user action (deposit, borrow, repay, liquidate) reads and
writes state through PoolCore. It must be fully working before the main pool can
be implemented. Suggested sub-issues:
- Reserve init and storage
- `update_cumulative_indexes` (index math)
- `update_interest_rates` (cross-contract call to strategy)
- Liquidity and borrow balance mutations

---

## Phase 7 — Data and Configuration

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 14 | LendingPoolDataProvider | `contracts/lendingpool/LendingPoolDataProvider` | High | PoolCore, PriceOracle, WadRayMath |
| 15 | LendingPoolConfigurator | `contracts/lendingpool/LendingPoolConfigurator` | Medium | PoolCore, AddressesProvider |

**Why seventh:** `LendingPoolDataProvider` calculates health factors — `LendingPool`
calls it before every borrow and withdraw. `LendingPoolConfigurator` initialises
reserves — no reserve can be used until it has been configured.

---

## Phase 8 — Main Pool

The user-facing contract. Implement actions in order of complexity.

| # | Contract / Action | Location | Complexity | Depends on |
|---|---|---|---|---|
| 16 | LendingPool — `deposit` + `withdraw` | `contracts/lendingpool/LendingPool` | High | PoolCore, SToken, DataProvider |
| 17 | LendingPool — `borrow` + `repay` | `contracts/lendingpool/LendingPool` | High | PoolCore, DataProvider, FeeProvider |
| 18 | LendingPool — `swap_borrow_rate_mode` | `contracts/lendingpool/LendingPool` | Medium | PoolCore |

**Why eighth:** Deposit and withdraw are the simplest user actions and the first
integration milestone. Borrow and repay depend on health factor checks being
working. Rate swap is the least critical and can come last.

---

## Phase 9 — Liquidation

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 19 | LendingPoolLiquidationManager | `contracts/lendingpool/LendingPoolLiquidationManager` | High | DataProvider, PriceOracle, PoolCore |

**Why ninth:** Liquidation depends on health factor calculation (DataProvider),
collateral valuation (PriceOracle), and state mutations (PoolCore) all being
correct. It is the last core protocol feature before flash loans.

---

## Phase 10 — Flash Loans

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 20 | FlashLoanReceiverBase | `contracts/flashloan/base/FlashLoanReceiverBase` | Medium | — |
| 21 | LendingPool — `flash_loan` | `contracts/lendingpool/LendingPool` | High | PoolCore, FeeProvider, FlashLoanReceiverBase |

**Why tenth:** Flash loans are the last major feature. They depend on the pool's
liquidity accounting being correct and the fee system being in place.

---

## Phase 11 — Mocks (test infrastructure, runs in parallel)

These can be worked on at any point alongside the main phases.

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 22 | MintableSEP41 | `contracts/mocks/tokens/MintableSEP41` | Trivial | — |
| 23 | MockFlashLoanReceiver | `contracts/mocks/flashloan/FlashLoanReceiver` | Trivial | FlashLoanReceiverBase |
| 24 | MockPoolCore | `contracts/mocks/upgradeability/MockPoolCore` | Medium | PoolCore interface |

**Why parallel:** `MintableSEP41` is needed for integration tests (deposit/borrow
flows need a real token to move around). `MockFlashLoanReceiver` is needed to test
the flash loan flow. `MockPoolCore` is needed to test `LendingPool` in isolation
without a full PoolCore deployment.

---

## Phase 12 — Misc Utilities

| # | Contract | Location | Complexity | Depends on |
|---|---|---|---|---|
| 25 | OracleAggregator | `contracts/misc/OracleAggregator` | Medium | PriceOracle, IPriceFeed |
| 26 | WalletBalanceProvider | `contracts/misc/WalletBalanceProvider` | Trivial | SToken |
| 27 | TokenDistributor | `contracts/fees/TokenDistributor` | Medium | FeeProvider |

**Why last:** These are peripheral utilities — useful for frontends and fee
distribution but not required for the core protocol to function.

---

## Quick Reference

```
Phase 1  →  WadRayMath, CoreLibrary, StellarAddressLib
Phase 2  →  AddressStorage, UintStorage, AddressesProvider, ParametersProvider
Phase 3  →  PriceOracle, LendingRateOracle, FeeProvider
Phase 4  →  InterestRateStrategy
Phase 5  →  SToken
Phase 6  →  LendingPoolCore
Phase 7  →  LendingPoolDataProvider, LendingPoolConfigurator
Phase 8  →  LendingPool (deposit/withdraw → borrow/repay → swap rate)
Phase 9  →  LiquidationManager
Phase 10 →  FlashLoanReceiverBase, LendingPool flash_loan
Phase 11 →  Mocks (parallel)
Phase 12 →  Misc utilities
```

First working milestone: **deposit → earn interest → withdraw** (Phases 1–8, actions 16).
Second milestone: **borrow → repay** (Phase 8, actions 17).
Third milestone: **liquidation** (Phase 9).
Fourth milestone: **flash loans** (Phase 10).
