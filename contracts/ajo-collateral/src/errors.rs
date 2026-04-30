//! AjoChain Collateral — Error definitions

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CollateralError {
    AlreadyInitialized = 200,
    NotInitialized = 201,
    NotAdmin = 202,
    AlreadyDeposited = 203,
    NoDeposit = 204,
    InsufficientCollateral = 205,
    AlreadyReleased = 206,
    Overflow = 207,
}
