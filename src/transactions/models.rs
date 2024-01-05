use crate::common::components::*;
use super::{graphql::transactions_query::TransactionsQueryTransactions, functions::{get_block_datetime, get_from, get_receiver_public_key, get_fee, get_hash, get_amount}};

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        ["Date", "From", "To", "Hash", "Fee", "Amount"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|opt_trans| {
                match opt_trans {
                    Some(transaction) => vec![
                        get_block_datetime(transaction),
                        get_from(transaction),
                        get_receiver_public_key(transaction),
                        get_hash(transaction),
                        get_fee(transaction),
                        get_amount(transaction),
                    ],
                    None => vec![]
                }
            })
            .collect::<Vec<_>>()
    }
}

