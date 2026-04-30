# AjoChain — Contract Events Specification

This document specifies all events emitted by AjoChain contracts. These events are consumed by the backend indexer and frontend for real-time UI updates.

## Event Format

All events follow the Soroban publish format:
```
env.events().publish((topic1, topic2, ...), data)
```

## ajo-pool Events

| Event | Topics | Data | Description |
|---|---|---|---|
| `pool_created` | `("pool_created", pool_id)` | `(admin, token, amount)` | New pool created |
| `member_joined` | `("member_joined", pool_id)` | `(member, index)` | Member joined pool |
| `member_left` | `("member_left", pool_id)` | `member` | Member left pool |
| `contribution` | `("contribution", pool_id, round)` | `(member, amount)` | Contribution made |
| `payout` | `("payout", pool_id, round)` | `(recipient, amount)` | Payout distributed |
| `cycle_started` | `("cycle_started", pool_id)` | `total_rounds` | Cycle began |
| `cycle_completed` | `("cycle_completed", pool_id)` | `true` | All rounds finished |
| `round_started` | `("round_started", pool_id, round)` | `deadline` | New round began |
| `auction_bid` | `("auction_bid", pool_id, round)` | `(bidder, amount)` | Auction bid placed |

## ajo-factory Events

| Event | Topics | Data |
|---|---|---|
| `factory_init` | `("factory_init",)` | `(admin, fee_bps)` |
| `pool_registered` | `("pool_registered", pool_index)` | `(pool_address, creator)` |
| `token_allowed` | `("token_allowed",)` | `token` |
| `token_removed` | `("token_removed",)` | `token` |
| `fee_updated` | `("fee_updated",)` | `new_fee_bps` |

## ajo-collateral Events

| Event | Topics | Data |
|---|---|---|
| `vault_init` | `("vault_init",)` | `(admin, token, required)` |
| `collateral_dep` | `("collateral_dep",)` | `(member, amount)` |
| `collateral_slash` | `("collateral_slash",)` | `(member, amount)` |
| `collateral_rel` | `("collateral_rel",)` | `(member, amount)` |

## ajo-reputation Events

| Event | Topics | Data |
|---|---|---|
| `rep_init` | `("rep_init",)` | `admin` |
| `rep_updated` | `("rep_updated",)` | `(member, new_score)` |

## ajo-governance Events

| Event | Topics | Data |
|---|---|---|
| `gov_init` | `("gov_init",)` | `(admin, timelock_delay)` |
| `proposal_new` | `("proposal_new", proposal_id)` | `proposer` |
| `proposal_exec` | `("proposal_exec", proposal_id)` | `true` |
| `proposal_veto` | `("proposal_veto", proposal_id)` | `vetoed_by` |
| `emergency_pause` | `("emergency_pause",)` | `admin` |
| `emergency_unpause` | `("emergency_unpause",)` | `admin` |
