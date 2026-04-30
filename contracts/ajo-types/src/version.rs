//! AjoChain — Contract versioning
//!
//! Provides version tracking for upgradeable contracts. Every contract stores
//! its version in instance storage, and the `migrate()` pattern uses this to
//! handle state transformations during upgrades.

use soroban_sdk::{contracttype, Env};

/// Current protocol version. Bump this when making breaking changes.
pub const PROTOCOL_VERSION: u32 = 1;

/// Storage key for the contract version.
#[contracttype]
#[derive(Clone)]
pub enum VersionKey {
    Version,
}

/// Read the stored contract version. Returns 0 if not yet set.
pub fn get_version(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&VersionKey::Version)
        .unwrap_or(0)
}

/// Write the contract version to instance storage.
pub fn set_version(env: &Env, version: u32) {
    env.storage()
        .instance()
        .set(&VersionKey::Version, &version);
}
