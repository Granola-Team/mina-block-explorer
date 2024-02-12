use url::Url;

use crate::common::models::*;
use crate::common::spotlight::*;

use super::models::*;

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

pub async fn load_all_data(
    offset: Option<i8>,
    limit: Option<i8>,
    public_key: Option<String>,
) -> Result<AllAccountResponse, MyError> {
    let base_url = "https://minaexplorer.com/all-accounts";

    let mut url = Url::parse(base_url)?;

    match public_key {
        Some(pk) => {
            url.query_pairs_mut().append_pair("search[value]", &pk);
        }
        None => {
            url.query_pairs_mut()
                .append_pair("start", &offset.unwrap_or(0).to_string());
            url.query_pairs_mut()
                .append_pair("length", &limit.unwrap_or(50).to_string());
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

pub fn get_spotlight_loading_data() -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            value: None,
            pill_variant: None,
            copiable: false,
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            value: None,
            pill_variant: None,
            copiable: false,
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            value: None,
            pill_variant: None,
            copiable: true,
        },
    ]
}

pub fn get_spotlight_data(account: &AccountSummary) -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            value: Some(account.balance.total.clone()),
            pill_variant: Some(PillVariant::Green),
            copiable: false,
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            value: Some(account.nonce.to_string()),
            pill_variant: Some(PillVariant::Blue),
            copiable: false,
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            value: Some(account.receipt_chain_hash.to_string()),
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            value: Some(account.delegate.to_string()),
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            value: Some(account.voting_for.to_string()),
            pill_variant: None,
            copiable: true,
        },
    ]
}
