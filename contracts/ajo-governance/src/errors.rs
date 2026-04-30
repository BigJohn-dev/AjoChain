//! AjoChain Governance — Error definitions

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GovernanceError {
    AlreadyInitialized = 400,
    NotInitialized = 401,
    NotAuthorized = 402,
    ProposalNotFound = 403,
    InvalidStatus = 404,
    TimelockNotElapsed = 405,
    Paused = 406,
    Overflow = 407,
}
