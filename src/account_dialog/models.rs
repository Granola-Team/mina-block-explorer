use crate::{
    account_dialog::graphql::account_activity_query::{
        AccountActivityQueryIncomingTransactions, AccountActivityQueryOutgoingTransactions,
    },
    common::functions::nanomina_to_mina,
};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AccountActivityQueryDirectionalTransactions {
    pub fee: Option<f64>,
    pub counterparty: Option<String>,
    pub direction: Option<String>,
    pub hash: Option<String>,
    pub amount: Option<f64>,
    pub date_time: Option<DateTime<Utc>>,
    pub height: Option<i64>,
    pub kind: Option<String>,
    pub nonce: Option<i64>,
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
            height: i.block_height,
            kind: i.kind,
            nonce: i.nonce,
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
            height: i.block_height,
            kind: i.kind,
            nonce: i.nonce,
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
            .map(nanomina_to_mina)
            .map_or(String::new(), |f| f.to_string())
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
            .map(nanomina_to_mina)
            .map_or(String::new(), |f| f.to_string())
    }
    fn get_date_time(&self) -> String {
        self.date_time.map_or(String::new(), |f| f.to_string())
    }

    fn get_height(&self) -> String {
        self.height.map_or(String::new(), |f| f.to_string())
    }

    fn get_kind(&self) -> String {
        self.kind.as_ref().map_or(String::new(), |f| f.to_string())
    }

    fn get_nonce(&self) -> String {
        self.nonce.map_or(String::new(), |f| f.to_string())
    }

    fn get_failure_reason(&self) -> String {
        self.failure_reason
            .as_ref()
            .map_or(String::new(), |f| f.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo.as_ref().map_or(String::new(), |f| f.to_string())
    }

    fn get_canonical(&self) -> bool {
        self.canonical.unwrap_or_default()
    }
}
