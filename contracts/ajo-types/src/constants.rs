//! AjoChain — Protocol-wide constants
//!
//! Centralised constants shared across all contracts. Changing a value here
//! propagates to every contract at compile time.

/// Maximum number of members a single pool can hold.
/// Bounded to prevent gas exhaustion during iteration.
pub const MAX_POOL_MEMBERS: u32 = 50;

/// Maximum number of allowed tokens in the factory allowlist.
pub const MAX_ALLOWED_TOKENS: u32 = 20;

/// Maximum number of council members in governance.
pub const MAX_COUNCIL_MEMBERS: u32 = 9;

/// Maximum protocol fee in basis points (5% cap).
pub const MAX_FEE_BPS: u32 = 500;

/// Minimum contribution frequency in seconds (1 hour).
pub const MIN_FREQUENCY_SECS: u64 = 3_600;

/// Maximum contribution frequency in seconds (90 days).
pub const MAX_FREQUENCY_SECS: u64 = 7_776_000;

/// Minimum collateral ratio in basis points (100% = 10_000).
pub const MIN_COLLATERAL_RATIO_BPS: u32 = 10_000;

/// Maximum collateral ratio in basis points (300% = 30_000).
pub const MAX_COLLATERAL_RATIO_BPS: u32 = 30_000;

/// Default governance timelock delay (48 hours in seconds).
pub const DEFAULT_TIMELOCK_DELAY_SECS: u64 = 172_800;

/// Maximum reputation score.
pub const MAX_REPUTATION_SCORE: u32 = 1_000;

/// Starting reputation score for new members (Silver tier).
pub const STARTING_REPUTATION_SCORE: u32 = 500;

/// Reputation penalty per dispute against a member.
pub const DISPUTE_PENALTY_POINTS: u32 = 25;

/// Maximum pagination limit for list queries.
pub const MAX_PAGINATION_LIMIT: u32 = 100;

/// Hard cap on auction bids per round to prevent storage exhaustion.
pub const MAX_AUCTION_BIDS_PER_ROUND: u32 = 50;
