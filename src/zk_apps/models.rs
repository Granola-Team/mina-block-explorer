#[derive(Clone)]
pub struct ZkAppData {
    pub validator_name: String,
    pub validator_pk: String,
    pub balance: usize,
    pub nonce: usize,
    pub receipt_chain_hash: String,
    pub delegate: usize,
}
