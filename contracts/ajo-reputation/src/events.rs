//! AjoChain Reputation — Event emissions

use soroban_sdk::{Address, Env, Symbol};

pub fn oracle_initialized(env: &Env, admin: &Address) {
    env.events().publish(
        (Symbol::new(env, "rep_init"),),
        admin.clone(),
    );
}

pub fn reputation_updated(env: &Env, member: &Address, new_score: u32) {
    env.events().publish(
        (Symbol::new(env, "rep_updated"),),
        (member.clone(), new_score),
    );
}
