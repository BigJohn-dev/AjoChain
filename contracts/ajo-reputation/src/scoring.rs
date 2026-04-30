//! AjoChain Reputation — Scoring engine and data types
//!
//! The scoring algorithm weights:
//! - Completion bonus: +50 for completing a full cycle
//! - Timeliness: +3 per on-time contribution, -5 per late
//! - Consistency: bonus for consecutive completions
//! - Dispute penalty: -25 per dispute against (applied directly)
//!
//! Scores are clamped to [0, 1000].

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Admin address (Instance storage).
    Admin,
    /// Per-member reputation profile (Persistent storage).
    Profile(Address),
}

/// Reputation tier levels.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ReputationTier {
    /// Score 0-249: Basic pools only.
    Bronze = 0,
    /// Score 250-499: Standard pools.
    Silver = 1,
    /// Score 500-749: Premium pools.
    Gold = 2,
    /// Score 750-1000: Elite pools.
    Diamond = 3,
}

/// A member's full reputation profile.
#[contracttype]
#[derive(Clone, Debug)]
pub struct ReputationProfile {
    /// Member address.
    pub member: Address,
    /// Current reputation score [0, 1000].
    pub score: u32,
    /// Total cycles participated in.
    pub total_cycles: u32,
    /// Total cycles completed successfully.
    pub completed_cycles: u32,
    /// Total on-time contributions across all cycles.
    pub total_on_time: u32,
    /// Total late contributions across all cycles.
    pub total_late: u32,
    /// Disputes raised by this member.
    pub total_disputes_raised: u32,
    /// Disputes raised against this member.
    pub total_disputes_against: u32,
    /// Last update timestamp.
    pub last_updated: u64,
}

/// Calculate a reputation score based on the member's profile.
///
/// Scoring formula:
/// - Base: 500 (starting score for new members)
/// - Completion bonus: +50 per completed cycle
/// - Timeliness: +3 per on-time, -5 per late
/// - Consistency multiplier: 1.1× for 3+ consecutive completions
/// - Dispute penalty: already applied directly (-25 each)
///
/// Clamped to [0, 1000].
pub fn calculate_score(
    profile: &ReputationProfile,
    _total_rounds: u32,
    completed: bool,
) -> u32 {
    let mut score: i64 = 500; // Base score.

    // Completion bonus.
    score += (profile.completed_cycles as i64) * 50;
    if completed {
        score += 50;
    }

    // Timeliness.
    score += (profile.total_on_time as i64) * 3;
    score -= (profile.total_late as i64) * 5;

    // Dispute penalties (already deducted inline, but we also factor cumulative).
    score -= (profile.total_disputes_against as i64) * 25;

    // Consistency bonus: 3+ completed cycles gets a multiplier.
    if profile.completed_cycles >= 3 {
        score = (score * 110) / 100; // 10% bonus.
    }

    // Clamp to [0, 1000].
    if score < 0 {
        0
    } else if score > 1000 {
        1000
    } else {
        score as u32
    }
}

/// Convert a numeric score to a tier.
pub fn score_to_tier(score: u32) -> ReputationTier {
    match score {
        0..=249 => ReputationTier::Bronze,
        250..=499 => ReputationTier::Silver,
        500..=749 => ReputationTier::Gold,
        _ => ReputationTier::Diamond,
    }
}
