#!/bin/bash
# AjoChain — Deploy contracts to Stellar testnet
set -euo pipefail

NETWORK="testnet"
SOURCE="deployer"

echo "🚀 Deploying AjoChain to ${NETWORK}..."
echo ""

# Build first
echo "🔨 Building contracts..."
stellar contract build
echo ""

# Deploy Factory
echo "📦 Deploying ajo-factory..."
FACTORY_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_factory.wasm \
  --source ${SOURCE} \
  --network ${NETWORK})
echo "   Factory: ${FACTORY_ID}"

# Deploy Pool (template)
echo "📦 Deploying ajo-pool (template)..."
POOL_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_pool.wasm \
  --source ${SOURCE} \
  --network ${NETWORK})
echo "   Pool:    ${POOL_ID}"

# Deploy Collateral
echo "📦 Deploying ajo-collateral..."
COLLATERAL_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_collateral.wasm \
  --source ${SOURCE} \
  --network ${NETWORK})
echo "   Collateral: ${COLLATERAL_ID}"

# Deploy Reputation
echo "📦 Deploying ajo-reputation..."
REPUTATION_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_reputation.wasm \
  --source ${SOURCE} \
  --network ${NETWORK})
echo "   Reputation: ${REPUTATION_ID}"

# Deploy Governance
echo "📦 Deploying ajo-governance..."
GOVERNANCE_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_governance.wasm \
  --source ${SOURCE} \
  --network ${NETWORK})
echo "   Governance: ${GOVERNANCE_ID}"

echo ""
echo "✅ All contracts deployed!"
echo ""
echo "Contract IDs:"
echo "  FACTORY:    ${FACTORY_ID}"
echo "  POOL:       ${POOL_ID}"
echo "  COLLATERAL: ${COLLATERAL_ID}"
echo "  REPUTATION: ${REPUTATION_ID}"
echo "  GOVERNANCE: ${GOVERNANCE_ID}"
