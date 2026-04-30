//! AjoChain Pool — Payout logic
//!
//! Implements three payout modes:
//! 1. **Fixed Rotation** — members receive payouts in join order.
//! 2. **Random Rotation** — deterministic pseudo-random selection using ledger entropy.
//! 3. **Auction** — members bid for priority; highest bidder receives the pot.

use crate::errors::PoolError;
use crate::events;
use crate::storage::{
    AuctionBid, DataKey, MemberRecord, PayoutMode, PoolConfig, PoolState, RoundContributions,
};
use soroban_sdk::{token, Address, Env, Vec};

/// Record a member's contribution for the current round.
pub fn contribute(env: &Env, pool_id: u64, member: &Address) -> Result<(), PoolError> {
    member.require_auth();

    let config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Active {
        return Err(PoolError::InvalidState);
    }

    let round = config.current_round;

    // Load or create the round's contribution record.
    let mut contribs: RoundContributions = env
        .storage()
        .temporary()
        .get(&DataKey::Contributions(pool_id, round))
        .unwrap_or(RoundContributions {
            contributors: Vec::new(env),
            total_collected: 0,
        });

    // Check for duplicate contribution.
    for i in 0..contribs.contributors.len() {
        if contribs.contributors.get(i).unwrap() == *member {
            return Err(PoolError::AlreadyContributed);
        }
    }

    // Verify membership.
    let members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .ok_or(PoolError::NotMember)?;

    let mut is_member = false;
    for i in 0..members.len() {
        if members.get(i).unwrap().address == *member {
            is_member = true;
            break;
        }
    }
    if !is_member {
        return Err(PoolError::NotMember);
    }

    // Transfer the contribution from the member to this contract.
    let contract_address = env.current_contract_address();
    let token_client = token::Client::new(env, &config.token);
    token_client.transfer(member, &contract_address, &config.contribution_amount);

    // Update the contribution record.
    contribs.contributors.push_back(member.clone());
    contribs.total_collected = contribs
        .total_collected
        .checked_add(config.contribution_amount)
        .ok_or(PoolError::Overflow)?;

    env.storage()
        .temporary()
        .set(&DataKey::Contributions(pool_id, round), &contribs);

    // Update member stats.
    let mut members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .unwrap();
    for i in 0..members.len() {
        let mut rec = members.get(i).unwrap();
        if rec.address == *member {
            rec.total_contributed = rec
                .total_contributed
                .checked_add(config.contribution_amount)
                .ok_or(PoolError::Overflow)?;

            // Check timeliness.
            let deadline = config.round_start_time + config.frequency;
            if env.ledger().timestamp() <= deadline {
                rec.on_time_count += 1;
            } else {
                rec.late_count += 1;
            }

            members.set(i, rec);
            break;
        }
    }
    env.storage()
        .persistent()
        .set(&DataKey::Members(pool_id), &members);

    events::contribution_made(env, pool_id, member, round, config.contribution_amount);

    Ok(())
}

/// Distribute the pot for the current round to the designated recipient.
/// The recipient is determined by the pool's payout mode.
pub fn distribute_payout(env: &Env, pool_id: u64) -> Result<Address, PoolError> {
    let config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Active {
        return Err(PoolError::InvalidState);
    }

    let round = config.current_round;

    // Load contributions for this round.
    let contribs: RoundContributions = env
        .storage()
        .temporary()
        .get(&DataKey::Contributions(pool_id, round))
        .ok_or(PoolError::RoundIncomplete)?;

    // All members must have contributed (or deadline must be passed).
    let all_contributed = contribs.contributors.len() == config.member_count;
    let deadline_passed = env.ledger().timestamp() >= config.round_start_time + config.frequency;

    if !all_contributed && !deadline_passed {
        return Err(PoolError::RoundIncomplete);
    }

    // Determine recipient based on payout mode.
    let members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .ok_or(PoolError::InvalidState)?;

    let payout_tracker: Vec<Address> = env
        .storage()
        .persistent()
        .get(&DataKey::PayoutTracker(pool_id))
        .unwrap_or(Vec::new(env));

    let recipient = match config.payout_mode {
        PayoutMode::FixedRotation => {
            select_fixed_rotation(env, &members, &payout_tracker)?
        }
        PayoutMode::RandomRotation => {
            select_random_rotation(env, &members, &payout_tracker, pool_id, round)?
        }
        PayoutMode::Auction => {
            select_auction_winner(env, pool_id, round, &payout_tracker)?
        }
    };

    // Calculate payout amount (total collected minus protocol fee).
    let fee = contribs
        .total_collected
        .checked_mul(config.fee_bps as i128)
        .ok_or(PoolError::Overflow)?
        / 10_000;
    let payout_amount = contribs
        .total_collected
        .checked_sub(fee)
        .ok_or(PoolError::Overflow)?;

    // Transfer the pot to the recipient.
    let token_client = token::Client::new(env, &config.token);
    token_client.transfer(
        &env.current_contract_address(),
        &recipient,
        &payout_amount,
    );

    // Mark recipient as paid.
    let mut payout_tracker = payout_tracker;
    payout_tracker.push_back(recipient.clone());
    env.storage()
        .persistent()
        .set(&DataKey::PayoutTracker(pool_id), &payout_tracker);

    // Update member record.
    let mut members: Vec<MemberRecord> = env
        .storage()
        .persistent()
        .get(&DataKey::Members(pool_id))
        .unwrap();
    for i in 0..members.len() {
        let mut rec = members.get(i).unwrap();
        if rec.address == recipient {
            rec.has_received_payout = true;
            members.set(i, rec);
            break;
        }
    }
    env.storage()
        .persistent()
        .set(&DataKey::Members(pool_id), &members);

    events::payout_distributed(env, pool_id, round, &recipient, payout_amount);

    Ok(recipient)
}

/// Place a bid in an auction-mode pool.
pub fn place_bid(
    env: &Env,
    pool_id: u64,
    bidder: &Address,
    bid_amount: i128,
) -> Result<(), PoolError> {
    bidder.require_auth();

    let config: PoolConfig = env
        .storage()
        .persistent()
        .get(&DataKey::Pool(pool_id))
        .ok_or(PoolError::InvalidState)?;

    if config.state != PoolState::Active {
        return Err(PoolError::InvalidState);
    }
    if config.payout_mode != PayoutMode::Auction {
        return Err(PoolError::InvalidPayoutMode);
    }
    if bid_amount <= 0 {
        return Err(PoolError::BidTooLow);
    }

    let round = config.current_round;

    // Check that the bidder hasn't already been paid out.
    let payout_tracker: Vec<Address> = env
        .storage()
        .persistent()
        .get(&DataKey::PayoutTracker(pool_id))
        .unwrap_or(Vec::new(env));

    for i in 0..payout_tracker.len() {
        if payout_tracker.get(i).unwrap() == *bidder {
            return Err(PoolError::AlreadyPaidOut);
        }
    }

    // Store the bid.
    let mut bids: Vec<AuctionBid> = env
        .storage()
        .temporary()
        .get(&DataKey::AuctionBids(pool_id, round))
        .unwrap_or(Vec::new(env));

    bids.push_back(AuctionBid {
        bidder: bidder.clone(),
        bid_amount,
    });

    env.storage()
        .temporary()
        .set(&DataKey::AuctionBids(pool_id, round), &bids);

    events::auction_bid_placed(env, pool_id, round, bidder, bid_amount);

    Ok(())
}

// ─── Internal Selection Helpers ─────────────────────────────────────────────

/// Fixed rotation: select the first member who hasn't received a payout yet,
/// in the order they joined.
fn select_fixed_rotation(
    _env: &Env,
    members: &Vec<MemberRecord>,
    payout_tracker: &Vec<Address>,
) -> Result<Address, PoolError> {
    for i in 0..members.len() {
        let member = members.get(i).unwrap();
        let mut already_paid = false;
        for j in 0..payout_tracker.len() {
            if payout_tracker.get(j).unwrap() == member.address {
                already_paid = true;
                break;
            }
        }
        if !already_paid {
            return Ok(member.address);
        }
    }
    Err(PoolError::NoEligibleRecipient)
}

/// Random rotation: deterministic pseudo-random selection using ledger sequence
/// and pool/round identifiers as entropy.
fn select_random_rotation(
    env: &Env,
    members: &Vec<MemberRecord>,
    payout_tracker: &Vec<Address>,
    pool_id: u64,
    round: u32,
) -> Result<Address, PoolError> {
    // Build a list of eligible members.
    let mut eligible: Vec<Address> = Vec::new(env);
    for i in 0..members.len() {
        let member = members.get(i).unwrap();
        let mut already_paid = false;
        for j in 0..payout_tracker.len() {
            if payout_tracker.get(j).unwrap() == member.address {
                already_paid = true;
                break;
            }
        }
        if !already_paid {
            eligible.push_back(member.address);
        }
    }

    if eligible.is_empty() {
        return Err(PoolError::NoEligibleRecipient);
    }

    // Deterministic pseudo-random: hash(ledger_sequence + pool_id + round).
    let seed = env
        .ledger()
        .sequence()
        .wrapping_add(pool_id as u32)
        .wrapping_add(round);
    let index = seed % eligible.len();

    Ok(eligible.get(index).unwrap())
}

/// Auction: select the highest bidder who hasn't already been paid.
fn select_auction_winner(
    env: &Env,
    pool_id: u64,
    round: u32,
    payout_tracker: &Vec<Address>,
) -> Result<Address, PoolError> {
    let bids: Vec<AuctionBid> = env
        .storage()
        .temporary()
        .get(&DataKey::AuctionBids(pool_id, round))
        .ok_or(PoolError::NoEligibleRecipient)?;

    let mut highest_bid: i128 = -1;
    let mut winner: Option<Address> = None;

    for i in 0..bids.len() {
        let bid = bids.get(i).unwrap();

        // Skip already-paid members.
        let mut already_paid = false;
        for j in 0..payout_tracker.len() {
            if payout_tracker.get(j).unwrap() == bid.bidder {
                already_paid = true;
                break;
            }
        }

        if !already_paid && bid.bid_amount > highest_bid {
            highest_bid = bid.bid_amount;
            winner = Some(bid.bidder);
        }
    }

    winner.ok_or(PoolError::NoEligibleRecipient)
}
