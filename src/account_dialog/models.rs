use crate::{account_dialog::graphql::account_activity_query::{
    AccountActivityQueryIncomingTransactions, AccountActivityQueryOutgoingTransactions,
}, common::functions::nanomina_to_mina};
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AccountActivityQueryDirectionalTransactions {
    pub fee: Option<f64>,
    pub counterparty: Option<String>,
    pub direction: Option<String>,
    pub hash: Option<String>,
    pub amount: Option<f64>,
    pub date_time: Option<DateTime<Utc>>,
}

impl From<AccountActivityQueryIncomingTransactions>
    for AccountActivityQueryDirectionalTransactions
{
    fn from(
        i: AccountActivityQueryIncomingTransactions,
    ) -> AccountActivityQueryDirectionalTransactions {
        AccountActivityQueryDirectionalTransactions {
            fee: i.fee,
            counterparty: i.from,
            direction: Some("IN".to_string()),
            hash: i.hash,
            amount: i.amount,
            date_time: if let Some(block) = i.block {
                block.date_time
            } else {
                None
            },
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
            counterparty: i.to,
            direction: Some("OUT".to_string()),
            hash: i.hash,
            amount: i.amount,
            date_time: if let Some(block) = i.block {
                block.date_time
            } else {
                None
            },
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
}

impl AccountActivityQueryDirectionalTransactionTrait
    for AccountActivityQueryDirectionalTransactions
{
    fn get_fee(&self) -> String {
        self.fee.map(nanomina_to_mina).map_or(String::new(), |f| f.to_string())
    }
    fn get_counterparty(&self) -> String {
        self.counterparty.as_ref().map_or(String::new(), |f| f.to_string())
    }
    fn get_direction(&self) -> String {
        self.direction.as_ref().map_or(String::new(), |f| f.to_string())
    }
    fn get_hash(&self) -> String {
        self.hash.as_ref().map_or(String::new(), |f| f.to_string())
    }
    fn get_amount(&self) -> String {
        self.amount.map(nanomina_to_mina).map_or(String::new(), |f| f.to_string())
    }
    fn get_date_time(&self) -> String {
        self.date_time.map_or(String::new(), |f| f.to_string())
    }
}
