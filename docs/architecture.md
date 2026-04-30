# AjoChain — Architecture Overview

## System Architecture

AjoChain is a modular protocol composed of 5 Soroban smart contracts, a Go-based backend, and a Next.js frontend.

```
┌────────────────────────────────────────────────────────────┐
│                    Frontend (Next.js 15)                    │
│  Dashboard │ Pool Explorer │ Create Pool │ Profile          │
│               Freighter Wallet Integration                 │
└──────────────────────┬─────────────────────────────────────┘
                       │ Stellar SDK / TypeScript Bindings
┌──────────────────────┴─────────────────────────────────────┐
│                    Stellar / Soroban                        │
│                                                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐             │
│  │ Factory  │──│   Pool   │──│  Collateral   │             │
│  │ Registry │  │ Lifecycle│  │    Vault      │             │
│  └──────────┘  └────┬─────┘  └──────────────┘             │
│                     │                                      │
│           ┌─────────┴──────────┐                           │
│           │                    │                           │
│  ┌────────┴───┐   ┌───────────┴──┐                        │
│  │ Reputation │   │  Governance  │                        │
│  │   Oracle   │   │  + Timelock  │                        │
│  └────────────┘   └──────────────┘                        │
└──────────────────────┬─────────────────────────────────────┘
                       │ Event Streaming
┌──────────────────────┴─────────────────────────────────────┐
│                    Backend (Go)                            │
│  Event Indexer │ REST API │ WebSocket │ PostgreSQL          │
└────────────────────────────────────────────────────────────┘
```

## Contract Interactions

### Pool Lifecycle Flow
1. **Factory** deploys a new **Pool** instance
2. Members **join** the Pool and **deposit collateral** in the Vault
3. **Reputation Oracle** is checked for tier eligibility
4. Pool transitions through: `Recruiting → Active → Rounds → Completed`
5. On completion, **Reputation** scores are updated and **Collateral** released

### Security Architecture
- All admin actions pass through **Governance** with a 48-hour timelock
- **Security Council** can veto malicious proposals
- **Emergency Circuit Breaker** for critical situations (bypasses timelock)
- Collateral vault provides economic default protection

## Storage Strategy

| Storage Type | Purpose | TTL |
|---|---|---|
| **Instance** | Contract config, admin, counters | Extended |
| **Persistent** | Pool configs, members, reputation profiles | Extended |
| **Temporary** | Per-round contributions, auction bids | Short (1 cycle) |

## Error Code Ranges

| Contract | Range | Example |
|---|---|---|
| ajo-pool | 1-99 | `InvalidState = 3` |
| ajo-factory | 100-199 | `PoolNotFound = 104` |
| ajo-collateral | 200-299 | `NoDeposit = 204` |
| ajo-reputation | 300-399 | `ProfileNotFound = 303` |
| ajo-governance | 400-499 | `TimelockNotElapsed = 405` |
