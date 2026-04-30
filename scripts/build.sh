#!/bin/bash
# AjoChain — Build all contracts
set -euo pipefail

echo "🔨 Building AjoChain contracts..."

stellar contract build

echo ""
echo "✅ All contracts built successfully!"
echo ""
echo "📦 WASM artifacts:"
ls -lh target/wasm32-unknown-unknown/release/*.wasm 2>/dev/null || echo "  (check target/wasm32v1-none/release/ for newer Soroban versions)"
ls -lh target/wasm32v1-none/release/*.wasm 2>/dev/null || true
