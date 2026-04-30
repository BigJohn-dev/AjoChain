# AjoChain — Deployment Guide

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli) v22+
- [wasm32-unknown-unknown target](https://doc.rust-lang.org/nightly/rustc/platform-support.html)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli
```

## Building Contracts

```bash
# Build all contracts in the workspace
stellar contract build

# Or build individually
cd contracts/ajo-pool && stellar contract build

# Run all tests
cargo test

# Run tests for a specific contract
cargo test -p ajo-pool
```

## Testnet Deployment

### 1. Configure Network

```bash
stellar network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 2. Create Deployer Identity

```bash
stellar keys generate deployer --network testnet
stellar keys address deployer
```

### 3. Fund Account

```bash
stellar keys fund deployer --network testnet
```

### 4. Deploy Contracts

```bash
# Deploy Factory
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_factory.wasm \
  --source deployer \
  --network testnet

# Deploy Pool (template for factory to use)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ajo_pool.wasm \
  --source deployer \
  --network testnet

# Deploy supporting contracts similarly...
```

### 5. Initialise Factory

```bash
stellar contract invoke \
  --id <FACTORY_CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin <DEPLOYER_ADDRESS> \
  --pool_wasm_hash <POOL_WASM_HASH> \
  --default_fee_bps 50
```

### 6. Generate TypeScript Bindings

```bash
# Generate bindings for frontend integration
stellar contract bindings typescript \
  --wasm target/wasm32-unknown-unknown/release/ajo_pool.wasm \
  --output-dir frontend/src/lib/contracts/ajo-pool \
  --contract-id <POOL_CONTRACT_ID> \
  --network testnet
```

## Mainnet Deployment

> ⚠️ Before mainnet deployment:
> 1. Complete a professional security audit
> 2. Run comprehensive fuzz tests
> 3. Verify all collateral ratios
> 4. Test governance timelock flows end-to-end
> 5. Ensure backend indexer is fully operational

Follow the same steps as testnet, replacing `testnet` with `mainnet` and using the mainnet RPC URL.
