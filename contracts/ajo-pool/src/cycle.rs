//! AjoChain Pool — Cycle state machine
//!
//! Manages pool lifecycle transitions: Recruiting → Active → rounds → Completed.
//! All state transitions are validated and emit events for indexer consumption.

use crate::errors::PoolError;
use crate::events;
use crate::storage::{DataKey, PoolConfig, PoolState, RoundContributions};
use ajo_types::ttl;
use soroban_sdk::{Env, Vec};

/// Start the savings cycle. Transitions from Recruiting → Active.
/// Requires that the minimum member count has been met.
pub fn start_cycle(env: &Env, pool_id: u64) -> Result<(), PoolError> {
    let mut config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Recruiting {
        return Err(PoolError::InvalidState);
    }
    if config.member_count < config.min_members {
        return Err(PoolError::InsufficientMembers);
    }

    // Total rounds = number of members (each member gets exactly one payout).
    config.total_rounds = config.member_count;
    config.current_round = 0;
    config.state = PoolState::Active;
    config.round_start_time = env.ledger().timestamp();

    env.storage()
        .persistent()
        .set(&DataKey::Pool(pool_id), &config);
    ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

    // Initialise the first round's contribution tracker.
    let empty_contribs = RoundContributions {
        contributors: Vec::new(env),
        total_collected: 0,
    };
    env.storage()
        .temporary()
        .set(&DataKey::Contributions(pool_id, 0), &empty_contribs);

    events::cycle_started(env, pool_id, config.total_rounds);
    events::round_started(env, pool_id, 0, config.round_start_time + config.frequency);

    Ok(())
}

/// Advance to the next round after a successful payout.
/// Returns `true` if the cycle has completed (no more rounds).
pub fn advance_round(env: &Env, pool_id: u64) -> Result<bool, PoolError> {
    let mut config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Active {
        return Err(PoolError::InvalidState);
    }

    let next_round = config
        .current_round
        .checked_add(1)
        .ok_or(PoolError::Overflow)?;

    if next_round >= config.total_rounds {
        // Cycle is complete.
        config.state = PoolState::Completed;
        config.current_round = next_round;
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &config);
        ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

        events::cycle_completed(env, pool_id);
        return Ok(true);
    }

    // Start the next round.
    config.current_round = next_round;
    config.round_start_time = env.ledger().timestamp();
    env.storage()
        .persistent()
        .set(&DataKey::Pool(pool_id), &config);
    ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

    // Initialise contribution tracker for the new round.
    let empty_contribs = RoundContributions {
        contributors: Vec::new(env),
        total_collected: 0,
    };
    env.storage()
        .temporary()
        .set(&DataKey::Contributions(pool_id, next_round), &empty_contribs);

    let deadline = config.round_start_time + config.frequency;
    events::round_started(env, pool_id, next_round, deadline);

    Ok(false)
}

/// Cancel a pool that is still in the Recruiting state.
pub fn cancel_pool(env: &Env, pool_id: u64) -> Result<(), PoolError> {
    let mut config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Recruiting {
        return Err(PoolError::InvalidState);
    }

    config.state = PoolState::Cancelled;
    env.storage()
        .persistent()
        .set(&DataKey::Pool(pool_id), &config);
    ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

    Ok(())
}

/// Check whether the current round's deadline has been exceeded.
pub fn is_round_deadline_passed(env: &Env, config: &PoolConfig) -> bool {
    let deadline = config.round_start_time + config.frequency;
    env.ledger().timestamp() >= deadline
}
