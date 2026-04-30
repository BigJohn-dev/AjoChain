//! # AjoChain Pool Contract
//!
//! The core ROSCA (Rotating Savings and Credit Association) smart contract.
//!
//! This contract manages the complete lifecycle of a savings circle:
//! - Pool creation with configurable parameters
//! - Member registration with auth-gated joining
//! - Per-round contribution tracking with token transfers
//! - Three payout modes: Fixed Rotation, Random Rotation, and Auction
//! - Automatic round advancement and cycle completion
//! - Upgradeable with version-tracked migrations
//! - TTL-managed storage for production safety
//!
//! ## Architecture
//!
//! The contract is decomposed into focused modules:
//! - [`storage`] — Data types, storage keys, and state structures
//! - [`errors`] — Deterministic error codes for indexer compatibility
//! - [`events`] — Structured event emissions for off-chain consumers
//! - [`cycle`] — Pool lifecycle state machine
//! - [`members`] — Member registration and management
//! - [`payout`] — Contribution recording and payout distribution

#![no_std]

mod cycle;
mod errors;
mod events;
mod members;
mod payout;
mod storage;

use errors::PoolError;
use storage::{DataKey, MemberRecord, PayoutMode, PoolConfig, PoolState};

use ajo_types::constants;
use ajo_types::ttl;
use ajo_types::version;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

#[contract]
pub struct AjoPoolContract;

#[contractimpl]
impl AjoPoolContract {
    // ─── Initialisation ─────────────────────────────────────────────────

    /// Create a new savings pool.
    ///
    /// # Arguments
    /// * `admin` — The pool creator and administrator.
    /// * `token` — The Stellar asset token address for contributions.
    /// * `contribution_amount` — Fixed contribution per member per round (in stroops).
    /// * `frequency` — Round duration in ledger-seconds (e.g. 604_800 for weekly).
    /// * `max_members` — Maximum number of members allowed in this pool.
    /// * `min_members` — Minimum members required to start the cycle.
    /// * `payout_mode` — The payout ordering strategy (0=Fixed, 1=Random, 2=Auction).
    /// * `fee_bps` — Protocol fee in basis points (e.g. 50 = 0.5%).
    ///
    /// # Returns
    /// The newly created pool's unique identifier.
    pub fn create_pool(
        env: Env,
        admin: Address,
        token: Address,
        contribution_amount: i128,
        frequency: u64,
        max_members: u32,
        min_members: u32,
        payout_mode: u32,
        fee_bps: u32,
    ) -> Result<u64, PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        // ── Input validation with protocol constants ──
        if max_members < 2 || max_members > constants::MAX_POOL_MEMBERS {
            return Err(PoolError::InvalidMaxMembers);
        }
        if frequency < constants::MIN_FREQUENCY_SECS || frequency > constants::MAX_FREQUENCY_SECS {
            return Err(PoolError::InvalidFrequency);
        }
        if contribution_amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }
        if min_members < 2 || min_members > max_members {
            return Err(PoolError::InvalidMaxMembers);
        }
        if fee_bps > constants::MAX_FEE_BPS {
            return Err(PoolError::InvalidAmount);
        }

        let mode = match payout_mode {
            0 => PayoutMode::FixedRotation,
            1 => PayoutMode::RandomRotation,
            2 => PayoutMode::Auction,
            _ => return Err(PoolError::InvalidPayoutMode),
        };

        // Increment the global pool counter.
        let pool_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::PoolCounter)
            .unwrap_or(0_u64);
        let next_id = pool_id.checked_add(1).ok_or(PoolError::Overflow)?;
        env.storage()
            .instance()
            .set(&DataKey::PoolCounter, &next_id);

        let config = PoolConfig {
            pool_id: next_id,
            admin: admin.clone(),
            token: token.clone(),
            contribution_amount,
            frequency,
            max_members,
            member_count: 0,
            payout_mode: mode,
            state: PoolState::Recruiting,
            total_rounds: 0,
            current_round: 0,
            round_start_time: 0,
            fee_bps,
            min_members,
            created_at: env.ledger().timestamp(),
            is_paused: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Pool(next_id), &config);
        ttl::extend_persistent_ttl(&env, &DataKey::Pool(next_id));

        // Initialise empty member list.
        let empty_members: Vec<MemberRecord> = Vec::new(&env);
        env.storage()
            .persistent()
            .set(&DataKey::Members(next_id), &empty_members);
        ttl::extend_persistent_ttl(&env, &DataKey::Members(next_id));

        // Set version on first pool creation.
        if version::get_version(&env) == 0 {
            version::set_version(&env, version::PROTOCOL_VERSION);
        }

        events::pool_created(&env, next_id, &admin, &token, contribution_amount);

        Ok(next_id)
    }

    // ─── Member Operations ──────────────────────────────────────────────

    /// Join a pool as a new member. The pool must be in the Recruiting state.
    pub fn join_pool(env: Env, pool_id: u64, member: Address) -> Result<u32, PoolError> {
        ttl::extend_instance_ttl(&env);
        members::join(&env, pool_id, &member)
    }

    /// Leave a pool. Allowed during Recruiting, or during Active if the member
    /// has not yet received their payout.
    pub fn leave_pool(env: Env, pool_id: u64, member: Address) -> Result<(), PoolError> {
        ttl::extend_instance_ttl(&env);
        members::leave(&env, pool_id, &member)
    }

    // ─── Cycle Operations ───────────────────────────────────────────────

    /// Start the savings cycle. Requires admin auth and minimum members met.
    pub fn start_cycle(env: Env, pool_id: u64, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;

        if config.admin != admin {
            return Err(PoolError::NotAdmin);
        }
        if config.is_paused {
            return Err(PoolError::Paused);
        }

        cycle::start_cycle(&env, pool_id)
    }

    /// Cancel a pool that hasn't started yet. Admin only.
    pub fn cancel_pool(env: Env, pool_id: u64, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;

        if config.admin != admin {
            return Err(PoolError::NotAdmin);
        }

        cycle::cancel_pool(&env, pool_id)
    }

    // ─── Contribution & Payout ──────────────────────────────────────────

    /// Make a contribution for the current round.
    pub fn contribute(env: Env, pool_id: u64, member: Address) -> Result<(), PoolError> {
        ttl::extend_instance_ttl(&env);
        payout::contribute(&env, pool_id, &member)
    }

    /// Place a bid in an auction-mode pool.
    pub fn place_bid(
        env: Env,
        pool_id: u64,
        bidder: Address,
        bid_amount: i128,
    ) -> Result<(), PoolError> {
        ttl::extend_instance_ttl(&env);
        payout::place_bid(&env, pool_id, &bidder, bid_amount)
    }

    /// Trigger the payout for the current round and advance to the next.
    /// Can be called by anyone once conditions are met (all contributed or deadline passed).
    ///
    /// # Returns
    /// The address of the payout recipient.
    pub fn trigger_payout(env: Env, pool_id: u64) -> Result<Address, PoolError> {
        ttl::extend_instance_ttl(&env);

        // Check pause state.
        let config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;
        if config.is_paused {
            return Err(PoolError::Paused);
        }

        let recipient = payout::distribute_payout(&env, pool_id)?;
        cycle::advance_round(&env, pool_id)?;
        Ok(recipient)
    }

    // ─── Admin Operations ───────────────────────────────────────────────

    /// Pause a pool. Prevents contributions and payouts. Admin only.
    pub fn pause_pool(env: Env, pool_id: u64, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let mut config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;

        if config.admin != admin {
            return Err(PoolError::NotAdmin);
        }

        config.is_paused = true;
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &config);
        ttl::extend_persistent_ttl(&env, &DataKey::Pool(pool_id));

        events::pool_paused(&env, pool_id, &admin);
        Ok(())
    }

    /// Unpause a pool. Admin only.
    pub fn unpause_pool(env: Env, pool_id: u64, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let mut config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;

        if config.admin != admin {
            return Err(PoolError::NotAdmin);
        }

        config.is_paused = false;
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &config);
        ttl::extend_persistent_ttl(&env, &DataKey::Pool(pool_id));

        events::pool_unpaused(&env, pool_id, &admin);
        Ok(())
    }

    /// Transfer pool admin to a new address. Current admin only.
    pub fn transfer_admin(
        env: Env,
        pool_id: u64,
        current_admin: Address,
        new_admin: Address,
    ) -> Result<(), PoolError> {
        current_admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let mut config: PoolConfig = env
            .storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)?;

        if config.admin != current_admin {
            return Err(PoolError::NotAdmin);
        }

        config.admin = new_admin.clone();
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &config);
        ttl::extend_persistent_ttl(&env, &DataKey::Pool(pool_id));

        events::admin_transferred(&env, pool_id, &current_admin, &new_admin);
        Ok(())
    }

    // ─── Upgrade ────────────────────────────────────────────────────────

    /// Upgrade the contract to a new WASM binary. Admin-only.
    /// The admin used here is a protocol-level admin stored in instance storage.
    pub fn upgrade(env: Env, admin: Address, new_wasm_hash: BytesN<32>) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(PoolError::NotAdmin)?;

        if stored_admin != admin {
            return Err(PoolError::NotAdmin);
        }

        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }

    /// Migrate state after an upgrade. Called once after `upgrade()`.
    pub fn migrate(env: Env, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        let current = version::get_version(&env);
        if current >= version::PROTOCOL_VERSION {
            return Err(PoolError::AlreadyInitialized);
        }

        // Future migrations would go here, conditioned on `current` version.
        // Example:
        // if current < 2 {
        //     // migrate from v1 → v2
        // }

        version::set_version(&env, version::PROTOCOL_VERSION);
        Ok(())
    }

    /// Set the protocol admin. Can only be called once during initial setup,
    /// or via the upgrade path.
    pub fn set_admin(env: Env, admin: Address) -> Result<(), PoolError> {
        admin.require_auth();
        ttl::extend_instance_ttl(&env);

        if env.storage().instance().has(&DataKey::Admin) {
            // If admin already exists, only current admin can change it.
            let current: Address = env
                .storage()
                .instance()
                .get(&DataKey::Admin)
                .unwrap();
            if current != admin {
                return Err(PoolError::NotAdmin);
            }
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    // ─── Read-Only Queries ──────────────────────────────────────────────

    /// Get the configuration and current state of a pool.
    pub fn get_pool(env: Env, pool_id: u64) -> Result<PoolConfig, PoolError> {
        ttl::extend_instance_ttl(&env);
        env.storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .ok_or(PoolError::InvalidState)
    }

    /// Get a specific member's record.
    pub fn get_member(
        env: Env,
        pool_id: u64,
        member: Address,
    ) -> Result<MemberRecord, PoolError> {
        ttl::extend_instance_ttl(&env);
        members::get_member(&env, pool_id, &member)
    }

    /// Get all members of a pool.
    pub fn get_members(env: Env, pool_id: u64) -> Result<Vec<MemberRecord>, PoolError> {
        ttl::extend_instance_ttl(&env);
        members::get_all_members(&env, pool_id)
    }

    /// Get the total number of pools created.
    pub fn get_pool_count(env: Env) -> u64 {
        ttl::extend_instance_ttl(&env);
        env.storage()
            .instance()
            .get(&DataKey::PoolCounter)
            .unwrap_or(0_u64)
    }

    /// Get the current contract version.
    pub fn get_version(env: Env) -> u32 {
        version::get_version(&env)
    }
}

#[cfg(test)]
mod test;
