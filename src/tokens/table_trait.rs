use super::models::TokenData;
use crate::common::{functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<TokenData> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|token| {
                vec![
                    convert_to_pill(token.name.to_string(), ColorVariant::Grey),
                    convert_to_link(token.id.to_string(), "#".to_string()),
                    decorate_with_currency_tag(token.supply.to_string(), token.name.to_string()),
                    convert_to_link(
                        token.owner.to_string(),
                        format!("/addresses/accounts/{}", token.owner),
                    ),
                    convert_to_pill(token.holders.to_string(), ColorVariant::Blue),
                    convert_to_pill(token.transactions.to_string(), ColorVariant::Blue),
                    convert_to_span(token.locked.to_string()),
                ]
            })
            .collect::<Vec<_>>()
    }
}
