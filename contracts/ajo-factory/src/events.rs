//! AjoChain Factory — Event emissions

use soroban_sdk::{Address, Env, Symbol};

pub fn factory_initialized(env: &Env, admin: &Address, fee_bps: u32) {
    env.events().publish(
        (Symbol::new(env, "factory_init"),),
        (admin.clone(), fee_bps),
    );
}

pub fn pool_registered(env: &Env, pool_index: u64, pool_address: &Address, creator: &Address) {
    env.events().publish(
        (Symbol::new(env, "pool_registered"), pool_index),
        (pool_address.clone(), creator.clone()),
    );
}

pub fn token_allowed(env: &Env, token: &Address) {
    env.events().publish(
        (Symbol::new(env, "token_allowed"),),
        token.clone(),
    );
}

pub fn token_removed(env: &Env, token: &Address) {
    env.events().publish(
        (Symbol::new(env, "token_removed"),),
        token.clone(),
    );
}

pub fn fee_updated(env: &Env, new_fee_bps: u32) {
    env.events().publish(
        (Symbol::new(env, "fee_updated"),),
        new_fee_bps,
    );
}
