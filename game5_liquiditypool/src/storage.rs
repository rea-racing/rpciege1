use soroban_sdk::{Address, Env};

use crate::types::DataKey;

pub fn has_token_a(env: &Env) -> bool {
    env.storage().has(&DataKey::TokenA)
}

pub fn read_token_a(env: &Env) -> Address {
    env.storage().get(&DataKey::TokenA).unwrap().unwrap()
}

pub fn read_token_b(env: &Env) -> Address {
    env.storage().get(&DataKey::TokenB).unwrap().unwrap()
}

pub fn write_token_a(env: &Env, a: Address) {
    env.storage().set(&DataKey::TokenA, &a)
}

pub fn write_token_b(env: &Env, b: Address) {
    env.storage().set(&DataKey::TokenB, &b)
}

pub fn read_reserve_a(env: &Env) -> i128 {
    env.storage()
        .get(&DataKey::ReserveA)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn read_reserve_b(env: &Env) -> i128 {
    env.storage()
        .get(&DataKey::ReserveB)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn write_reserve_a(env: &Env, reserve: i128) {
    env.storage().set(&DataKey::ReserveA, &reserve)
}

pub fn write_reserve_b(env: &Env, reserve: i128) {
    env.storage().set(&DataKey::ReserveB, &reserve)
}