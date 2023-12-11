use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub blockchain_length: u64,
    pub circulating_supply: String,
    pub epoch: u16,
    pub slot: u16,
    pub total_currency: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyError {
    NetworkError(String),
    ParseError(String), // other error variants
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> Self {
        MyError::NetworkError(err.to_string())
    }
}
