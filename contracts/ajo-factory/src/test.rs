//! AjoChain Factory — Unit tests

#![cfg(test)]

use crate::{AjoFactoryContract, AjoFactoryContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env};

fn setup_env() -> (Env, AjoFactoryContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_address = env.register(AjoFactoryContract, ());
    let env_static: &'static Env = Box::leak(Box::new(env.clone()));
    let client = AjoFactoryContractClient::new(env_static, &contract_address);

    let admin = Address::generate(&env);

    (env, client, admin)
}

#[test]
fn test_initialize() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);

    client.initialize(&admin, &wasm_hash, &50);

    let config = client.get_factory_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.default_fee_bps, 50);
    assert_eq!(config.total_pools, 0);
    assert_eq!(config.is_paused, false);
}

#[test]
fn test_add_remove_allowed_token() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.initialize(&admin, &wasm_hash, &50);

    let token = Address::generate(&env);

    // Add token.
    client.add_allowed_token(&admin, &token);
    assert_eq!(client.is_token_allowed(&token), true);

    // Remove token.
    client.remove_allowed_token(&admin, &token);
    assert_eq!(client.is_token_allowed(&token), false);
}

#[test]
fn test_register_pool() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.initialize(&admin, &wasm_hash, &50);

    let pool_addr = Address::generate(&env);
    let token = Address::generate(&env);
    let creator = Address::generate(&env);

    let idx = client.register_pool(&admin, &pool_addr, &token, &creator);
    assert_eq!(idx, 1);
    assert_eq!(client.get_total_pools(), 1);

    let entry = client.get_pool(&1);
    assert_eq!(entry.pool_address, pool_addr);
    assert_eq!(entry.creator, creator);
    assert_eq!(entry.is_active, true);
}

#[test]
fn test_pause_unpause() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.initialize(&admin, &wasm_hash, &50);

    client.pause(&admin);
    assert_eq!(client.get_factory_config().is_paused, true);

    client.unpause(&admin);
    assert_eq!(client.get_factory_config().is_paused, false);
}

#[test]
fn test_update_fee() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.initialize(&admin, &wasm_hash, &50);

    client.update_fee(&admin, &100);
    assert_eq!(client.get_factory_config().default_fee_bps, 100);
}

#[test]
fn test_list_pools_pagination() {
    let (env, client, admin) = setup_env();
    let wasm_hash = BytesN::from_array(&env, &[0u8; 32]);
    client.initialize(&admin, &wasm_hash, &50);

    // Register 5 pools.
    for _ in 0..5 {
        let pool = Address::generate(&env);
        let token = Address::generate(&env);
        let creator = Address::generate(&env);
        client.register_pool(&admin, &pool, &token, &creator);
    }

    assert_eq!(client.get_total_pools(), 5);

    // Page 1: first 2.
    let page1 = client.list_pools(&1, &2);
    assert_eq!(page1.len(), 2);

    // Page 2: next 2.
    let page2 = client.list_pools(&3, &2);
    assert_eq!(page2.len(), 2);

    // Page 3: last 1.
    let page3 = client.list_pools(&5, &2);
    assert_eq!(page3.len(), 1);
}
