# 🤝 AjoChain — On-Chain Cooperative Savings Protocol

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Stellar](https://img.shields.io/badge/Blockchain-Stellar-black)
![Soroban](https://img.shields.io/badge/Contracts-Soroban_v22-purple)
![Status](https://img.shields.io/badge/Status-In_Development-yellow)

**AjoChain** is a decentralized protocol digitizing the traditional African rotating savings and credit association (ROSCA) model—commonly known as *Esusu* or *Ajo*—using Stellar and Soroban smart contracts.

---

## 🌍 The Problem

Across Africa, informal cooperative savings groups empower communities to pool capital and access lump sums of cash. However, these systems rely entirely on a central human coordinator. This creates a single point of failure: coordinators can mismanage funds, abscond, or make human errors, breaking the foundational chain of trust.

## 💡 The Solution

AjoChain removes the need for a central coordinator by replacing them with a trustless Soroban smart contract.

- **🔍 Transparent:** Every cycle and contribution is fully verifiable on-chain.
- **⚡ Automated:** The smart contract auto-distributes the pot to the designated member per round.
- **🌐 Inclusive:** Built on Stellar to ensure near-zero micro-transaction fees, making it genuinely accessible to everyday users.
- **🛡️ Secure:** Collateral vault prevents rational defaults; governance timelock prevents admin abuse.
- **⭐ Merit-Based:** On-chain reputation system rewards consistent participation with access to premium pools.

---

## 🏛️ Architecture

AjoChain is a modular protocol composed of **5 Soroban smart contracts**:

```
┌──────────────────────────────────────────────────────────┐
│                  Frontend (Next.js 15)                    │
│  Dashboard │ Pool Explorer │ Create Pool │ Profile        │
└──────────────────────┬───────────────────────────────────┘
                       │
┌──────────────────────┴───────────────────────────────────┐
│                   Stellar / Soroban                       │
│                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐           │
│  │ Factory  │──│   Pool   │──│  Collateral   │           │
│  │ Registry │  │ Lifecycle│  │    Vault      │           │
│  └──────────┘  └────┬─────┘  └──────────────┘           │
│                     │                                    │
│           ┌─────────┴──────────┐                         │
│           │                    │                         │
│  ┌────────┴───┐   ┌───────────┴──┐                      │
│  │ Reputation │   │  Governance  │                      │
│  │   Oracle   │   │  + Timelock  │                      │
│  └────────────┘   └──────────────┘                      │
└──────────────────────┬───────────────────────────────────┘
                       │
┌──────────────────────┴───────────────────────────────────┐
│                   Backend (Go)                            │
│  Event Indexer │ REST API │ WebSocket │ PostgreSQL         │
└──────────────────────────────────────────────────────────┘
```

### Smart Contracts

| Contract | Purpose |
|---|---|
| **ajo-pool** | Core ROSCA lifecycle — creation, contributions, 3 payout modes, round advancement |
| **ajo-factory** | Pool deployer + global registry with token allowlisting |
| **ajo-collateral** | Collateral vault with deposit, slash, and release mechanics |
| **ajo-reputation** | On-chain trust scoring with tiered pool access (Bronze → Diamond) |
| **ajo-governance** | Proposal + Vote + 48h Timelock + Emergency Circuit Breaker |

### Payout Modes

1. **Fixed Rotation** — Members receive payouts in join order
2. **Random Rotation** — Deterministic pseudo-random selection using ledger entropy
3. **Auction** — Members bid for priority; highest bidder pays a premium to the pool

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Smart Contracts | Rust / Soroban SDK v22 |
| Frontend | Next.js 15, React, TypeScript |
| Backend | Go (Indexer + REST API) |
| Database | PostgreSQL |
| Blockchain | Stellar Network |
| Wallet | Freighter |

---

## 📁 Project Structure

```
AjoChain/
├── Cargo.toml                     # Workspace root
├── contracts/
│   ├── ajo-pool/                  # Core ROSCA lifecycle
│   │   └── src/
│   │       ├── lib.rs             # Contract entry-point
│   │       ├── cycle.rs           # State machine
│   │       ├── payout.rs          # 3-mode payout engine
│   │       ├── members.rs         # Member management
│   │       ├── errors.rs          # Error codes (1-99)
│   │       ├── events.rs          # Event emissions
│   │       ├── storage.rs         # Data types & keys
│   │       └── test.rs            # Unit tests
│   ├── ajo-factory/               # Pool deployer + registry
│   ├── ajo-collateral/            # Collateral vault
│   ├── ajo-reputation/            # Trust scoring oracle
│   └── ajo-governance/            # Governance + timelock
├── docs/
│   ├── architecture.md
│   ├── events.md
│   ├── security.md
│   └── deployment.md
├── scripts/
│   ├── build.sh
│   ├── test.sh
│   ├── deploy.sh
│   └── generate-bindings.sh
├── frontend/                      # Next.js 15 app (coming soon)
└── backend/                       # Go indexer + API (coming soon)
```

---

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli) v22+

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli
```

### Build

```bash
# Build all contracts
stellar contract build

# Run all tests
cargo test
```

### Test a Specific Contract

```bash
cargo test -p ajo-pool -- --nocapture
```

### Deploy to Testnet

See [docs/deployment.md](docs/deployment.md) for the full deployment guide.

---

## 🧱 Roadmap

### Phase 1: Smart Contract Foundation ✅
- [x] Cargo workspace with 5 modular contracts
- [x] Core ROSCA pool lifecycle (create → join → contribute → payout)
- [x] Three payout modes (Fixed, Random, Auction)
- [x] Collateral vault with slash/release mechanics
- [x] Reputation oracle with tiered scoring
- [x] Governance with timelock and emergency controls
- [x] Unit tests for core pool lifecycle

### Phase 2: Frontend & Wallet Integration 🔲
- [ ] Next.js 15 app with premium dark-mode UI
- [ ] Freighter wallet connect/disconnect
- [ ] Dashboard with portfolio overview
- [ ] Pool explorer with filters and live data
- [ ] Multi-step pool creation wizard
- [ ] Reputation profile visualization

### Phase 3: Backend & Indexer 🔲
- [ ] Go-based event indexer subscribing to contract events
- [ ] REST API with pagination
- [ ] WebSocket for real-time updates
- [ ] PostgreSQL for historical data

### Phase 4: Testnet MVP 🔲
- [ ] Full end-to-end deployment on Stellar testnet
- [ ] TypeScript binding generation
- [ ] Integration testing with Freighter

### Phase 5: Mainnet & Ecosystem 🔲
- [ ] Professional security audit
- [ ] Mainnet deployment
- [ ] Stellar Community Fund (SCF) submission
- [ ] B2B white-labeling features

---

## 🔒 Security

AjoChain is built with security-first principles:

- **`#![no_std]`** on all contracts
- **Checked arithmetic** — all math uses `checked_*` operations
- **Bounded iterations** — no unbounded loops
- **Auth-gated mutations** — `require_auth()` on all state changes
- **Storage segregation** — Instance / Persistent / Temporary
- **Collateral protection** — 150% collateralization prevents rational defaults
- **Governance timelock** — 48-hour delay on protocol changes
- **Security Council veto** — Multi-party oversight
- **Emergency circuit breaker** — Instant pause capability

See [docs/security.md](docs/security.md) for the full threat model.

---

## 🤝 Contributing

AjoChain is open-source, and community contributions are highly encouraged!

1. **Fork** the repository.
2. **Clone** your fork locally.
3. **Create** a feature branch (`git checkout -b feature/AmazingFeature`).
4. **Commit** your changes (`git commit -m 'Add some AmazingFeature'`).
5. **Push** to the branch (`git push origin feature/AmazingFeature`).
6. **Open** a Pull Request.

Please review our `Issues` tab and look for the `good first issue` tags if you want to jump in. Ensure all tests pass (`cargo test`) before submitting PRs.

---

## 📄 License

Distributed under the MIT License.