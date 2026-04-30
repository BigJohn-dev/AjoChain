//! AjoChain Governance — Unit tests

#![cfg(test)]

use crate::proposals::{ProposalAction, ProposalStatus};
use crate::{AjoGovernanceContract, AjoGovernanceContractClient};
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, Env, String, Vec};

fn setup_env() -> (Env, AjoGovernanceContractClient<'static>, Address, Vec<Address>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_address = env.register(AjoGovernanceContract, ());
    let env_static: &'static Env = Box::leak(Box::new(env.clone()));
    let client = AjoGovernanceContractClient::new(env_static, &contract_address);

    let admin = Address::generate(&env);
    let council1 = Address::generate(&env);
    let council2 = Address::generate(&env);

    let mut council = Vec::new(&env);
    council.push_back(council1.clone());
    council.push_back(council2.clone());

    (env, client, admin, council)
}

#[test]
fn test_initialize() {
    let (_, client, admin, council) = setup_env();

    client.initialize(&admin, &council, &172_800);

    let config = client.get_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.timelock_delay, 172_800);
    assert_eq!(config.proposal_count, 0);
    assert_eq!(config.is_paused, false);
}

#[test]
fn test_create_proposal() {
    let (env, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &172_800);

    let title = String::from_str(&env, "Update fee to 1%");
    let desc = String::from_str(&env, "Change protocol fee from 0.5% to 1%");
    let action = ProposalAction::UpdateFee(100);

    let id = client.create_proposal(&admin, &title, &desc, &action);
    assert_eq!(id, 1);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.status, ProposalStatus::Pending);
    assert_eq!(proposal.proposer, admin);
}

#[test]
fn test_execute_proposal_after_timelock() {
    let (env, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &100); // 100 seconds delay.

    let title = String::from_str(&env, "Test proposal");
    let desc = String::from_str(&env, "Test");
    let action = ProposalAction::UpdateFee(100);

    let id = client.create_proposal(&admin, &title, &desc, &action);

    // Advance time past the timelock.
    env.ledger().with_mut(|li| {
        li.timestamp += 200;
    });

    client.execute_proposal(&admin, &id);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.status, ProposalStatus::Executed);
}

#[test]
fn test_veto_proposal() {
    let (env, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &172_800);

    let title = String::from_str(&env, "Bad proposal");
    let desc = String::from_str(&env, "This should be vetoed");
    let action = ProposalAction::UpdateFee(5000);

    let id = client.create_proposal(&admin, &title, &desc, &action);

    // Council member vetoes.
    let council_member = council.get(0).unwrap();
    client.veto_proposal(&council_member, &id);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.status, ProposalStatus::Vetoed);
    assert_eq!(proposal.vetoed_by, Some(council_member));
}

#[test]
fn test_emergency_pause_unpause() {
    let (_, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &172_800);

    client.emergency_pause(&admin);
    assert_eq!(client.get_config().is_paused, true);

    client.unpause(&admin);
    assert_eq!(client.get_config().is_paused, false);
}

#[test]
fn test_proposal_count() {
    let (env, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &172_800);

    assert_eq!(client.get_proposal_count(), 0);

    for i in 0..3 {
        let title = String::from_str(&env, "Proposal");
        let desc = String::from_str(&env, "Description");
        let action = ProposalAction::UpdateFee(i * 10);
        client.create_proposal(&admin, &title, &desc, &action);
    }

    assert_eq!(client.get_proposal_count(), 3);
}

#[test]
fn test_council_member_can_propose() {
    let (env, client, admin, council) = setup_env();
    client.initialize(&admin, &council, &172_800);

    let council_member = council.get(0).unwrap();
    let title = String::from_str(&env, "Council proposal");
    let desc = String::from_str(&env, "From council");
    let action = ProposalAction::UpdateFee(75);

    let id = client.create_proposal(&council_member, &title, &desc, &action);
    assert_eq!(id, 1);

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.proposer, council_member);
}
