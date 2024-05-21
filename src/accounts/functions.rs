use super::models::*;
use crate::common::functions::*;
use rand::{distributions::Uniform, prelude::Distribution};

pub fn stub_account_summaries(size: i64) -> Vec<Option<AllAccountSummary>> {
    let mut rng = rand::thread_rng();
    let int_dist = Uniform::from(0..=1000);

    (0..size)
        .map(|_| {
            let balance = generate_random_mina_price();

            Some(AllAccountSummary {
                pk: generate_base58_string(44),
                balance,
                delegate: generate_base58_string(44),
                token: int_dist.sample(&mut rng),
                nonce: int_dist.sample(&mut rng),
                voting_for: generate_base58_string(44),
                public_key: generate_base58_string(44),
                username: generate_random_string(10),
            })
        })
        .collect()
}
