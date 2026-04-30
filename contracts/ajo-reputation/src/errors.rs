//! AjoChain Reputation — Error definitions

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ReputationError {
    AlreadyInitialized = 300,
    NotInitialized = 301,
    NotAdmin = 302,
    ProfileNotFound = 303,
    Overflow = 304,
}
