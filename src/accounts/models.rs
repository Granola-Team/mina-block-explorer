use crate::common::table::*;
use serde::*;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokenSymbolResponse {
    pub data: TokensData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TokensData {
    pub tokens: Vec<TokenData>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TokenData {
    pub symbol: String,
    pub total_num_txns: u64,
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
    pub voting_for: String,
    pub public_key: String,
    pub username: String,
}

#[derive(Clone)]
pub enum AccountsSort {
    BalanceDesc,
    BalanceAsc,
}

impl SortDirection for AccountsSort {
    fn is_desc(&self) -> bool {
        matches!(self, AccountsSort::BalanceDesc)
    }
    fn is_active(&self) -> bool {
        true
    }
}

impl fmt::Display for AccountsSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountsSort::BalanceDesc => {
                write!(f, "BALANCE_DESC")
            }
            AccountsSort::BalanceAsc => {
                write!(f, "BALANCE_ASC")
            }
        }
    }
}

impl CycleSort for AccountsSort {
    fn cycle(&self) -> AnySort {
        match self {
            AccountsSort::BalanceDesc => AnySort::Accounts(AccountsSort::BalanceAsc),
            AccountsSort::BalanceAsc => AnySort::Accounts(AccountsSort::BalanceDesc),
        }
    }
}

impl TryFrom<String> for AccountsSort {
    type Error = &'static str;
    fn try_from(str: String) -> Result<AccountsSort, Self::Error> {
        match str.as_str() {
            "BALANCE_ASC" => Ok(AccountsSort::BalanceAsc),
            "BALANCE_DESC" => Ok(AccountsSort::BalanceDesc),
            _ => Err("Unable to parse the AccountsSort from string"),
        }
    }
}
