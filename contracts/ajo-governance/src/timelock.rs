//! AjoChain Governance — Timelock utilities
//!
//! Provides helper functions for timelock validation and delay computation.

use crate::errors::GovernanceError;
use crate::proposals::{DataKey, GovernanceConfig, Proposal, ProposalStatus};
use soroban_sdk::Env;

/// Check if a proposal's timelock has elapsed.
pub fn is_timelock_elapsed(env: &Env, proposal: &Proposal) -> bool {
    env.ledger().timestamp() >= proposal.executable_at
}

/// Get the remaining time until a proposal can be executed.
/// Returns 0 if the timelock has already elapsed.
pub fn remaining_delay(env: &Env, proposal: &Proposal) -> u64 {
    let now = env.ledger().timestamp();
    if now >= proposal.executable_at {
        0
    } else {
        proposal.executable_at - now
    }
}

/// Validate that a proposal is in a valid state for execution.
pub fn validate_execution(
    env: &Env,
    proposal: &Proposal,
) -> Result<(), GovernanceError> {
    if proposal.status != ProposalStatus::Pending {
        return Err(GovernanceError::InvalidStatus);
    }

    if !is_timelock_elapsed(env, proposal) {
        return Err(GovernanceError::TimelockNotElapsed);
    }

    // Check if governance is paused.
    let config: GovernanceConfig = env
        .storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(GovernanceError::NotInitialized)?;

    if config.is_paused {
        return Err(GovernanceError::Paused);
    }

    Ok(())
}
