#[derive(Clone)]
pub struct ZkAppData {
    pub validator_name: String,
    pub validator_pk: String,
    pub balance: usize,
    pub nonce: usize,
    pub delegate: String,
}

#[derive(Clone)]
pub struct ZkAppTransactionData {
    pub hash: String,
    pub prover: String,
    pub updates: usize,
    pub updated_accounts: Vec<String>,
    pub fee: f64,
    pub date_time: chrono::DateTime<chrono::Utc>,
}
