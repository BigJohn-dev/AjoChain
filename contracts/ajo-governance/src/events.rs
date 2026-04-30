//! AjoChain Governance — Event emissions

use soroban_sdk::{Address, Env, Symbol};

pub fn governance_initialized(env: &Env, admin: &Address, timelock_delay: u64) {
    env.events().publish(
        (Symbol::new(env, "gov_init"),),
        (admin.clone(), timelock_delay),
    );
}

pub fn proposal_created(env: &Env, proposal_id: u64, proposer: &Address) {
    env.events().publish(
        (Symbol::new(env, "proposal_new"), proposal_id),
        proposer.clone(),
    );
}

pub fn proposal_executed(env: &Env, proposal_id: u64) {
    env.events().publish(
        (Symbol::new(env, "proposal_exec"), proposal_id),
        true,
    );
}

pub fn proposal_vetoed(env: &Env, proposal_id: u64, vetoed_by: &Address) {
    env.events().publish(
        (Symbol::new(env, "proposal_veto"), proposal_id),
        vetoed_by.clone(),
    );
}

pub fn emergency_paused(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "emergency_pause"),),
        admin.clone(),
    );
}

pub fn emergency_unpaused(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "emergency_unpause"),),
        admin.clone(),
    );
}
