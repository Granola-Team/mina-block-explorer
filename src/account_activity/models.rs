use super::graphql::account_activity_query::{
    AccountActivityQueryIncomingTransactions, AccountActivityQueryOutgoingTransactions,
};
use crate::{common::functions::*, Params};
use chrono::{DateTime, Utc};
use heck::ToTitleCase;
use leptos_router::Params;
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

#[derive(Clone)]
pub struct AccountActivityQueryDirectionalTransactions {
    pub fee: Option<f64>,
    pub counterparty: Option<String>,
    pub direction: Option<String>,
    pub hash: Option<String>,
    pub amount: Option<f64>,
    pub date_time: Option<DateTime<Utc>>,
    pub height: Option<u64>,
    pub kind: Option<String>,
    pub nonce: Option<u64>,
    pub failure_reason: Option<String>,
    pub memo: Option<String>,
    pub canonical: Option<bool>,
}

impl From<AccountActivityQueryIncomingTransactions>
    for AccountActivityQueryDirectionalTransactions
{
    fn from(
        i: AccountActivityQueryIncomingTransactions,
    ) -> AccountActivityQueryDirectionalTransactions {
        AccountActivityQueryDirectionalTransactions {
            fee: i.fee,
            counterparty: if i.from == i.to {
                Some("Self".to_string())
            } else {
                i.from
            },
            direction: Some("IN".to_string()),
            hash: i.hash,
            amount: i.amount,
            date_time: if let Some(block) = i.block {
                block.date_time
            } else {
                None
            },
            height: Some(i.block_height.unwrap_or_default() as u64),
            kind: i.kind,
            nonce: Some(i.nonce.unwrap_or_default() as u64),
            failure_reason: i.failure_reason,
            memo: i.memo,
            canonical: i.canonical,
        }
    }
}

impl From<AccountActivityQueryOutgoingTransactions>
    for AccountActivityQueryDirectionalTransactions
{
    fn from(
        i: AccountActivityQueryOutgoingTransactions,
    ) -> AccountActivityQueryDirectionalTransactions {
        AccountActivityQueryDirectionalTransactions {
            fee: i.fee,
            counterparty: if i.from == i.to {
                Some("Self".to_string())
            } else {
                i.to
            },
            direction: Some("OUT".to_string()),
            hash: i.hash,
            amount: i.amount,
            date_time: if let Some(block) = i.block {
                block.date_time
            } else {
                None
            },
            height: Some(i.block_height.unwrap_or_default() as u64),
            kind: i.kind,
            nonce: Some(i.nonce.unwrap_or_default() as u64),
            failure_reason: i.failure_reason,
            memo: i.memo,
            canonical: i.canonical,
        }
    }
}

pub trait AccountActivityQueryDirectionalTransactionTrait {
    fn get_fee(&self) -> String;
    fn get_counterparty(&self) -> String;
    fn get_direction(&self) -> String;
    fn get_hash(&self) -> String;
    fn get_amount(&self) -> String;
    fn get_date_time(&self) -> String;
    fn get_height(&self) -> String;
    fn get_kind(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_failure_reason(&self) -> String;
    fn get_memo(&self) -> String;
    fn get_canonical(&self) -> bool;
}

impl AccountActivityQueryDirectionalTransactionTrait
    for AccountActivityQueryDirectionalTransactions
{
    fn get_fee(&self) -> String {
        self.fee
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map_or(String::new(), |f| format_mina(f.to_string()))
    }
    fn get_counterparty(&self) -> String {
        self.counterparty
            .as_ref()
            .map_or(String::new(), |f| f.to_string())
    }
    fn get_direction(&self) -> String {
        self.direction
            .as_ref()
            .map_or(String::new(), |f| f.to_string())
    }
    fn get_hash(&self) -> String {
        self.hash.as_ref().map_or(String::new(), |f| f.to_string())
    }
    fn get_amount(&self) -> String {
        self.amount
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map_or(String::new(), |f| f.to_string())
    }
    fn get_date_time(&self) -> String {
        self.date_time.map_or(String::new(), |f| f.to_string())
    }

    fn get_height(&self) -> String {
        self.height
            .map_or(String::new(), |f| format_number(f.to_string()))
    }

    fn get_kind(&self) -> String {
        self.kind.as_ref().map_or(String::new(), |o| {
            ToTitleCase::to_title_case(o.as_str()).to_string()
        })
    }

    fn get_nonce(&self) -> String {
        self.nonce
            .map_or(String::new(), |f| format_number(f.to_string()))
    }

    fn get_failure_reason(&self) -> String {
        self.failure_reason
            .as_ref()
            .map_or(String::new(), |f| f.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    }

    fn get_canonical(&self) -> bool {
        self.canonical.unwrap_or_default()
    }
}
