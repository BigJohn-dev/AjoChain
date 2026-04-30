//! AjoChain — Storage TTL (Time-To-Live) Configuration
//!
//! Soroban persistent and instance storage entries must be regularly extended
//! (bumped) to prevent them from being archived. This module centralises
//! TTL constants and provides helper functions for consistent TTL management.
//!
//! ## TTL Strategy
//!
//! | Storage Type  | Min TTL  | Max TTL  | Rationale                           |
//! |---------------|----------|----------|-------------------------------------|
//! | Instance      | 30 days  | 90 days  | Contract config, always needed      |
//! | Persistent    | 60 days  | 180 days | Pool data, member records           |
//! | Temporary     | N/A      | N/A      | Auto-expired, no bumping needed     |

use soroban_sdk::Env;

/// Instance storage: minimum TTL in ledgers (~30 days at 5s/ledger).
pub const INSTANCE_TTL_THRESHOLD: u32 = 518_400;
/// Instance storage: extend-to TTL in ledgers (~90 days).
pub const INSTANCE_TTL_EXTEND: u32 = 1_555_200;

/// Persistent storage: minimum TTL in ledgers (~60 days).
pub const PERSISTENT_TTL_THRESHOLD: u32 = 1_036_800;
/// Persistent storage: extend-to TTL in ledgers (~180 days).
pub const PERSISTENT_TTL_EXTEND: u32 = 3_110_400;

/// Extend the TTL of the contract's instance storage if it has fallen
/// below the threshold. Call this at the beginning of every public entry-point.
pub fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_EXTEND);
}

/// Extend the TTL of a specific persistent storage key.
/// Call this after reading or writing important persistent data.
pub fn extend_persistent_ttl<K: soroban_sdk::TryIntoVal<Env, soroban_sdk::Val>>(
    env: &Env,
    key: &K,
) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_EXTEND);
}
