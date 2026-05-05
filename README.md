# Stellar Lending Protocol

A decentralised, non-custodial lending protocol built on [Stellar](https://stellar.org) using [Soroban](https://developers.stellar.org/docs/build/smart-contracts) smart contracts.

Users can deposit assets to earn interest, borrow against their collateral, and participate in liquidations — all governed by on-chain smart contracts with no intermediary.

> **Status:** Active development — v0 (core deposit/borrow/repay loop). Contributions welcome via [Drips Wave](https://drips.network/wave).

---

## The Problem

Stellar has fast, cheap transactions and a growing asset ecosystem (USDC, XLM, tokenised real-world assets via anchors). But there is no native on-chain lending market. Holders of Stellar assets have no way to earn yield on idle capital or borrow against their holdings without trusting a centralised intermediary.

This protocol solves that.

---

## How It Works

**Liquidity Pools**
Each supported asset has its own reserve pool. Depositors supply assets and receive **sTokens** — SEP-41 interest-bearing receipt tokens. The sToken balance grows automatically as interest accrues.

**Over-collateralised Borrowing**
To borrow, a user deposits collateral worth more than the loan. Each asset has a configurable Loan-to-Value (LTV) ratio. The protocol tracks a **health factor** for every borrowing position:

```
health_factor = Σ(collateral_i × liquidation_threshold_i) / total_debt
```

A position is healthy when `health_factor >= 1.0`.

**Liquidations**
When a position's health factor drops below 1 (collateral value falls or debt grows), anyone can liquidate it. The liquidator repays part of the debt and receives the collateral at a discount (the liquidation bonus). This keeps the protocol solvent.

**Interest Rate Model**
Rates are set algorithmically based on utilisation (`U = total_borrows / total_liquidity`):

```
if U ≤ U_optimal:
    borrow_rate = base_rate + (U / U_optimal) × slope1
else:
    borrow_rate = base_rate + slope1 + ((U - U_optimal) / (1 - U_optimal)) × slope2
```

Low utilisation → cheap rates to attract borrowers. High utilisation → rates spike to incentivise repayment and attract new deposits.

**Flash Loans**
Borrow any amount with zero collateral, as long as it is repaid within the same transaction. Enables arbitrage, collateral swaps, and self-liquidation.

---

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for full details.

```
contracts/
├── interfaces/              # Cross-contract trait definitions
├── configuration/           # Registry and admin
│   ├── LendingPoolAddressesProvider/   # Central registry of all contract addresses
│   ├── LendingPoolParametersProvider/  # Global protocol parameters
│   ├── AddressStorage/                 # Reusable address storage primitive
│   └── UintStorage/                    # Reusable uint storage primitive
├── lendingpool/             # Core business logic
│   ├── LendingPool/                    # User entry point: deposit, borrow, repay, liquidate, flash loan
│   ├── LendingPoolCore/                # State holder: reserves, user positions, token custody
│   ├── LendingPoolDataProvider/        # Read-only: health factor, account data
│   ├── LendingPoolConfigurator/        # Admin: add reserves, set parameters
│   ├── LendingPoolLiquidationManager/  # Liquidation logic
│   └── DefaultReserveInterestRateStrategy/ # Two-slope interest rate model
├── tokenization/            # Interest-bearing tokens
│   └── SToken/              # SEP-41 receipt token — balance grows with interest
├── libraries/               # Shared utilities
│   ├── CoreLibrary/         # ReserveData & UserReserveData structs
│   ├── WadRayMath/          # Fixed-point arithmetic (WAD/RAY precision)
│   └── StellarAddressLib/   # Address utilities for Stellar (XLM sentinel)
├── fees/                    # Fee management
│   ├── FeeProvider/         # Origination and flash loan fee calculations
│   └── TokenDistributor/    # Protocol fee distribution
├── oracles/                 # Price and rate oracles
│   ├── PriceOracle/         # Admin-fed USD prices (v0) → Reflector (v1)
│   └── LendingRateOracle/   # Market lending rates for stable rate seeding
├── misc/                    # Peripheral utilities
│   ├── OracleAggregator/    # Aggregates oracle sources; Reflector integration (v1)
│   └── WalletBalanceProvider/  # Batch balance queries for frontends
├── flashloan/               # Flash loan infrastructure
│   ├── interfaces/IFlashLoanReceiver/  # Interface receivers must implement
│   └── base/FlashLoanReceiverBase/     # Base contract with pool wiring
└── mocks/                   # Test helpers
    └── FlashLoanReceiver/   # Mock receiver for flash loan tests
```

---

## Roadmap

| Milestone | Description | Status |
|-----------|-------------|--------|
| v0 | Core deposit/borrow/repay, single-asset health check | 🔨 In progress |
| v1 | Multi-asset collateral, utilisation-based interest curve | 📋 Planned |
| v2 | Reflector oracle integration, flash loans | 📋 Planned |
| v3 | Governance token, protocol fee switch | 📋 Planned |

---

## Getting Started

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Build

```bash
make build
```

### Test

```bash
make test
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/lending_pool.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>
```

---

## Contributing

This project participates in the **Stellar Wave** program on [Drips](https://drips.network/wave). Issues tagged `Stellar Wave` are bounty-eligible.

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT
