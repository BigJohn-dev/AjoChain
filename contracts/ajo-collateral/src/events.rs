//! AjoChain Collateral — Event emissions

use soroban_sdk::{Address, Env, Symbol};

pub fn vault_initialized(env: &Env, admin: &Address, token: &Address, required: i128) {
    env.events().publish(
        (Symbol::new(env, "vault_init"),),
        (admin.clone(), token.clone(), required),
    );
}

pub fn collateral_deposited(env: &Env, member: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "collateral_dep"),),
        (member.clone(), amount),
    );
}

pub fn collateral_slashed(env: &Env, member: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "collateral_slash"),),
        (member.clone(), amount),
    );
}

pub fn collateral_released(env: &Env, member: &Address, amount: i128) {
    env.events().publish(
        (Symbol::new(env, "collateral_rel"),),
        (member.clone(), amount),
    );
}
