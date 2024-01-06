use super::{
    functions::{
        get_amount, get_block_datetime, get_fee, get_from, get_hash, get_receiver_public_key,
    },
    graphql::transactions_query::TransactionsQueryTransactions,
};
use crate::common::components::*;

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        ["Date", "From", "To", "Hash", "Fee", "Amount"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    get_block_datetime(transaction),
                    get_from(transaction),
                    get_receiver_public_key(transaction),
                    get_hash(transaction),
                    get_fee(transaction),
                    get_amount(transaction),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
