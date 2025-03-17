use super::graphql::tokens_query::TokensQueryTokens;
use crate::common::{constants::*, functions::*, models::*, table::TableData};
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
                                token.get_number_of_txn(),
                                format!(
                                    "/commands/user?{}={}&{}={}&{}={}",
                                    QUERY_PARAM_TOKEN,
                                    token.get_token(),
                                    QUERY_PARAM_USER_COMMAND,
                                    "false",
                                    QUERY_PARAM_TXN_APPLIED,
                                    "true"
                                ),
                            ),
                            ColorVariant::Blue,
                        ),
                        convert_to_span(token.get_percent_unlocked().unwrap()),
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
    fn get_number_of_txn(&self) -> String;
    fn get_percent_unlocked(&self) -> Result<String, String>;
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
    fn get_number_of_txn(&self) -> String {
        format_number(self.total_num_txns.to_string())
    }
    fn get_percent_unlocked(&self) -> Result<String, String> {
        if self.total_num_tokens == 0 {
            return Err("n/a".to_string());
        }

        // Convert to f64 for floating-point division
        let locked = self.total_num_locked as f64;
        let total = self.total_num_tokens as f64;
        let percent_locked = locked / total;
        let percent_unlocked = 1.0 - percent_locked;

        // Format as a percentage string (e.g., "75.00%")
        Ok(format!("{:.2}%", percent_unlocked * 100.0))
    }
}
