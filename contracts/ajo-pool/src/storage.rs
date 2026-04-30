//! AjoChain Pool — Storage keys and data types
//!
//! Defines all persistent and instance storage keys, plus the core data
//! structures that model a ROSCA savings pool.

use soroban_sdk::{contracttype, Address, Vec};

// ─── Storage Keys ───────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Global pool counter (Instance storage).
    PoolCounter,
    /// Pool configuration — keyed by pool_id (Persistent storage).
    Pool(u64),
    /// Member list for a pool (Persistent storage).
    Members(u64),
    /// Per-round contribution tracker — keyed by (pool_id, round) (Temporary storage).
    Contributions(u64, u32),
    /// Tracks which members have been paid out — keyed by pool_id (Persistent storage).
    PayoutTracker(u64),
    /// Current round index for a pool (Persistent storage).
    CurrentRound(u64),
    /// Auction bids for a round — keyed by (pool_id, round) (Temporary storage).
    AuctionBids(u64, u32),
    /// Admin address (Instance storage).
    Admin,
}

// ─── Pool Configuration ─────────────────────────────────────────────────────

/// The payout ordering strategy for the savings circle.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PayoutMode {
    /// Members receive payouts in the order they joined.
    FixedRotation = 0,
    /// Payout order is determined by deterministic pseudo-random selection.
    RandomRotation = 1,
    /// Members bid for earlier payouts; highest bidder pays a premium to the pool.
    Auction = 2,
}

/// The lifecycle state of a savings pool.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PoolState {
    /// Pool is open for new members to join.
    Recruiting = 0,
    /// The savings cycle is actively running.
    Active = 1,
    /// All rounds have been completed.
    Completed = 2,
    /// The pool was cancelled before starting.
    Cancelled = 3,
}

/// Core configuration and runtime state of a single ROSCA pool.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PoolConfig {
    /// Unique pool identifier.
    pub pool_id: u64,
    /// The administrator who created this pool.
    pub admin: Address,
    /// The Stellar asset token used for contributions.
    pub token: Address,
    /// The fixed contribution amount per member per round (in stroops).
    pub contribution_amount: i128,
    /// Contribution frequency in ledger-seconds (e.g. 604_800 for weekly).
    pub frequency: u64,
    /// Maximum number of members allowed.
    pub max_members: u32,
    /// Current number of members.
    pub member_count: u32,
    /// The payout ordering strategy.
    pub payout_mode: PayoutMode,
    /// Current lifecycle state.
    pub state: PoolState,
    /// Total number of rounds (equals max_members for standard ROSCA).
    pub total_rounds: u32,
    /// Index of the current round (0-based).
    pub current_round: u32,
    /// Ledger timestamp when the current round started.
    pub round_start_time: u64,
    /// The protocol fee in basis points (e.g. 50 = 0.5%).
    pub fee_bps: u32,
    /// Minimum members required to start the cycle.
    pub min_members: u32,
    /// Ledger timestamp when the pool was created.
    pub created_at: u64,
    /// Whether this pool is currently paused (emergency stop).
    pub is_paused: bool,
}

/// Tracks a single member's participation in a pool.
#[contracttype]
#[derive(Clone, Debug)]
pub struct MemberRecord {
    /// The member's Stellar address.
    pub address: Address,
    /// Index in the pool's member list (0-based).
    pub index: u32,
    /// Total contributions made across all rounds.
    pub total_contributed: i128,
    /// Whether this member has received their payout.
    pub has_received_payout: bool,
    /// Number of rounds where contribution was on time.
    pub on_time_count: u32,
    /// Number of rounds where contribution was late or missed.
    pub late_count: u32,
    /// Ledger timestamp when the member joined.
    pub joined_at: u64,
}

/// A bid placed in an auction-mode pool.
#[contracttype]
#[derive(Clone, Debug)]
pub struct AuctionBid {
    /// The bidding member's address.
    pub bidder: Address,
    /// The premium amount the bidder is willing to pay.
    pub bid_amount: i128,
}

/// Contribution record for a specific round.
#[contracttype]
#[derive(Clone, Debug)]
pub struct RoundContributions {
    /// Addresses that have contributed this round.
    pub contributors: Vec<Address>,
    /// Total amount collected this round.
    pub total_collected: i128,
}
