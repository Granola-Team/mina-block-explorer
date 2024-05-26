use serde::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllAccountResponse {
    pub data: Vec<AllAccountSummary>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllAccountSummary {
    pub pk: String,
    pub balance: f64,
    pub delegate: String,
    pub token: i32,
    pub nonce: i32,
    pub voting_for: String,
    pub public_key: String,
    pub username: String,
}
