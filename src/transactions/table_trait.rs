use leptos::*;

use super::{functions::*, graphql::transactions_query::TransactionsQueryTransactions};
use crate::common::{components::*, functions::*, models::PillVariant};

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
                    convert_to_span(get_block_datetime(transaction)),
                    convert_to_link(
                        get_from(transaction),
                        format!("/accounts/{}", get_from(transaction)),
                    ),
                    convert_to_link(
                        get_receiver_public_key(transaction),
                        format!("/accounts/{}", get_receiver_public_key(transaction)),
                    ),
                    convert_to_link(
                        get_hash(transaction),
                        format!("/transactions/{}", get_hash(transaction)),
                    ),
                    convert_to_pill(get_fee(transaction), PillVariant::Orange),
                    convert_to_pill(get_amount(transaction), PillVariant::Green),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
