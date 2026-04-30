#!/bin/bash
# AjoChain — Run all tests
set -euo pipefail

echo "🧪 Running AjoChain test suite..."
echo ""

echo "── ajo-pool ──────────────────────────────────"
cargo test -p ajo-pool -- --nocapture
echo ""

echo "── ajo-factory ─────────────────────────────"
cargo test -p ajo-factory -- --nocapture
echo ""

echo "── ajo-collateral ──────────────────────────"
cargo test -p ajo-collateral -- --nocapture
echo ""

echo "── ajo-reputation ──────────────────────────"
cargo test -p ajo-reputation -- --nocapture
echo ""

echo "── ajo-governance ──────────────────────────"
cargo test -p ajo-governance -- --nocapture
echo ""

echo "✅ All tests passed!"
