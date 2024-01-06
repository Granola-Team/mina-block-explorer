use leptos::*;
use leptos_router::*;

use crate::common::{models::MyError, spotlight::SpotlightEntry, functions::*};

use super::models::*;

pub fn get_base_page_path(location: Location) -> String {
    let path = location.pathname.with(|path| path.clone());
    let path_parts: Vec<&str> = path.split("/accounts").collect();
    match path_parts.first() {
        Some(base) => base.to_string(),
        None => "/".to_string(),
    }
}

pub async fn load_data(id: &str) -> Result<AccountResponse, MyError> {
    let response = reqwest::get(format!("https://api.minaexplorer.com/accounts/{}", id)).await;

    match response {
        Ok(res) => match res.json::<AccountResponse>().await {
            Ok(account) => Ok(account),
            Err(_) => Err(MyError::ParseError(String::from(
                "Error deserializing JSON",
            ))),
        },
        Err(_) => Err(MyError::NetworkError(String::from("API error"))),
    }
}

pub fn get_spotlight_data(account: AccountSummary) -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            value: convert_to_span(account.balance.total),
            is_pill: true,
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            value: convert_to_span(account.nonce.to_string()),
            is_pill: true,
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            value: convert_to_span(account.receipt_chain_hash),
            is_pill: false,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            value: convert_to_span(account.delegate),
            is_pill: false,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            value: convert_to_span(account.voting_for),
            is_pill: false,
        },
    ]
}
