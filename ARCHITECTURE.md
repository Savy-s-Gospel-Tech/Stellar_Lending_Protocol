# Architecture

The Stellar Lending Protocol is a non-custodial lending protocol built on Stellar using Soroban smart contracts. This document describes how the contracts are structured and how they interact.

## Contract Structure

```
contracts/
├── interfaces/                          # Cross-contract trait definitions
│   └── src/lib.rs                       # All interface traits in one crate
│
├── configuration/                       # Registry and admin
│   ├── LendingPoolAddressesProvider/    # Central registry — all contracts resolve each other here
│   ├── LendingPoolParametersProvider/   # Global protocol parameters (max LTV, liquidation threshold)
│   ├── AddressStorage/                  # Reusable address key-value storage primitive
│   └── UintStorage/                     # Reusable uint key-value storage primitive
│
├── lendingpool/                         # Core business logic
│   ├── LendingPool/                     # User-facing entry point: deposit, borrow, repay, liquidate, flash loan
│   ├── LendingPoolCore/                 # State holder: all reserve data, user positions, token custody
│   ├── LendingPoolDataProvider/         # Read-only calculations: health factor, account data, reserve data
│   ├── LendingPoolConfigurator/         # Admin: add reserves, set LTV/liquidation params, freeze/unfreeze
│   ├── LendingPoolLiquidationManager/   # Liquidation logic
│   └── DefaultReserveInterestRateStrategy/ # Two-slope interest rate model
│
├── tokenization/                        # Interest-bearing tokens
│   └── SToken/                          # SEP-41 receipt token — balance grows with interest
│
├── libraries/                           # Shared utilities
│   ├── CoreLibrary/                     # ReserveData and UserReserveData structs
│   ├── WadRayMath/                      # Fixed-point arithmetic: WAD (1e18) and RAY (1e27)
│   └── StellarAddressLib/               # Address utilities for Stellar (XLM sentinel address)
│
├── fees/                                # Fee management
│   ├── FeeProvider/                     # Origination fee and flash loan fee calculations
│   └── TokenDistributor/               # Protocol fee distribution
│
├── oracles/                             # Price and rate oracles
│   ├── PriceOracle/                     # Admin-fed USD price oracle (v0); Reflector integration (v1)
│   └── LendingRateOracle/              # Market lending rates — seeds the stable borrow rate
│
├── misc/                                # Peripheral utilities
│   ├── OracleAggregator/               # Aggregates oracle sources; implements IPriceOracle
│   └── WalletBalanceProvider/          # Batch balance queries for frontends and indexers
│
├── flashloan/                           # Flash loan infrastructure
│   ├── interfaces/
│   │   └── IFlashLoanReceiver/         # Interface every flash loan receiver must implement
│   └── base/
│       └── FlashLoanReceiverBase/      # Base contract with pool address wiring
│
└── mocks/                               # Test helpers
    └── FlashLoanReceiver/              # Mock receiver used in flash loan tests
```

## How Contracts Relate

```
AddressesProvider  ← single source of truth for all contract addresses
    │
    ├── LendingPool          ← users call this for all actions
    │       │
    │       ├── reads  AddressesProvider   (to resolve other contracts)
    │       ├── calls  LendingPoolCore     (state reads/writes)
    │       ├── calls  LendingPoolDataProvider  (health factor checks)
    │       ├── calls  InterestRateStrategy.calculate_interest_rates()
    │       ├── calls  PriceOracle.get_asset_price()
    │       ├── mints/burns  SToken        (on deposit/withdraw)
    │       └── delegates to  LiquidationManager  (on liquidation)
    │
    ├── LendingPoolCore      ← holds all state; only LendingPool can mutate it
    │       ├── stores  ReserveData  (per asset, defined in CoreLibrary)
    │       ├── stores  UserReserveData  (per user per asset)
    │       └── uses  WadRayMath  (all interest index math)
    │
    ├── PriceOracle          ← queried for USD prices (health factor, liquidation)
    └── LendingRateOracle    ← queried for market rates (stable rate seeding)
```

## Key Data Structures (CoreLibrary)

### ReserveData
Stored in LendingPoolCore, one per supported asset.

| Field | Type | Description |
|---|---|---|
| `total_liquidity` | i128 | Total deposited tokens (available + borrowed) |
| `total_variable_borrows` | i128 | Total outstanding variable-rate debt |
| `total_stable_borrows` | i128 | Total outstanding stable-rate debt |
| `avg_stable_borrow_rate` | i128 (RAY) | Weighted average stable rate across all stable borrowers |
| `liquidity_index` | i128 (RAY) | Ci — cumulative depositor interest index |
| `variable_borrow_index` | i128 (RAY) | Bvc — cumulative variable borrow index |
| `current_liquidity_rate` | i128 (RAY) | Current deposit APY |
| `current_variable_borrow_rate` | i128 (RAY) | Current variable borrow APY |
| `current_stable_borrow_rate` | i128 (RAY) | Current stable borrow APY |
| `last_update_ledger` | u32 | Ledger of last index update |
| `s_token_address` | Address | The sToken contract for this reserve |
| `interest_rate_strategy` | Address | The rate strategy contract for this reserve |
| `ltv_bps` | u32 | Max loan-to-value in basis points (e.g. 7500 = 75%) |
| `liquidation_threshold_bps` | u32 | Liquidation threshold in basis points |
| `liquidation_bonus_bps` | u32 | Liquidation bonus in basis points (e.g. 10500 = 5% bonus) |

### UserReserveData
Stored in LendingPoolCore, one per (user, asset) pair.

| Field | Type | Description |
|---|---|---|
| `principal_borrow_balance` | i128 | Debt principal at last borrow/repay |
| `variable_borrow_index` | i128 (RAY) | Bvc snapshot at last borrow |
| `stable_borrow_rate` | i128 (RAY) | User's locked-in stable rate (0 if none) |
| `origination_fee` | i128 | Fee owed, paid on repay |
| `use_as_collateral` | bool | Whether this deposit backs borrows |

## Interest Rate Model

`DefaultReserveInterestRateStrategy` implements a two-slope model. Each reserve has its own strategy contract with its own parameters.

```
U  = (total_variable_borrows + total_stable_borrows) / total_liquidity

if U ≤ U_optimal:
    variable_rate = base_rate + (U / U_optimal) × slope1

if U > U_optimal:
    variable_rate = base_rate + slope1 + ((U - U_optimal) / (1 - U_optimal)) × slope2

stable_rate  = market_rate (from LendingRateOracle) + stable_rate_offset
liquidity_rate = variable_rate × U × (1 - reserve_factor)
```

All values in RAY precision (1e27). Parameters (base_rate, slope1, slope2, U_optimal) are set per-reserve by the admin.

## Interest Accrual (Cumulative Indexes)

Interest accrues lazily — no per-user loop is needed.

Two indexes are maintained per reserve:

**Liquidity index (Ci)** — tracks depositor earnings:
```
Ci_new = Ci_old × (1 + Rl × Δt)
```
A depositor's real balance = `sToken_balance × (Ci_now / Ci_at_deposit)`.

**Variable borrow index (Bvc)** — tracks variable borrower debt:
```
Bvc_new = Bvc_old × (1 + Rv)^Δt   [approximated via 3-term binomial]
```
A variable borrower's real debt = `principal × (Bvc_now / Bvc_at_borrow)`.

Both indexes start at 1 RAY and only ever increase.

## Flash Loan Flow

```
1. User calls LendingPool::flash_loan(receiver, asset, amount, params)
2. Pool validates reserve has >= amount available liquidity
3. Pool records balance_before
4. Pool transfers amount of asset to receiver
5. Pool calls receiver::execute_operation(asset, amount, fee, initiator, params)
6. Receiver executes arbitrary logic (arbitrage, collateral swap, etc.)
7. Receiver transfers amount + fee back to pool
8. Pool asserts balance_after >= balance_before + fee
9. If assertion fails → entire transaction reverts
```

Fee is set in `FeeProvider` (default: 0.09% = 9 basis points).

## Deployment Order

Contracts must be deployed in dependency order:

1. WadRayMath, CoreLibrary, StellarAddressLib  (no deps)
2. LendingPoolAddressesProvider                (no deps)
3. PriceOracle, LendingRateOracle              (no deps)
4. FeeProvider, LendingPoolParametersProvider
5. DefaultReserveInterestRateStrategy          (one per reserve)
6. SToken                                      (one per reserve)
7. LendingPoolCore
8. LendingPoolDataProvider
9. LendingPoolConfigurator
10. LendingPoolLiquidationManager
11. LendingPool                                (wires everything together)
12. Register all addresses in LendingPoolAddressesProvider
