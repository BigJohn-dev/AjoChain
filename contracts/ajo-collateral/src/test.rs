//! AjoChain Collateral — Unit tests

#![cfg(test)]

use crate::{AjoCollateralContract, AjoCollateralContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env};

fn setup_env() -> (
    Env,
    AjoCollateralContractClient<'static>,
    Address,
    Address,
    TokenClient<'static>,
    StellarAssetClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let vault_address = env.register(AjoCollateralContract, ());
    let env_static: &'static Env = Box::leak(Box::new(env.clone()));
    let client = AjoCollateralContractClient::new(env_static, &vault_address);

    let admin = Address::generate(&env);
    let token_address = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address()
        .clone();
    let token_client = TokenClient::new(env_static, &token_address);
    let token_admin = StellarAssetClient::new(env_static, &token_address);

    (env, client, admin, token_address, token_client, token_admin)
}

#[test]
fn test_initialize_vault() {
    let (env, client, admin, token, _, _) = setup_env();
    let pool = Address::generate(&env);

    client.initialize(
        &admin,
        &pool,
        &token,
        &15_000, // 150%
        &1_000_000,
        &3,
    );

    let config = client.get_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.collateral_ratio_bps, 15_000);
    // required = 1_000_000 * 3 * 15_000 / 10_000 = 4_500_000
    assert_eq!(config.required_collateral, 4_500_000);
}

#[test]
fn test_deposit_collateral() {
    let (env, client, admin, token, _, token_admin) = setup_env();
    let pool = Address::generate(&env);
    let member = Address::generate(&env);

    client.initialize(&admin, &pool, &token, &15_000, &1_000_000, &3);

    // Mint tokens for the member.
    token_admin.mint(&member, &10_000_000);

    client.deposit(&member);

    let deposit = client.get_deposit(&member);
    assert_eq!(deposit.amount, 4_500_000);
    assert_eq!(deposit.slashed_amount, 0);
    assert_eq!(deposit.is_locked, true);
}

#[test]
fn test_slash_collateral() {
    let (env, client, admin, token, token_client, token_admin) = setup_env();
    let pool = Address::generate(&env);
    let member = Address::generate(&env);

    client.initialize(&admin, &pool, &token, &15_000, &1_000_000, &3);
    token_admin.mint(&member, &10_000_000);
    client.deposit(&member);

    // Slash 1_000_000.
    client.slash(&admin, &member, &1_000_000);

    let deposit = client.get_deposit(&member);
    assert_eq!(deposit.slashed_amount, 1_000_000);

    // Pool should have received the slashed amount.
    assert_eq!(token_client.balance(&pool), 1_000_000);
}

#[test]
fn test_release_collateral() {
    let (env, client, admin, token, token_client, token_admin) = setup_env();
    let pool = Address::generate(&env);
    let member = Address::generate(&env);

    client.initialize(&admin, &pool, &token, &15_000, &1_000_000, &3);
    token_admin.mint(&member, &10_000_000);
    client.deposit(&member);

    // Slash 500_000 first.
    client.slash(&admin, &member, &500_000);

    // Release remaining.
    let balance_before = token_client.balance(&member);
    client.release(&admin, &member);

    let deposit = client.get_deposit(&member);
    assert_eq!(deposit.is_locked, false);

    // Member should have received 4_500_000 - 500_000 = 4_000_000 back.
    let balance_after = token_client.balance(&member);
    assert_eq!(balance_after - balance_before, 4_000_000);
}

#[test]
fn test_slash_capped_at_remaining() {
    let (env, client, admin, token, _, token_admin) = setup_env();
    let pool = Address::generate(&env);
    let member = Address::generate(&env);

    client.initialize(&admin, &pool, &token, &15_000, &1_000_000, &3);
    token_admin.mint(&member, &10_000_000);
    client.deposit(&member);

    // Slash more than available — should be capped.
    client.slash(&admin, &member, &100_000_000);

    let deposit = client.get_deposit(&member);
    assert_eq!(deposit.slashed_amount, 4_500_000); // Capped at full amount.
}
