//! AjoChain Reputation — Unit tests

#![cfg(test)]

use crate::scoring::ReputationTier;
use crate::{AjoReputationContract, AjoReputationContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

fn setup_env() -> (Env, AjoReputationContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_address = env.register(AjoReputationContract, ());
    let env_static: &'static Env = Box::leak(Box::new(env.clone()));
    let client = AjoReputationContractClient::new(env_static, &contract_address);

    let admin = Address::generate(&env);

    (env, client, admin)
}

#[test]
fn test_initialize() {
    let (_, client, admin) = setup_env();
    client.initialize(&admin);
    // Should not panic — oracle is initialised.
}

#[test]
fn test_record_cycle_creates_profile() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);

    client.record_cycle(
        &admin, &member,
        &10, // on_time
        &0,  // late
        &10, // total_rounds
        &true, // completed
    );

    let profile = client.get_profile(&member);
    assert_eq!(profile.total_cycles, 1);
    assert_eq!(profile.completed_cycles, 1);
    assert_eq!(profile.total_on_time, 10);
    assert_eq!(profile.total_late, 0);
}

#[test]
fn test_reputation_tiers() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);

    // First cycle — should give a Gold+ score (starting 500 + completion bonus).
    client.record_cycle(&admin, &member, &10, &0, &10, &true);

    let tier = client.get_tier(&member);
    // Score = 500 (base) + 50 (1 completed) + 50 (completed this time) + 30 (10*3 on-time) = 630 → Gold
    assert_eq!(tier, ReputationTier::Gold);
}

#[test]
fn test_dispute_penalty() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);
    client.record_cycle(&admin, &member, &5, &0, &5, &true);

    let score_before = client.get_profile(&member).score;

    client.record_dispute(&admin, &member, &true); // dispute against

    let score_after = client.get_profile(&member).score;
    assert!(score_after < score_before);
}

#[test]
fn test_meets_tier() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);
    client.record_cycle(&admin, &member, &10, &0, &10, &true);

    // Should meet Bronze (0), Silver (1), Gold (2).
    assert_eq!(client.meets_tier(&member, &0), true);
    assert_eq!(client.meets_tier(&member, &1), true);
    assert_eq!(client.meets_tier(&member, &2), true);
    // Probably not Diamond yet after 1 cycle.
    // (Score = 630 → Gold, not Diamond which requires 750+)
    assert_eq!(client.meets_tier(&member, &3), false);
}

#[test]
fn test_new_user_bronze_only() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);

    // Unknown member should only qualify for Bronze tier (0).
    assert_eq!(client.meets_tier(&member, &0), true);
    assert_eq!(client.meets_tier(&member, &1), false);
}

#[test]
fn test_multiple_cycles_increase_score() {
    let (env, client, admin) = setup_env();
    client.initialize(&admin);

    let member = Address::generate(&env);

    for _ in 0..5 {
        client.record_cycle(&admin, &member, &10, &0, &10, &true);
    }

    let profile = client.get_profile(&member);
    assert_eq!(profile.total_cycles, 5);
    assert_eq!(profile.completed_cycles, 5);

    // With 5 completions + consistency bonus, should be Diamond (750+).
    let tier = client.get_tier(&member);
    assert_eq!(tier, ReputationTier::Diamond);
}
