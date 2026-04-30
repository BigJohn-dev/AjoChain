//! AjoChain Pool — Error definitions
//!
//! Centralised error codes for the ajo-pool contract.
//! Using explicit `u32` repr ensures deterministic, indexer-friendly error codes.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PoolError {
    /// The pool has already been initialised.
    AlreadyInitialized = 1,
    /// The caller is not the pool administrator.
    NotAdmin = 2,
    /// The pool is not in the expected state for this operation.
    InvalidState = 3,
    /// The member is already registered in this pool.
    AlreadyMember = 4,
    /// The member is not registered in this pool.
    NotMember = 5,
    /// The pool has reached its maximum member capacity.
    PoolFull = 6,
    /// The contribution amount does not match the required amount.
    InvalidAmount = 7,
    /// The member has already contributed this round.
    AlreadyContributed = 8,
    /// Not all members have contributed for this round.
    RoundIncomplete = 9,
    /// The payout recipient has already received their payout.
    AlreadyPaidOut = 10,
    /// The round deadline has not yet been reached.
    DeadlineNotReached = 11,
    /// No more rounds remain in this cycle.
    CycleComplete = 12,
    /// The minimum member threshold has not been met.
    InsufficientMembers = 13,
    /// The provided payout mode is invalid.
    InvalidPayoutMode = 14,
    /// The contribution frequency is invalid (must be > 0).
    InvalidFrequency = 15,
    /// The maximum member count is invalid (must be >= 2).
    InvalidMaxMembers = 16,
    /// An arithmetic overflow occurred.
    Overflow = 17,
    /// The pool cycle has not started yet.
    CycleNotStarted = 18,
    /// The member cannot leave after receiving a payout without settling.
    CannotLeaveAfterPayout = 19,
    /// The auction bid is too low.
    BidTooLow = 20,
    /// No eligible recipient found for this round.
    NoEligibleRecipient = 21,
    /// The pool is currently paused.
    Paused = 22,
    /// The requested pool was not found.
    PoolNotFound = 23,
    /// The upgrade is not needed (already at latest version).
    UpgradeNotNeeded = 24,
}
