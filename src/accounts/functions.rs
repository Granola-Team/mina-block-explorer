use leptos::*;
use leptos_router::*;
use url::Url;

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

pub async fn load_all_data(offset: Option<i8>, limit: Option<i8>, public_key: Option<String>) -> Result<AllAccountResponse, MyError> {
    let base_url = "https://minaexplorer.com/all-accounts";

    let mut url = Url::parse(base_url)?;

    match public_key {
        Some(pk) => {
            url.query_pairs_mut().append_pair("search[value]", &pk);
        },
        None => {
            url.query_pairs_mut().append_pair("start", &offset.unwrap_or(0).to_string());
            url.query_pairs_mut().append_pair("length", &limit.unwrap_or(50).to_string());
        }
    }

    let response = reqwest::get(url.as_str()).await;

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
