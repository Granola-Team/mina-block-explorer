use super::models::*;
use crate::common::{functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<Option<ZkAppData>> {
    fn get_columns(&self) -> Vec<String> {
        ["Account", "Balance", "Outgoing Transactions", "Delegate"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_app| match opt_app {
                Some(zk_app) => vec![
                    convert_to_link(zk_app.validator_pk.to_string(), "#".to_string()),
                    wrap_in_pill(
                        decorate_with_currency_tag(zk_app.balance.to_string(), "mina".to_string()),
                        ColorVariant::Green,
                    ),
                    convert_to_pill(
                        zk_app.nonce.to_string(),
                        ColorVariant::Blue,
                    ),
                    convert_to_link(zk_app.delegate.to_string(), "#".to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
