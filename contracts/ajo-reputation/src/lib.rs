//! # AjoChain Reputation Oracle Contract
//!
//! Maintains on-chain trust scores for ROSCA participants. Scores are updated
//! after each cycle completion based on:
//! - Completion rate (did they finish the full cycle?)
//! - Timeliness (were contributions on time?)
//! - Dispute history
//!
//! Reputation scores gate access to pool tiers:
//! - Bronze (0-249): Basic pools only
//! - Silver (250-499): Standard pools
//! - Gold (500-749): Premium pools
//! - Diamond (750-1000): Elite pools

#![no_std]

mod errors;
mod events;
mod scoring;

use errors::ReputationError;
use scoring::{DataKey, ReputationProfile, ReputationTier};

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct AjoReputationContract;

#[contractimpl]
impl AjoReputationContract {
    /// Initialise the reputation oracle. Admin only.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ReputationError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ReputationError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);

        events::oracle_initialized(&env, &admin);

        Ok(())
    }

    /// Record a successful cycle completion for a member.
    /// Called by the pool contract or admin after cycle finishes.
    ///
    /// # Arguments
    /// * `caller` — Must be admin or an authorised pool contract.
    /// * `member` — The member whose reputation is being updated.
    /// * `on_time_contributions` — Number of on-time contributions.
    /// * `late_contributions` — Number of late contributions.
    /// * `total_rounds` — Total rounds in the cycle.
    /// * `completed` — Whether the member completed the full cycle.
    pub fn record_cycle(
        env: Env,
        caller: Address,
        member: Address,
        on_time_contributions: u32,
        late_contributions: u32,
        total_rounds: u32,
        completed: bool,
    ) -> Result<(), ReputationError> {
        caller.require_auth();

        // Verify caller is admin.
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ReputationError::NotInitialized)?;

        if caller != admin {
            return Err(ReputationError::NotAdmin);
        }

        // Load or create profile.
        let mut profile: ReputationProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(member.clone()))
            .unwrap_or(ReputationProfile {
                member: member.clone(),
                score: 500, // Starting score (Silver tier).
                total_cycles: 0,
                completed_cycles: 0,
                total_on_time: 0,
                total_late: 0,
                total_disputes_raised: 0,
                total_disputes_against: 0,
                last_updated: 0,
            });

        profile.total_cycles += 1;
        if completed {
            profile.completed_cycles += 1;
        }
        profile.total_on_time += on_time_contributions;
        profile.total_late += late_contributions;

        // Recalculate score.
        profile.score = scoring::calculate_score(&profile, total_rounds, completed);
        profile.last_updated = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&DataKey::Profile(member.clone()), &profile);

        events::reputation_updated(&env, &member, profile.score);

        Ok(())
    }

    /// Record a dispute event against a member.
    pub fn record_dispute(
        env: Env,
        admin: Address,
        member: Address,
        is_against: bool,
    ) -> Result<(), ReputationError> {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ReputationError::NotInitialized)?;

        if admin != stored_admin {
            return Err(ReputationError::NotAdmin);
        }

        let mut profile: ReputationProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(member.clone()))
            .ok_or(ReputationError::ProfileNotFound)?;

        if is_against {
            profile.total_disputes_against += 1;
            // Penalty: -25 points per dispute against.
            profile.score = profile.score.saturating_sub(25);
        } else {
            profile.total_disputes_raised += 1;
        }

        profile.last_updated = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Profile(member.clone()), &profile);

        events::reputation_updated(&env, &member, profile.score);

        Ok(())
    }

    // ─── Read-Only Queries ──────────────────────────────────────────────

    /// Get a member's reputation profile.
    pub fn get_profile(env: Env, member: Address) -> Result<ReputationProfile, ReputationError> {
        env.storage()
            .persistent()
            .get(&DataKey::Profile(member))
            .ok_or(ReputationError::ProfileNotFound)
    }

    /// Get a member's current tier.
    pub fn get_tier(env: Env, member: Address) -> Result<ReputationTier, ReputationError> {
        let profile: ReputationProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(member))
            .ok_or(ReputationError::ProfileNotFound)?;

        Ok(scoring::score_to_tier(profile.score))
    }

    /// Check if a member meets the minimum tier requirement.
    pub fn meets_tier(
        env: Env,
        member: Address,
        required_tier: u32,
    ) -> bool {
        let profile: Option<ReputationProfile> = env
            .storage()
            .persistent()
            .get(&DataKey::Profile(member));

        match profile {
            Some(p) => {
                let tier = scoring::score_to_tier(p.score);
                (tier as u32) >= required_tier
            }
            None => required_tier == 0, // New users only qualify for Bronze.
        }
    }
}

#[cfg(test)]
mod test;
