//! # AjoChain Collateral Vault Contract
//!
//! Manages collateral deposits for ROSCA pool members. Members must lock
//! collateral (configurable ratio) when joining a pool. Collateral is:
//! - Slashed on missed contributions (default)
//! - Released on pool completion
//! - Partially liquidated for late payments

#![no_std]

mod errors;
mod events;
mod vault;

use errors::CollateralError;
use vault::{DataKey, VaultConfig, VaultDeposit};

use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

#[contract]
pub struct AjoCollateralContract;

#[contractimpl]
impl AjoCollateralContract {
    /// Initialise the collateral vault for a specific pool.
    ///
    /// # Arguments
    /// * `admin` — The vault administrator (typically the pool admin).
    /// * `pool_address` — The associated ajo-pool contract address.
    /// * `token` — The token used for collateral (same as pool contribution token).
    /// * `collateral_ratio_bps` — Ratio as basis points (e.g. 15000 = 150% of contribution).
    /// * `contribution_amount` — The pool's per-round contribution amount.
    /// * `total_rounds` — Expected number of rounds in the pool.
    pub fn initialize(
        env: Env,
        admin: Address,
        pool_address: Address,
        token: Address,
        collateral_ratio_bps: u32,
        contribution_amount: i128,
        total_rounds: u32,
    ) -> Result<(), CollateralError> {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Config) {
            return Err(CollateralError::AlreadyInitialized);
        }

        // Calculate required collateral per member.
        // required = (contribution_amount * total_rounds * ratio_bps) / 10_000
        let required = contribution_amount
            .checked_mul(total_rounds as i128)
            .ok_or(CollateralError::Overflow)?
            .checked_mul(collateral_ratio_bps as i128)
            .ok_or(CollateralError::Overflow)?
            / 10_000;

        let config = VaultConfig {
            admin: admin.clone(),
            pool_address,
            token: token.clone(),
            collateral_ratio_bps,
            required_collateral: required,
            contribution_amount,
            total_rounds,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        events::vault_initialized(&env, &admin, &token, required);

        Ok(())
    }

    /// Deposit collateral when joining a pool.
    pub fn deposit(env: Env, member: Address) -> Result<(), CollateralError> {
        member.require_auth();

        let config: VaultConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(CollateralError::NotInitialized)?;

        // Check for existing deposit.
        if env
            .storage()
            .persistent()
            .has(&DataKey::Deposit(member.clone()))
        {
            return Err(CollateralError::AlreadyDeposited);
        }

        // Transfer collateral from member to vault.
        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(
            &member,
            &env.current_contract_address(),
            &config.required_collateral,
        );

        let deposit = VaultDeposit {
            member: member.clone(),
            amount: config.required_collateral,
            slashed_amount: 0,
            is_locked: true,
            deposited_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Deposit(member.clone()), &deposit);

        events::collateral_deposited(&env, &member, config.required_collateral);

        Ok(())
    }

    /// Slash a member's collateral for a missed contribution. Pool admin only.
    pub fn slash(
        env: Env,
        admin: Address,
        member: Address,
        slash_amount: i128,
    ) -> Result<(), CollateralError> {
        admin.require_auth();

        let config: VaultConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(CollateralError::NotInitialized)?;

        if config.admin != admin {
            return Err(CollateralError::NotAdmin);
        }

        let mut deposit: VaultDeposit = env
            .storage()
            .persistent()
            .get(&DataKey::Deposit(member.clone()))
            .ok_or(CollateralError::NoDeposit)?;

        let remaining = deposit
            .amount
            .checked_sub(deposit.slashed_amount)
            .ok_or(CollateralError::Overflow)?;

        let actual_slash = if slash_amount > remaining {
            remaining
        } else {
            slash_amount
        };

        deposit.slashed_amount = deposit
            .slashed_amount
            .checked_add(actual_slash)
            .ok_or(CollateralError::Overflow)?;

        env.storage()
            .persistent()
            .set(&DataKey::Deposit(member.clone()), &deposit);

        // Transfer slashed amount to the pool (or admin for redistribution).
        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(
            &env.current_contract_address(),
            &config.pool_address,
            &actual_slash,
        );

        events::collateral_slashed(&env, &member, actual_slash);

        Ok(())
    }

    /// Release a member's remaining collateral after pool completion.
    pub fn release(env: Env, admin: Address, member: Address) -> Result<(), CollateralError> {
        admin.require_auth();

        let config: VaultConfig = env
            .storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(CollateralError::NotInitialized)?;

        if config.admin != admin {
            return Err(CollateralError::NotAdmin);
        }

        let mut deposit: VaultDeposit = env
            .storage()
            .persistent()
            .get(&DataKey::Deposit(member.clone()))
            .ok_or(CollateralError::NoDeposit)?;

        if !deposit.is_locked {
            return Err(CollateralError::AlreadyReleased);
        }

        let refund = deposit
            .amount
            .checked_sub(deposit.slashed_amount)
            .ok_or(CollateralError::Overflow)?;

        if refund > 0 {
            let token_client = token::Client::new(&env, &config.token);
            token_client.transfer(&env.current_contract_address(), &member, &refund);
        }

        deposit.is_locked = false;
        env.storage()
            .persistent()
            .set(&DataKey::Deposit(member.clone()), &deposit);

        events::collateral_released(&env, &member, refund);

        Ok(())
    }

    // ─── Read-Only Queries ──────────────────────────────────────────────

    /// Get a member's collateral deposit status.
    pub fn get_deposit(env: Env, member: Address) -> Result<VaultDeposit, CollateralError> {
        env.storage()
            .persistent()
            .get(&DataKey::Deposit(member))
            .ok_or(CollateralError::NoDeposit)
    }

    /// Get the vault configuration.
    pub fn get_config(env: Env) -> Result<VaultConfig, CollateralError> {
        env.storage()
            .instance()
            .get(&DataKey::Config)
            .ok_or(CollateralError::NotInitialized)
    }
}

#[cfg(test)]
mod test;
