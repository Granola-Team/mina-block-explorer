use super::graphql::account_activity_query::{
    AccountActivityQueryIncomingTransactions, AccountActivityQueryOutgoingTransactions,
};
use crate::{
    Params,
    account_activity::graphql::account_activity_query::StakesQueryInput,
    common::{functions::*, table::SortDirection},
};
use chrono::{DateTime, Utc};
use heck::ToTitleCase;
use leptos_router::Params;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

#[derive(Copy, Clone)]
pub struct DelegateCount(pub usize);

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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AccountActivityQueryDirectionalTransactions {
    pub fee: Option<f64>,
    pub counterparty: Option<String>,
    pub counterparty_username: Option<String>,
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
                i.to.clone()
            } else {
                i.from.clone()
            },
            counterparty_username: if i.from == i.to {
                i.receiver_account
                    .as_ref()
                    .and_then(|ra| ra.username.clone())
            } else {
                i.sender_username
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
                i.from.clone()
            } else {
                i.to.clone()
            },
            counterparty_username: if i.from == i.to {
                i.sender_username
            } else {
                i.receiver_account
                    .as_ref()
                    .and_then(|ra| ra.username.clone())
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

pub fn merge_transactions(
    incoming: Vec<Option<AccountActivityQueryIncomingTransactions>>,
    outgoing: Vec<Option<AccountActivityQueryOutgoingTransactions>>,
) -> Vec<Option<AccountActivityQueryDirectionalTransactions>> {
    let mut transactions: Vec<Option<AccountActivityQueryDirectionalTransactions>> = incoming
        .into_iter()
        .map(|t| t.map(AccountActivityQueryDirectionalTransactions::from))
        .chain(
            outgoing
                .into_iter()
                .map(|t| t.map(AccountActivityQueryDirectionalTransactions::from)),
        )
        .collect();

    transactions.sort_by(|a, b| {
        match (
            a.as_ref().and_then(|x| x.date_time),
            b.as_ref().and_then(|x| x.date_time),
        ) {
            (Some(date_time_a), Some(date_time_b)) => date_time_b.cmp(&date_time_a),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    });

    transactions
}

pub trait AccountActivityQueryDirectionalTransactionTrait {
    fn get_fee(&self) -> String;
    fn get_counterparty(&self) -> String;
    fn get_counterparty_username(&self) -> String;
    fn get_direction(&self) -> String;
    fn get_hash(&self) -> String;
    fn get_amount(&self) -> String;
    fn get_date_time(&self) -> String;
    fn get_height(&self) -> String;
    fn get_kind(&self) -> String;
    fn get_nonce(&self) -> String;
    fn _get_failure_reason(&self) -> String;
    fn get_memo(&self) -> String;
    fn _get_canonical(&self) -> bool;
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
    fn get_counterparty_username(&self) -> String {
        self.counterparty_username
            .as_ref()
            .map_or(String::from("..."), |f| f.to_string())
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

    fn _get_failure_reason(&self) -> String {
        self.failure_reason
            .as_ref()
            .map_or(String::new(), |f| f.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    }

    fn _get_canonical(&self) -> bool {
        self.canonical.unwrap_or_default()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for StakesQueryInput {
    fn default() -> Self {
        StakesQueryInput {
            genesis_state_hash: None,
            stake_lte: None,
            timing_exists: None,
            chain_id_gte: None,
            balance_ne: None,
            nonce: None,
            delegate_gte: None,
            epoch_nin: None,
            public_key_gt: None,
            ledger_hash_nin: None,
            ledger_hash_lt: None,
            chain_id_in: None,
            delegate_nin: None,
            token_nin: None,
            token_lt: None,
            pk_gte: None,
            balance_gt: None,
            ledger_hash_lte: None,
            epoch_in: None,
            ledger_hash_gt: None,
            delegate_gt: None,
            pk_nin: None,
            balance_lte: None,
            public_key_nin: None,
            ledger_hash_ne: None,
            voting_for: None,
            nonce_in: None,
            permissions_exists: None,
            permissions: None,
            chain_id_nin: None,
            token_gte: None,
            chain_id_gt: None,
            nonce_lt: None,
            pk_gt: None,
            voting_for_lte: None,
            receipt_chain_hash_in: None,
            public_key_gte: None,
            ledger_hash: None,
            chain_id_lte: None,
            ledger_hash_in: None,
            receipt_chain_hash_lt: None,
            voting_for_gt: None,
            balance_lt: None,
            token_ne: None,
            pk_in: None,
            balance_exists: None,
            voting_for_gte: None,
            delegate: None,
            or: None,
            balance_nin: None,
            epoch_ne: None,
            nonce_gte: None,
            and: None,
            voting_for_exists: None,
            public_key_exists: None,
            token_in: None,
            receipt_chain_hash_lte: None,
            epoch_lte: None,
            balance_gte: None,
            voting_for_lt: None,
            chain_id: None,
            delegate_ne: None,
            public_key_in: None,
            delegate_lte: None,
            pk_lte: None,
            receipt_chain_hash_ne: None,
            voting_for_in: None,
            public_key_lte: None,
            receipt_chain_hash: None,
            voting_for_ne: None,
            token: None,
            public_key: None,
            balance_in: None,
            public_key_lt: None,
            chain_id_exists: None,
            token_lte: None,
            delegate_exists: None,
            balance: None,
            receipt_chain_hash_gte: None,
            epoch_exists: None,
            nonce_nin: None,
            chain_id_ne: None,
            epoch_gte: None,
            epoch_gt: None,
            voting_for_nin: None,
            pk: None,
            pk_ne: None,
            nonce_exists: None,
            receipt_chain_hash_exists: None,
            token_exists: None,
            epoch: None,
            receipt_chain_hash_gt: None,
            chain_id_lt: None,
            token_gt: None,
            nonce_gt: None,
            receipt_chain_hash_nin: None,
            timing: None,
            pk_lt: None,
            public_key_ne: None,
            nonce_ne: None,
            epoch_lt: None,
            nonce_lte: None,
            delegate_in: None,
            ledger_hash_gte: None,
            ledger_hash_exists: None,
            pk_exists: None,
            delegate_lt: None,
        }
    }
}

#[derive(Clone)]
pub struct AccountActivityQueryDelegatorExt {
    pub username: Option<String>,
    #[allow(dead_code)]
    pub epoch: Option<i64>,
    pub public_key: Option<String>,
    pub delegated_balance: Option<i64>,
    pub percent_of_delegation: Option<f64>,
}

#[derive(Clone)]
pub enum Delegators {
    BalanceDesc,
}

impl SortDirection for Delegators {
    fn is_desc(&self) -> bool {
        matches!(self, Delegators::BalanceDesc)
    }
    fn is_active(&self) -> bool {
        true
    }
}

impl fmt::Display for Delegators {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Delegators::BalanceDesc => {
                write!(f, "BALANCE_DESC")
            }
        }
    }
}

#[cfg(test)]
mod merge_tests {
    use super::*;
    use crate::account_activity::graphql::account_activity_query::{
        AccountActivityQueryIncomingTransactionsBlock,
        AccountActivityQueryOutgoingTransactionsBlock,
    };
    use chrono::{DateTime, Utc};

    #[test]
    fn test_merge_transactions() {
        // Helper to parse dates
        let parse_date = |s: &str| DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc);

        // Test data (only Some transactions with date_time)
        let incoming = vec![
            Some(AccountActivityQueryIncomingTransactions {
                fee: Some(0.1),
                from: Some("alice".to_string()),
                to: Some("bob".to_string()),
                hash: Some("in1".to_string()),
                amount: Some(10.0),
                block: Some(AccountActivityQueryIncomingTransactionsBlock {
                    date_time: Some(parse_date("2023-10-01T10:00:00Z")),
                }),
                block_height: Some(100),
                kind: Some("payment".to_string()),
                nonce: Some(1),
                failure_reason: None,
                memo: Some("test".to_string()),
                canonical: Some(true),
                ..Default::default()
            }),
            Some(AccountActivityQueryIncomingTransactions {
                fee: Some(0.2),
                from: Some("alice".to_string()),
                to: Some("alice".to_string()), // Self-transfer
                hash: Some("in2".to_string()),
                amount: Some(5.0),
                block: Some(AccountActivityQueryIncomingTransactionsBlock {
                    date_time: Some(parse_date("2023-10-02T12:00:00Z")),
                }),
                block_height: Some(101),
                kind: Some("payment".to_string()),
                nonce: Some(2),
                failure_reason: None,
                memo: None,
                canonical: Some(true),
                ..Default::default()
            }),
        ];

        let outgoing = vec![Some(AccountActivityQueryOutgoingTransactions {
            fee: Some(0.3),
            from: Some("bob".to_string()),
            to: Some("charlie".to_string()),
            hash: Some("out1".to_string()),
            amount: Some(15.0),
            block: Some(AccountActivityQueryOutgoingTransactionsBlock {
                date_time: Some(parse_date("2023-10-01T09:00:00Z")),
            }),
            block_height: Some(99),
            kind: Some("payment".to_string()),
            nonce: Some(3),
            failure_reason: None,
            memo: Some("outgoing".to_string()),
            canonical: Some(true),
            ..Default::default()
        })];

        // Run the function
        let result = merge_transactions(incoming, outgoing);

        // Expected output (sorted by date_time descending, only Some transactions)
        let expected = vec![
            // Latest: 2023-10-02T12:00:00Z (in2)
            Some(AccountActivityQueryDirectionalTransactions {
                fee: Some(0.2),
                counterparty: Some("alice".to_string()),
                direction: Some("IN".to_string()),
                hash: Some("in2".to_string()),
                amount: Some(5.0),
                date_time: Some(parse_date("2023-10-02T12:00:00Z")),
                height: Some(101),
                kind: Some("payment".to_string()),
                nonce: Some(2),
                failure_reason: None,
                memo: None,
                canonical: Some(true),
                ..Default::default()
            }),
            // 2023-10-01T10:00:00Z (in1)
            Some(AccountActivityQueryDirectionalTransactions {
                fee: Some(0.1),
                counterparty: Some("alice".to_string()),
                direction: Some("IN".to_string()),
                hash: Some("in1".to_string()),
                amount: Some(10.0),
                date_time: Some(parse_date("2023-10-01T10:00:00Z")),
                height: Some(100),
                kind: Some("payment".to_string()),
                nonce: Some(1),
                failure_reason: None,
                memo: Some("test".to_string()),
                canonical: Some(true),
                ..Default::default()
            }),
            // 2023-10-01T09:00:00Z (out1)
            Some(AccountActivityQueryDirectionalTransactions {
                fee: Some(0.3),
                counterparty: Some("charlie".to_string()),
                direction: Some("OUT".to_string()),
                hash: Some("out1".to_string()),
                amount: Some(15.0),
                date_time: Some(parse_date("2023-10-01T09:00:00Z")),
                height: Some(99),
                kind: Some("payment".to_string()),
                nonce: Some(3),
                failure_reason: None,
                memo: Some("outgoing".to_string()),
                canonical: Some(true),
                ..Default::default()
            }),
        ];

        // Assert the result matches expected
        assert_eq!(
            result, expected,
            "Merged transactions do not match expected output"
        );

        // Verify length
        assert_eq!(result.len(), 3, "Unexpected number of transactions");

        // Verify sorting (check date_time order)
        let date_times: Vec<Option<DateTime<Utc>>> = result
            .iter()
            .map(|t| t.as_ref().and_then(|x| x.date_time))
            .collect();
        assert_eq!(
            date_times,
            vec![
                Some(parse_date("2023-10-02T12:00:00Z")),
                Some(parse_date("2023-10-01T10:00:00Z")),
                Some(parse_date("2023-10-01T09:00:00Z")),
            ],
            "Transactions are not sorted correctly by date_time"
        );

        // Verify directions
        let directions: Vec<Option<String>> = result
            .iter()
            .map(|t| t.as_ref().and_then(|x| x.direction.clone()))
            .collect();
        assert_eq!(
            directions,
            vec![
                Some("IN".to_string()),
                Some("IN".to_string()),
                Some("OUT".to_string()),
            ],
            "Transaction directions are incorrect"
        );
    }

    #[test]
    fn test_merge_transactions_empty() {
        // Test with empty inputs
        let incoming: Vec<Option<AccountActivityQueryIncomingTransactions>> = vec![];
        let outgoing: Vec<Option<AccountActivityQueryOutgoingTransactions>> = vec![];
        let result = merge_transactions(incoming, outgoing);
        assert!(
            result.is_empty(),
            "Empty inputs should produce empty output"
        );
    }

    // Optionally keep or remove this test
    #[test]
    fn test_merge_transactions_only_none() {
        // Test with only None values
        let incoming: Vec<Option<AccountActivityQueryIncomingTransactions>> = vec![None, None];
        let outgoing: Vec<Option<AccountActivityQueryOutgoingTransactions>> = vec![None];
        let result = merge_transactions(incoming, outgoing);
        assert_eq!(
            result,
            vec![None, None, None],
            "None inputs should produce None outputs"
        );
    }
}
