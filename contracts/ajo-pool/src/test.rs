//! AjoChain Pool — Unit and integration tests
//!
//! Tests the complete ROSCA lifecycle: pool creation → member joining →
//! contributions → payout distribution → cycle completion.

#![cfg(test)]

use crate::storage::{PoolState};
use crate::{AjoPoolContract, AjoPoolContractClient};
use soroban_sdk::testutils::{Address as _, Ledger, LedgerInfo};
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env};

/// Helper: set up the test environment with a token and the pool contract.
fn setup_env() -> (Env, Address, AjoPoolContractClient<'static>, Address, TokenClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy the pool contract.
    let pool_address = env.register(AjoPoolContract, ());
    let pool_client = AjoPoolContractClient::new(&env, &pool_address);

    // Deploy a test token.
    let admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(admin.clone()).address().clone();
    let token_client = TokenClient::new(&env, &token_address);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);

    // Mint tokens to test accounts (done per-test).
    // We return the admin client for minting in tests.
    let _ = token_admin_client;

    // Leak env for static lifetime (test-only pattern).
    let env_static: &'static Env = Box::leak(Box::new(env.clone()));
    let pool_client_static = AjoPoolContractClient::new(env_static, &pool_address);

    (env, admin, pool_client_static, token_address, token_client)
}

/// Helper: create a standard 3-person pool.
fn create_standard_pool(
    env: &Env,
    client: &AjoPoolContractClient,
    admin: &Address,
    token: &Address,
) -> u64 {
    client.create_pool(
        admin,
        token,
        &1_000_000_i128, // 0.1 XLM in stroops
        &604_800_u64,     // weekly
        &3_u32,           // max 3 members
        &3_u32,           // min 3 members
        &0_u32,           // FixedRotation
        &50_u32,          // 0.5% fee
    )
}

#[test]
fn test_create_pool() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);
    assert_eq!(pool_id, 1);

    let config = client.get_pool(&pool_id);
    assert_eq!(config.contribution_amount, 1_000_000);
    assert_eq!(config.max_members, 3);
    assert_eq!(config.member_count, 0);
    assert_eq!(config.state, PoolState::Recruiting);
}

#[test]
fn test_join_pool() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);

    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);
    let member3 = Address::generate(&env);

    let idx1 = client.join_pool(&pool_id, &member1);
    let idx2 = client.join_pool(&pool_id, &member2);
    let idx3 = client.join_pool(&pool_id, &member3);

    assert_eq!(idx1, 0);
    assert_eq!(idx2, 1);
    assert_eq!(idx3, 2);

    let config = client.get_pool(&pool_id);
    assert_eq!(config.member_count, 3);
}

#[test]
fn test_pool_count() {
    let (env, admin, client, token, _) = setup_env();

    assert_eq!(client.get_pool_count(), 0);

    create_standard_pool(&env, &client, &admin, &token);
    assert_eq!(client.get_pool_count(), 1);

    create_standard_pool(&env, &client, &admin, &token);
    assert_eq!(client.get_pool_count(), 2);
}

#[test]
fn test_start_cycle() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);

    // Add 3 members.
    let m1 = Address::generate(&env);
    let m2 = Address::generate(&env);
    let m3 = Address::generate(&env);
    client.join_pool(&pool_id, &m1);
    client.join_pool(&pool_id, &m2);
    client.join_pool(&pool_id, &m3);

    // Start the cycle.
    client.start_cycle(&pool_id, &admin);

    let config = client.get_pool(&pool_id);
    assert_eq!(config.state, PoolState::Active);
    assert_eq!(config.total_rounds, 3);
    assert_eq!(config.current_round, 0);
}

#[test]
fn test_full_cycle_fixed_rotation() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);

    // Create members and mint tokens.
    let m1 = Address::generate(&env);
    let m2 = Address::generate(&env);
    let m3 = Address::generate(&env);

    let token_admin_client = StellarAssetClient::new(&env, &token);
    token_admin_client.mint(&m1, &100_000_000);
    token_admin_client.mint(&m2, &100_000_000);
    token_admin_client.mint(&m3, &100_000_000);

    client.join_pool(&pool_id, &m1);
    client.join_pool(&pool_id, &m2);
    client.join_pool(&pool_id, &m3);

    client.start_cycle(&pool_id, &admin);

    // ── Round 0: All contribute, m1 receives payout ──
    client.contribute(&pool_id, &m1);
    client.contribute(&pool_id, &m2);
    client.contribute(&pool_id, &m3);

    let recipient0 = client.trigger_payout(&pool_id);
    assert_eq!(recipient0, m1);

    // ── Round 1: All contribute, m2 receives payout ──
    client.contribute(&pool_id, &m1);
    client.contribute(&pool_id, &m2);
    client.contribute(&pool_id, &m3);

    let recipient1 = client.trigger_payout(&pool_id);
    assert_eq!(recipient1, m2);

    // ── Round 2: All contribute, m3 receives payout ──
    client.contribute(&pool_id, &m1);
    client.contribute(&pool_id, &m2);
    client.contribute(&pool_id, &m3);

    let recipient2 = client.trigger_payout(&pool_id);
    assert_eq!(recipient2, m3);

    // Pool should now be completed.
    let config = client.get_pool(&pool_id);
    assert_eq!(config.state, PoolState::Completed);
}

#[test]
fn test_leave_pool_during_recruiting() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);

    let m1 = Address::generate(&env);
    client.join_pool(&pool_id, &m1);
    assert_eq!(client.get_pool(&pool_id).member_count, 1);

    client.leave_pool(&pool_id, &m1);
    assert_eq!(client.get_pool(&pool_id).member_count, 0);
}

#[test]
fn test_cancel_pool() {
    let (env, admin, client, token, _) = setup_env();

    let pool_id = create_standard_pool(&env, &client, &admin, &token);

    client.cancel_pool(&pool_id, &admin);

    let config = client.get_pool(&pool_id);
    assert_eq!(config.state, PoolState::Cancelled);
}
