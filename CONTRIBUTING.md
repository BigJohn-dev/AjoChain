# Contributing to AjoChain

Thank you for your interest in contributing to AjoChain! This guide will help you get started.

## Development Setup

### Prerequisites

- **Rust** (latest stable via [rustup](https://rustup.rs/))
- **WASM target**: `rustup target add wasm32-unknown-unknown`
- **Stellar CLI** (optional): `cargo install --locked stellar-cli`

### Building

```bash
# Clone the repository
git clone https://github.com/BigJohn-dev/AjoChain.git
cd AjoChain

# Build all contracts
make build

# Run all tests
make test
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Follow the existing code style and module patterns
- Add unit tests for any new functionality
- Update relevant documentation in `docs/`

### 3. Verify

```bash
# Run all checks
make check

# Run tests
make test

# Format code
make fmt
```

### 4. Submit

```bash
git commit -m "feat: add amazing feature"
git push origin feature/your-feature-name
```

Then open a Pull Request against the `develop` branch.

## Code Standards

### Rust Style

- Use `#![no_std]` on all contract crates
- Use `panic_with_error!` instead of `panic!`
- All arithmetic must use checked operations (`checked_add`, etc.)
- All public functions must call `ttl::extend_instance_ttl(&env)`
- All persistent storage writes must be followed by `ttl::extend_persistent_ttl`
- Error codes must be unique within their range (see `docs/architecture.md`)

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — New features
- `fix:` — Bug fixes
- `docs:` — Documentation changes
- `test:` — Test additions/changes
- `refactor:` — Code refactoring
- `chore:` — Maintenance tasks

### Testing

- Every public contract function must have at least one test
- Tests should cover both success and failure paths
- Use `env.mock_all_auths()` for test environments
- Test helper functions should be in the same test module

### Security Requirements

- Never use unbounded loops — always use `max_members` or pagination limits
- Always validate inputs against protocol constants from `ajo_types::constants`
- All state-mutating functions must call `require_auth()`
- Storage keys must use the correct storage type (Instance/Persistent/Temporary)

## Project Structure

Each contract follows this module pattern:

```
contracts/<name>/src/
├── lib.rs       # Contract entry-point with #[contractimpl]
├── storage.rs   # Data types and storage keys
├── errors.rs    # Error enum with unique u32 codes
├── events.rs    # Event emission helpers
├── test.rs      # Unit tests
└── <modules>.rs # Additional business logic modules
```

## Questions?

Open an issue or reach out to the maintainers. We're happy to help!
