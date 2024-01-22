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
    pub receipt_chain_hash: String,
    pub voting_for: String,
    pub public_key: String,
    pub username: String,
}

pub enum AccountCardVariant {
    Purple,
    Blue,
    Green,
}
