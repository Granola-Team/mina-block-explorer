use super::models::TokenData;
use crate::common::{functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<Option<TokenData>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Token Name",
            "Token ID",
            "Supply",
            "Token Owner",
            "Token Holders",
            "Transaction Count",
            "Locked",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_token| match opt_token {
                Some(token) => vec![
                    convert_to_pill(token.token_symbol.to_string(), ColorVariant::Grey),
                    convert_to_link(token.token_id.to_string(), "#".to_string()),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            token.token_balance.to_string(),
                            token.token_symbol.to_string(),
                        ),
                        ColorVariant::Green,
                    ),
                    convert_to_link(token.owner_pk.to_string(), "#".to_string()),
                    convert_to_pill(token.token_holders_count.to_string(), ColorVariant::Blue),
                    convert_to_pill(token.trx_count.to_string(), ColorVariant::Blue),
                    convert_to_span(token.locked.to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
