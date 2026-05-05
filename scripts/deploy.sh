#!/usr/bin/env bash
# deploy.sh — deploy all SLP contracts to Stellar testnet
#
# Prerequisites:
#   stellar-cli installed: cargo install --locked stellar-cli --features opt
#   STELLAR_SECRET_KEY env var set to your testnet account secret key
#
# Usage:
#   chmod +x scripts/deploy.sh
#   STELLAR_SECRET_KEY=S... ./scripts/deploy.sh

set -euo pipefail

NETWORK="testnet"
SOURCE="${STELLAR_SECRET_KEY:?Set STELLAR_SECRET_KEY}"

echo "Building optimised WASMs..."
make build

deploy() {
  local name="$1"
  local wasm="target/wasm32-unknown-unknown/release/${name}.wasm"
  echo "Deploying $name..."
  stellar contract deploy \
    --wasm "$wasm" \
    --network "$NETWORK" \
    --source "$SOURCE"
}

# Deploy in dependency order
MATH_ID=$(deploy slp_math)
echo "slp-math:              $MATH_ID"

ORACLE_ID=$(deploy oracle)
echo "oracle:                $ORACLE_ID"

RATE_STRATEGY_ID=$(deploy interest_rate_strategy)
echo "interest-rate-strategy: $RATE_STRATEGY_ID"

TOKEN_ID=$(deploy slp_token)
echo "slp-token:             $TOKEN_ID"

POOL_ID=$(deploy lending_pool)
echo "lending-pool:          $POOL_ID"

CONFIG_ID=$(deploy configuration)
echo "configuration:         $CONFIG_ID"

LIQUIDATOR_ID=$(deploy liquidator)
echo "liquidator:            $LIQUIDATOR_ID"

echo ""
echo "All contracts deployed. Next steps:"
echo "  1. Call configuration.initialize(admin)"
echo "  2. Call configuration.set_lending_pool($POOL_ID)"
echo "  3. Call configuration.set_price_oracle($ORACLE_ID)"
echo "  4. Call lending_pool.initialize(admin, $ORACLE_ID)"
echo "  5. Call lending_pool.add_reserve(...) for each asset"
