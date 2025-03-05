use super::models::TokenData;
use crate::common::{constants::QUERY_PARAM_TOKEN, functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<TokenData> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|token| {
                vec![
                    convert_to_pill(token.name.to_string(), ColorVariant::Grey),
                    convert_to_span(token.supply.to_string()),
                    convert_to_copy_link(token.id.to_string(), "#".to_string()),
                    convert_to_copy_link(
                        token.owner.to_string(),
                        format!("/addresses/accounts/{}", token.owner),
                    ),
                    wrap_in_pill(
                        convert_to_link(
                            token.holders.to_string(),
                            format!("/addresses/accounts?{}={}", QUERY_PARAM_TOKEN, token.id),
                        ),
                        ColorVariant::Blue,
                    ),
                    wrap_in_pill(
                        convert_to_link(
                            token.transactions.to_string(),
                            format!("/commands/user?{}={}", QUERY_PARAM_TOKEN, token.id),
                        ),
                        ColorVariant::Blue,
                    ),
                    convert_to_span(token.unlock_percentage.to_string()),
                ]
            })
            .collect::<Vec<_>>()
    }
}
