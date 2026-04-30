//! AjoChain Pool — Event emissions
//!
//! All contract events are emitted through these helpers to ensure consistent
//! topic structure for off-chain indexers and the frontend.

use soroban_sdk::{Address, Env, Symbol};

/// Emitted when a new pool is created.
pub fn pool_created(env: &Env, pool_id: u64, admin: &Address, token: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "pool_created"), pool_id),
        (admin.clone(), token.clone(), amount),
    );
}

/// Emitted when a member joins the pool.
pub fn member_joined(env: &Env, pool_id: u64, member: &Address, member_index: u32) {
    env.events().publish(
        (Symbol::new(env, "member_joined"), pool_id),
        (member.clone(), member_index),
    );
}

/// Emitted when a member leaves the pool.
pub fn member_left(env: &Env, pool_id: u64, member: &Address) {
    env.events().publish(
        (Symbol::new(env, "member_left"), pool_id),
        member.clone(),
    );
}

/// Emitted when a contribution is made.
pub fn contribution_made(
    env: &Env,
    pool_id: u64,
    member: &Address,
    round: u32,
    amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "contribution"), pool_id, round),
        (member.clone(), amount),
    );
}

/// Emitted when a round's payout is distributed.
pub fn payout_distributed(
    env: &Env,
    pool_id: u64,
    round: u32,
    recipient: &Address,
    amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "payout"), pool_id, round),
        (recipient.clone(), amount),
    );
}

/// Emitted when the cycle starts.
pub fn cycle_started(env: &Env, pool_id: u64, total_rounds: u32) {
    env.events().publish(
        (Symbol::new(env, "cycle_started"), pool_id),
        total_rounds,
    );
}

/// Emitted when the full cycle completes.
pub fn cycle_completed(env: &Env, pool_id: u64) {
    env.events().publish(
        (Symbol::new(env, "cycle_completed"), pool_id),
        true,
    );
}

/// Emitted when a new round begins.
pub fn round_started(env: &Env, pool_id: u64, round: u32, deadline: u64) {
    env.events().publish(
        (Symbol::new(env, "round_started"), pool_id, round),
        deadline,
    );
}

/// Emitted when an auction bid is placed.
pub fn auction_bid_placed(
    env: &Env,
    pool_id: u64,
    round: u32,
    bidder: &Address,
    bid_amount: i128,
) {
    env.events().publish(
        (Symbol::new(env, "auction_bid"), pool_id, round),
        (bidder.clone(), bid_amount),
    );
}

/// Emitted when a pool is paused.
pub fn pool_paused(env: &Env, pool_id: u64, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "pool_paused"), pool_id),
        admin.clone(),
    );
}

/// Emitted when a pool is unpaused.
pub fn pool_unpaused(env: &Env, pool_id: u64, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "pool_unpaused"), pool_id),
        admin.clone(),
    );
}

/// Emitted when a pool admin is transferred.
pub fn admin_transferred(env: &Env, pool_id: u64, old_admin: &Address, new_admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "admin_transfer"), pool_id),
        (old_admin.clone(), new_admin.clone()),
    );
}
