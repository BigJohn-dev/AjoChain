//! AjoChain Factory — Storage keys and data types

use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Factory configuration (Instance storage).
    Config,
    /// Registered pool entry — keyed by pool_index (Persistent storage).
    Pool(u64),
    /// Token allowlist (Instance storage).
    AllowedTokens,
}

/// Factory-level configuration.
#[contracttype]
#[derive(Clone, Debug)]
pub struct FactoryConfig {
    /// Protocol administrator address.
    pub admin: Address,
    /// WASM hash of the ajo-pool contract for deploying new instances.
    pub pool_wasm_hash: BytesN<32>,
    /// Default protocol fee in basis points.
    pub default_fee_bps: u32,
    /// Total number of pools registered.
    pub total_pools: u64,
    /// Whether the factory is paused (no new deployments).
    pub is_paused: bool,
}

/// A registered pool entry in the factory registry.
#[contracttype]
#[derive(Clone, Debug)]
pub struct PoolEntry {
    /// Sequential pool index (1-based).
    pub pool_index: u64,
    /// The deployed pool contract address.
    pub pool_address: Address,
    /// The contribution token for this pool.
    pub token: Address,
    /// The address that created this pool.
    pub creator: Address,
    /// Ledger timestamp of registration.
    pub created_at: u64,
    /// Whether the pool is still active.
    pub is_active: bool,
}
