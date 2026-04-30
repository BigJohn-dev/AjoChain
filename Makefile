.PHONY: build test fmt lint clean check audit

# ─── Build ───────────────────────────────────────────────────────────────

## Build all contracts for WASM deployment
build:
	cargo build --release --target wasm32-unknown-unknown

## Build with the Stellar CLI (preferred)
build-stellar:
	stellar contract build

# ─── Test ────────────────────────────────────────────────────────────────

## Run all tests
test:
	cargo test --all

## Run tests for a specific contract (usage: make test-one CRATE=ajo-pool)
test-one:
	cargo test -p $(CRATE) -- --nocapture

## Run tests in release mode
test-release:
	cargo test --all --release

# ─── Lint & Format ───────────────────────────────────────────────────────

## Format all source files
fmt:
	cargo fmt --all

## Check formatting (CI mode)
fmt-check:
	cargo fmt --all -- --check

## Run clippy lints
lint:
	cargo clippy --all-targets -- -D warnings

## Run all checks (format + lint + WASM check)
check: fmt-check lint
	cargo check --target wasm32-unknown-unknown

# ─── Security ────────────────────────────────────────────────────────────

## Run dependency security audit
audit:
	cargo audit

# ─── Clean ───────────────────────────────────────────────────────────────

## Remove build artifacts
clean:
	cargo clean

# ─── Help ────────────────────────────────────────────────────────────────

## Show available commands
help:
	@echo "AjoChain Development Commands:"
	@echo ""
	@echo "  make build          Build all contracts (WASM)"
	@echo "  make build-stellar  Build using Stellar CLI"
	@echo "  make test           Run all tests"
	@echo "  make test-one       Run tests for one contract (CRATE=ajo-pool)"
	@echo "  make test-release   Run tests in release mode"
	@echo "  make fmt            Format source files"
	@echo "  make fmt-check      Check formatting (CI)"
	@echo "  make lint           Run clippy lints"
	@echo "  make check          Run all checks"
	@echo "  make audit          Security audit"
	@echo "  make clean          Remove build artifacts"
