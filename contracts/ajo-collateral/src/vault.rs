//! AjoChain Collateral — Storage keys and data types

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Vault configuration (Instance storage).
    Config,
    /// Per-member collateral deposit (Persistent storage).
    Deposit(Address),
}

/// Vault configuration.
#[contracttype]
#[derive(Clone, Debug)]
pub struct VaultConfig {
    /// Vault administrator.
    pub admin: Address,
    /// Associated pool contract address.
    pub pool_address: Address,
    /// Token used for collateral.
    pub token: Address,
    /// Collateral ratio in basis points (e.g. 15000 = 150%).
    pub collateral_ratio_bps: u32,
    /// Required collateral amount per member.
    pub required_collateral: i128,
    /// Pool's per-round contribution amount.
    pub contribution_amount: i128,
    /// Expected total rounds in the pool.
    pub total_rounds: u32,
}

/// A member's collateral deposit.
#[contracttype]
#[derive(Clone, Debug)]
pub struct VaultDeposit {
    /// Member address.
    pub member: Address,
    /// Original deposit amount.
    pub amount: i128,
    /// Total amount slashed so far.
    pub slashed_amount: i128,
    /// Whether the deposit is still locked.
    pub is_locked: bool,
    /// When the deposit was made.
    pub deposited_at: u64,
}
