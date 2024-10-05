use crate::common::table::SortDirection;
use serde::*;
use std::fmt;

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

#[derive(Clone)]
pub enum AccountsSort {
    BalanceDesc,
}

impl SortDirection for AccountsSort {
    fn is_desc(&self) -> bool {
        matches!(self, AccountsSort::BalanceDesc)
    }
}

impl fmt::Display for AccountsSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountsSort::BalanceDesc => {
                write!(f, "BALANCE_DESC")
            }
        }
    }
}
