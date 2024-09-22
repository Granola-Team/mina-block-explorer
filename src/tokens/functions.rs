use super::models::*;
use crate::common::functions::generate_base58_string;
use rand::Rng;

fn generate_random_string(len: usize) -> String {
    let charset = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn stub_token_data(size: u64) -> Vec<Option<TokenData>> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            Some(TokenData {
                token_id: generate_random_string(10),
                locked: rng.gen_bool(0.5), // 50% chance to be true or false
                owner_pk: generate_base58_string(44),
                owner_token_id: generate_base58_string(10),
                token_symbol: generate_random_string(5), // Shorter string for symbol
                token_holders_count: rng.gen_range(1..=1000),
                token_balance: rng.gen_range(1..=1000),
                txn_count: rng.gen_range(1..=1000),
                unlock_percent: rng.gen_range(1..=100),
            })
        })
        .collect()
}
