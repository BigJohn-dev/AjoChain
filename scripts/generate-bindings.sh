#!/bin/bash
# AjoChain — Generate TypeScript bindings for frontend
set -euo pipefail

NETWORK="testnet"
OUTPUT_DIR="frontend/src/lib/contracts"

echo "🔗 Generating TypeScript contract bindings..."
echo ""

mkdir -p ${OUTPUT_DIR}

for contract in ajo-pool ajo-factory ajo-collateral ajo-reputation ajo-governance; do
    WASM_NAME=$(echo ${contract} | tr '-' '_')
    echo "  Generating bindings for ${contract}..."
    
    stellar contract bindings typescript \
        --wasm target/wasm32-unknown-unknown/release/${WASM_NAME}.wasm \
        --output-dir ${OUTPUT_DIR}/${contract} \
        --network ${NETWORK} 2>/dev/null || echo "    ⚠ Skipped (contract not deployed yet)"
done

echo ""
echo "✅ Bindings generated in ${OUTPUT_DIR}/"
