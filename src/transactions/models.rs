use leptos::*;

use super::{functions::*, graphql::transactions_query::TransactionsQueryTransactions};
use crate::common::{components::*, functions::convert_to_span};

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        ["Date", "From", "To", "Hash", "Fee", "Amount"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    get_block_datetime(transaction),
                    get_from(transaction),
                    get_receiver_public_key(transaction),
                    get_hash(transaction),
                    get_fee(transaction),
                    get_amount(transaction),
                ]
                .into_iter()
                .map(convert_to_span)
                .collect(),
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
