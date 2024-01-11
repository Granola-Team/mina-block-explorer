use leptos::*;
use leptos_router::*;

use crate::common::{models::*, spotlight::*};

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

pub async fn load_all_data(id: &str) -> Result<AllAccountResponse, MyError> {
    let response = reqwest::get(format!("https://api.minaexplorer.com/accounts/{}", id)).await;

    match response {
        Ok(res) => match res.json::<AllAccountResponse>().await {
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
            value: account.balance.total,
            pill_variant: Some(PillVariant::Green),
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            value: account.nonce.to_string(),
            pill_variant: Some(PillVariant::Blue),
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            value: account.receipt_chain_hash,
            pill_variant: None,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            value: account.delegate,
            pill_variant: None,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            value: account.voting_for,
            pill_variant: None,
        },
    ]
}
