use super::graphql::tokens_query::TokensQueryTokens;
use crate::common::{constants::QUERY_PARAM_TOKEN, functions::*, models::*, table::TableData};
use leptos::{html, HtmlElement};

impl TableData for Vec<Option<TokensQueryTokens>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|token| {
                if let Some(token) = token {
                    vec![
                        convert_to_pill(token.get_symbol(), ColorVariant::Grey),
                        convert_to_span(token.get_supply()),
                        convert_to_copy_link(token.get_token(), "#".to_string()),
                        convert_to_copy_link(
                            token.get_owner(),
                            format!("/addresses/accounts/{}", token.get_owner()),
                        ),
                        wrap_in_pill(
                            convert_to_link(
                                token.get_number_of_holders(),
                                format!(
                                    "/addresses/accounts?{}={}",
                                    QUERY_PARAM_TOKEN,
                                    token.get_token()
                                ),
                            ),
                            ColorVariant::Blue,
                        ),
                        wrap_in_pill(
                            convert_to_link(
                                "0".to_string(),
                                format!(
                                    "/commands/user?{}={}",
                                    QUERY_PARAM_TOKEN,
                                    token.get_token()
                                ),
                            ),
                            ColorVariant::Blue,
                        ),
                        convert_to_span("0".to_string()),
                    ]
                } else {
                    vec![]
                }
            })
            .collect::<Vec<_>>()
    }
}

pub trait TokensTrait {
    fn get_token(&self) -> String;
    fn get_owner(&self) -> String;
    fn get_symbol(&self) -> String;
    fn get_supply(&self) -> String;
    fn get_number_of_holders(&self) -> String;
}

impl TokensTrait for TokensQueryTokens {
    fn get_token(&self) -> String {
        self.token.to_string()
    }
    fn get_owner(&self) -> String {
        self.owner.as_ref().cloned().unwrap_or_default().to_string()
    }
    fn get_symbol(&self) -> String {
        self.symbol
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .to_string()
    }
    fn get_supply(&self) -> String {
        format_number(self.supply.to_string())
    }
    fn get_number_of_holders(&self) -> String {
        format_number(self.num_holders.to_string())
    }
}
