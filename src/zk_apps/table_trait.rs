use super::models::*;
use crate::common::{functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<Option<ZkAppData>> {
    fn get_columns(&self) -> Vec<String> {
        ["Account", "Balance", "Commands", "Delegate"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_app| match opt_app {
                Some(zk_app) => vec![
                    convert_to_link(zk_app.validator_pk.to_string(), "#".to_string()),
                    decorate_with_currency_tag(zk_app.balance.to_string(), "mina".to_string()),
                    convert_to_pill(zk_app.nonce.to_string(), ColorVariant::Blue),
                    convert_to_link(zk_app.delegate.to_string(), "#".to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<ZkAppTransactionData>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Prover",
            "txn Hash",
            "Age",
            "Account Updates",
            "Updated Accounts",
            "Fee",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_txn| match opt_txn {
                Some(txn) => vec![
                    convert_to_link(txn.prover.to_string(), "#".to_string()),
                    convert_to_link(txn.hash.to_string(), "#".to_string()),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&txn.date_time.to_string())),
                        convert_to_span(txn.date_time.to_string())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(txn.updated_accounts.len().to_string(), ColorVariant::Blue),
                    convert_array_to_span(
                        txn.updated_accounts
                            .iter()
                            .map(|ua| convert_to_link(ua.to_string(), "#".to_string()))
                            .collect::<Vec<_>>(),
                    )
                    .attr("class", "block"),
                    wrap_in_pill(
                        decorate_with_currency_tag(txn.fee.to_string(), "mina".to_string()),
                        ColorVariant::Orange,
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
