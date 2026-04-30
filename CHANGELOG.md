# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Smart Contracts
- **ajo-pool**: Core ROSCA lifecycle contract with 3 payout modes (Fixed Rotation, Random Rotation, Auction)
- **ajo-factory**: Pool deployer and global registry with token allowlisting and pagination
- **ajo-collateral**: Collateral vault with deposit, slash, and release mechanics
- **ajo-reputation**: On-chain trust scoring oracle with tiered pool access (Bronze → Diamond)
- **ajo-governance**: Protocol governance with proposal lifecycle, 48-hour timelock, council veto, and emergency circuit breaker
- **ajo-types**: Shared types crate with protocol constants, TTL management, and versioning

#### Production Hardening
- TTL (Time-To-Live) management on all storage entries to prevent data archival
- Contract version tracking with upgrade/migrate pattern
- Pool pause/unpause for emergency stops
- Admin transfer capability
- Protocol constant bounds validation (max members, fee caps, frequency limits)
- Bounded iteration safety caps from shared constants

#### Testing
- Comprehensive unit test suite for all 5 contracts (30+ tests)
- Full ROSCA lifecycle integration test (create → join → contribute → payout → complete)
- Collateral deposit/slash/release tests with balance verification
- Reputation scoring and tier progression tests
- Governance proposal lifecycle, veto, and timelock tests

#### CI/CD
- GitHub Actions pipeline: format check, clippy lint, WASM build, test suite, security audit
- WASM artifact upload for release builds

#### Documentation
- Architecture overview with system diagrams
- Complete event specification for all contracts
- Security threat model with audit checklist
- Deployment guide for testnet and mainnet
- Contributing guide with code standards
- Code of Conduct

#### Tooling
- Cargo workspace with WASM-optimized release profile
- Makefile with build, test, lint, format, audit, and help commands
- Rust toolchain pinning for reproducible builds
- Shell scripts for build, test, deploy, and TypeScript binding generation
