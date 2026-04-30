//! # AjoChain Factory Contract
//!
//! Deploys and registers new ROSCA pool instances. Maintains a global registry
//! of all pools and configures protocol-level fees and token allowlists.
//!
//! ## Responsibilities
//! - Deploy isolated `ajo-pool` contract instances
//! - Maintain a global pool registry with pagination support
//! - Configure protocol fee (basis points)
//! - Admin-controlled allowlist for supported contribution tokens

#![no_std]

mod errors;
mod events;
mod storage;

use errors::FactoryError;
use storage::{DataKey, FactoryConfig, PoolEntry};

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

#[contract]
pub struct AjoFactoryContract;

#[contractimpl]
impl AjoFactoryContract {
    /// Initialise the factory contract.
    ///
    /// # Arguments
    /// * `admin` — The protocol administrator.
    /// * `pool_wasm_hash` — The WASM hash of the compiled ajo-pool contract.
    /// * `default_fee_bps` — Default protocol fee in basis points.
    pub fn initialize(
        env: Env,
        admin: Address,
        pool_wasm_hash: BytesN<32>,
        default_fee_bps: u32,
    ) -> Result<(), FactoryError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Config) {
            return Err(FactoryError::AlreadyInitialized);
        }

        let config = FactoryConfig {
            admin: admin.clone(),
            pool_wasm_hash,
            default_fee_bps,
            total_pools: 0,
            is_paused: false,
        };

        env.storage().instance().set(&DataKey::Config, &config);

        // Initialise empty token allowlist.
        let empty_list: Vec<Address> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::AllowedTokens, &empty_list);

        events::factory_initialized(&env, &admin, default_fee_bps);

        Ok(())
    }

    /// Add a token to the allowlist. Admin only.
    pub fn add_allowed_token(
        env: Env,
        admin: Address,
        token: Address,
    ) -> Result<(), FactoryError> {
        admin.require_auth();
        let config: FactoryConfig = Self::get_config_checked(&env, &admin)?;
        let _ = config;

        let mut tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokens)
            .unwrap_or(Vec::new(&env));

        // Check for duplicates.
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == token {
                return Err(FactoryError::TokenAlreadyAllowed);
            }
        }

        tokens.push_back(token.clone());
        env.storage()
            .instance()
            .set(&DataKey::AllowedTokens, &tokens);

        events::token_allowed(&env, &token);

        Ok(())
    }

    /// Remove a token from the allowlist. Admin only.
    pub fn remove_allowed_token(
        env: Env,
        admin: Address,
        token: Address,
    ) -> Result<(), FactoryError> {
        admin.require_auth();
        let config: FactoryConfig = Self::get_config_checked(&env, &admin)?;
        let _ = config;

        let mut tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokens)
            .unwrap_or(Vec::new(&env));

        let mut found = false;
        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == token {
                tokens.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            return Err(FactoryError::TokenNotAllowed);
        }

        env.storage()
            .instance()
            .set(&DataKey::AllowedTokens, &tokens);

        events::token_removed(&env, &token);

        Ok(())
    }

    /// Check if a token is allowed.
    pub fn is_token_allowed(env: Env, token: Address) -> bool {
        let tokens: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::AllowedTokens)
            .unwrap_or(Vec::new(&env));

        for i in 0..tokens.len() {
            if tokens.get(i).unwrap() == token {
                return true;
            }
        }

        false
    }

    /// Register a pool in the factory registry (called after external pool creation).
    pub fn register_pool(
        env: Env,
        admin: Address,
        pool_address: Address,
        token: Address,
        creator: Address,
    ) -> Result<u64, FactoryError> {
        admin.require_auth();

        let mut config: FactoryConfig = Self::get_config_checked(&env, &admin)?;

        let pool_index = config
            .total_pools
            .checked_add(1)
            .ok_or(FactoryError::Overflow)?;

        let entry = PoolEntry {
            pool_index,
            pool_address: pool_address.clone(),
            token: token.clone(),
            creator: creator.clone(),
            created_at: env.ledger().timestamp(),
            is_active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_index), &entry);

        config.total_pools = pool_index;
        env.storage().instance().set(&DataKey::Config, &config);

        events::pool_registered(&env, pool_index, &pool_address, &creator);

        Ok(pool_index)
    }

    /// Pause the factory. Prevents new pool deployments.
    pub fn pause(env: Env, admin: Address) -> Result<(), FactoryError> {
        admin.require_auth();
        let mut config: FactoryConfig = Self::get_config_checked(&env, &admin)?;
        config.is_paused = true;
        env.storage().instance().set(&DataKey::Config, &config);
        Ok(())
    }

    /// Unpause the factory.
    pub fn unpause(env: Env, admin: Address) -> Result<(), FactoryError> {
        admin.require_auth();
        let mut config: FactoryConfig = Self::get_config_checked(&env, &admin)?;
        config.is_paused = false;
        env.storage().instance().set(&DataKey::Config, &config);
        Ok(())
    }

    /// Update the default fee. Admin only.
    pub fn update_fee(env: Env, admin: Address, new_fee_bps: u32) -> Result<(), FactoryError> {
        admin.require_auth();
        let mut config: FactoryConfig = Self::get_config_checked(&env, &admin)?;
        config.default_fee_bps = new_fee_bps;
        env.storage().instance().set(&DataKey::Config, &config);
        events::fee_updated(&env, new_fee_bps);
        Ok(())
    }

    // ─── Read-Only Queries ──────────────────────────────────────────────

    /// Get factory configuration.
    pub fn get_factory_config(env: Env) -> Result<FactoryConfig, FactoryError> {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(FactoryError::NotInitialized)
    }

    /// Get a registered pool by index.
    pub fn get_pool(env: Env, pool_index: u64) -> Result<PoolEntry, FactoryError> {
        env.storage()
            .persistent()
            .get(&DataKey::Pool(pool_index))
            .ok_or(FactoryError::PoolNotFound)
    }

    /// Get total number of registered pools.
    pub fn get_total_pools(env: Env) -> u64 {
        let config: Option<FactoryConfig> = env.storage().instance().get(&DataKey::Config);
        config.map(|c| c.total_pools).unwrap_or(0)
    }

    /// Paginated pool listing. Returns up to `limit` pools starting from `cursor`.
    pub fn list_pools(env: Env, cursor: u64, limit: u32) -> Vec<PoolEntry> {
        let config: Option<FactoryConfig> = env.storage().instance().get(&DataKey::Config);
        let total = config.map(|c| c.total_pools).unwrap_or(0);

        let mut results: Vec<PoolEntry> = Vec::new(&env);
        let mut count = 0_u32;
        let mut idx = cursor;

        while idx <= total && count < limit {
            if let Some(entry) = env
                .storage()
                .persistent()
                .get::<DataKey, PoolEntry>(&DataKey::Pool(idx))
            {
                results.push_back(entry);
                count += 1;
            }
            idx += 1;
        }

        results
    }

    // ─── Internal Helpers ───────────────────────────────────────────────

    fn get_config_checked(env: &Env, admin: &Address) -> Result<FactoryConfig, FactoryError> {
        let config: FactoryConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(FactoryError::NotInitialized)?;

        if config.admin != *admin {
            return Err(FactoryError::NotAdmin);
        }

        Ok(config)
    }
}

#[cfg(test)]
mod test;
