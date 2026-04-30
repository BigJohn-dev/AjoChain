//! AjoChain Factory — Error definitions

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    /// The factory has already been initialised.
    AlreadyInitialized = 100,
    /// The factory has not been initialised.
    NotInitialized = 101,
    /// The caller is not the protocol administrator.
    NotAdmin = 102,
    /// The factory is currently paused.
    Paused = 103,
    /// The requested pool was not found in the registry.
    PoolNotFound = 104,
    /// The token is already in the allowlist.
    TokenAlreadyAllowed = 105,
    /// The token is not in the allowlist.
    TokenNotAllowed = 106,
    /// An arithmetic overflow occurred.
    Overflow = 107,
}
