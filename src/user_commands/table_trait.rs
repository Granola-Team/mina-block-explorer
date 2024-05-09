use super::{functions::*, graphql::transactions_query::TransactionsQueryTransactions};
use crate::common::{functions::*, models::ColorVariant, table::*};
use leptos::*;

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Height", "Txn Hash", "Age", "Type", "From", "To", "Nonce", "Fee", "Amount",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "Txn Hash", "From", "To"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(
                            transaction.get_canonical(),
                            transaction.get_failure_reason(),
                        ),
                        convert_to_span(transaction.get_block_height()),
                    ]),
                    convert_to_link(
                        transaction.get_hash(),
                        format!("/commands/{}", transaction.get_hash()),
                    ),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&transaction.get_block_datetime())),
                        convert_to_span(transaction.get_block_datetime())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_link(
                                transaction.get_from(),
                                format!("/addresses/accounts/{}", transaction.get_from()),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_link(
                            transaction.get_from(),
                            format!("/addresses/accounts/{}", transaction.get_from()),
                        )
                    },
                    convert_to_link(
                        transaction.get_receiver_public_key(),
                        format!(
                            "/addresses/accounts/{}",
                            transaction.get_receiver_public_key()
                        ),
                    ),
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    decorate_with_currency_tag(transaction.get_fee(), "mina".to_string()),
                    decorate_with_currency_tag(transaction.get_amount(), "mina".to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait TransactionsTrait {
    fn get_failure_reason(&self) -> Option<String>;
    fn get_block_datetime(&self) -> String;
    fn get_block_height(&self) -> String;
    fn get_canonical(&self) -> Option<bool>;
    fn get_kind(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_memo(&self) -> String;
    fn get_block_state_hash(&self) -> String;
    fn get_from(&self) -> String;
    fn get_receiver_public_key(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_hash(&self) -> String;
    fn get_amount(&self) -> String;
    fn get_to(&self) -> String;
}

impl TransactionsTrait for TransactionsQueryTransactions {
    fn get_failure_reason(&self) -> Option<String> {
        self.failure_reason.clone()
    }

    fn get_block_datetime(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.date_time)
            .map_or_else(String::new, |o1| o1.to_string())
    }

    fn get_block_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_canonical(&self) -> Option<bool> {
        self.canonical
    }

    fn get_kind(&self) -> String {
        self.kind
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_nonce(&self) -> String {
        self.nonce.map_or_else(String::new, |o| o.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, |o| decode_memo(o).unwrap_or("".to_string()))
    }

    fn get_block_state_hash(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.state_hash.as_ref())
            .map_or_else(String::new, |o1| o1.to_string())
    }

    fn get_from(&self) -> String {
        self.from
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_receiver_public_key(&self) -> String {
        self.receiver.as_ref().map_or_else(String::new, |o| {
            o.public_key
                .as_ref()
                .map_or_else(String::new, |o| o.to_string())
        })
    }

    fn get_fee(&self) -> String {
        self.fee
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .unwrap_or_default()
    }

    fn get_hash(&self) -> String {
        self.hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        self.amount
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .unwrap_or_default()
    }

    fn get_to(&self) -> String {
        self.to.as_ref().map_or_else(String::new, |o| o.to_string())
    }
}
