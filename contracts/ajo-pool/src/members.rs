//! AjoChain Pool — Member management
//!
//! Handles member registration, removal, and lookups within a pool.

use crate::errors::PoolError;
use crate::events;
use crate::storage::{DataKey, MemberRecord, PoolConfig, PoolState};
use ajo_types::ttl;
use soroban_sdk::{Address, Env, Vec};

/// Register a new member in the pool. Must be in Recruiting state.
pub fn join(env: &Env, pool_id: u64, member: &Address) -> Result<u32, PoolError> {
    member.require_auth();

    let mut config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Recruiting {
        return Err(PoolError::InvalidState);
    }
    if config.member_count >= config.max_members {
        return Err(PoolError::PoolFull);
    }

    // Load existing members.
    let mut members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .unwrap_or(Vec::new(env));

    // Check for duplicate membership.
    for i in 0..members.len() {
        let existing = members.get(i).unwrap();
        if existing.address == *member {
            return Err(PoolError::AlreadyMember);
        }
    }

    let index = config.member_count;
    let record = MemberRecord {
        address: member.clone(),
        index,
        total_contributed: 0,
        has_received_payout: false,
        on_time_count: 0,
        late_count: 0,
        joined_at: env.ledger().timestamp(),
    };

    members.push_back(record);
    config.member_count = config
        .member_count
        .checked_add(1)
        .ok_or(PoolError::Overflow)?;

    env.storage()
        .persistent()
        .set(&DataKey::Members(pool_id), &members);
    ttl::extend_persistent_ttl(env, &DataKey::Members(pool_id));
    env.storage()
        .persistent()
        .set(&DataKey::Pool(pool_id), &config);
    ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

    events::member_joined(env, pool_id, member, index);

    Ok(index)
}

/// Remove a member from the pool. Only valid during Recruiting or if the member
/// has not yet received a payout. Members who already received a payout cannot
/// leave without settling their remaining obligations.
pub fn leave(env: &Env, pool_id: u64, member: &Address) -> Result<(), PoolError> {
    member.require_auth();

    let config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    let mut members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .ok_or(PoolError::NotMember)?;

    let mut found_index: Option<u32> = None;
    for i in 0..members.len() {
        let rec = members.get(i).unwrap();
        if rec.address == *member {
            if config.state == PoolState::Active && rec.has_received_payout {
                return Err(PoolError::CannotLeaveAfterPayout);
            }
            found_index = Some(i);
            break;
        }
    }

    let idx = found_index.ok_or(PoolError::NotMember)?;
    members.remove(idx);

    // Update pool config.
    let mut config = config;
    config.member_count = config.member_count.saturating_sub(1);

    env.storage()
        .persistent()
        .set(&DataKey::Members(pool_id), &members);
    ttl::extend_persistent_ttl(env, &DataKey::Members(pool_id));
    env.storage()
        .persistent()
        .set(&DataKey::Pool(pool_id), &config);
    ttl::extend_persistent_ttl(env, &DataKey::Pool(pool_id));

    events::member_left(env, pool_id, member);

    Ok(())
}

/// Look up a member's record by address. Returns None if not found.
pub fn get_member(
    env: &Env,
    pool_id: u64,
    member: &Address,
) -> Result<MemberRecord, PoolError> {
    let members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .ok_or(PoolError::NotMember)?;

    for i in 0..members.len() {
        let rec = members.get(i).unwrap();
        if rec.address == *member {
            return Ok(rec);
        }
    }

    Err(PoolError::NotMember)
}

/// Get all members of a pool.
pub fn get_all_members(env: &Env, pool_id: u64) -> Result<Vec<MemberRecord>, PoolError> {
    env.storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .ok_or(PoolError::InvalidState)
}
