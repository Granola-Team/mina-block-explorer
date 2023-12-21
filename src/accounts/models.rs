use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Params, PartialEq)]
pub struct URLParams {
    pub id: Option<String>,
}

#[derive(Params, PartialEq)]
pub struct QueryParams {
    pub f: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountBalance {
    pub total: String,
}

impl AccountBalance {
    pub fn total(&self) -> f64 {
        self.total.trim().parse().expect("Cannot parse total")
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    pub public_key: String,
    pub nonce: u32,
    pub receipt_chain_hash: String,
    pub delegate: String,
    pub voting_for: String,
    pub total_tx: u32,
    pub count_pending_transactions: u32,
    pub username: String,
    pub balance: AccountBalance,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountResponse {
    pub account: AccountSummary,
}