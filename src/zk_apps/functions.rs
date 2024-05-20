use super::models::*;
use crate::common::functions::*;
use rand::Rng;

pub fn stub_zk_apps_data(size: usize) -> Vec<Option<ZkAppData>> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            Some(ZkAppData {
                validator_name: generate_random_string(10),
                validator_pk: generate_base58_string(44),
                balance: rng.gen_range(1..=1000),
                nonce: rng.gen_range(1..=100),
                delegate: generate_base58_string(44),
            })
        })
        .collect()
}

pub fn stub_zk_app_txn_data(size: usize) -> Vec<Option<ZkAppTransactionData>> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            Some(ZkAppTransactionData {
                hash: generate_base58_string(44),
                prover: generate_base58_string(44),
                updates: rng.gen_range(1..=3),
                updated_accounts: vec![generate_base58_string(44), generate_base58_string(44)],
                fee: generate_random_mina_price(),
                date_time: generate_random_datetime_within_days(rng.gen_range(1..=25)),
            })
        })
        .collect()
}
