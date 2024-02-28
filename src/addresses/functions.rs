use url::Url;

use crate::common::models::*;
use crate::common::functions::*;
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
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            ..Default::default()
        },
    ]
}

pub fn get_spotlight_data(account: &AccountSummary) -> Vec<SpotlightEntry> {
    vec![
        SpotlightEntry {
            label: String::from("Balance"),
            any_el: Some(wrap_in_pill(decorate_with_currency_tag(account.balance.total.clone(),"mina".to_string()),PillVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            any_el: Some(convert_to_pill(account.nonce.to_string(),PillVariant::Blue)),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            any_el: Some(convert_to_span(account.receipt_chain_hash.to_string())),
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            any_el: Some(convert_to_span(account.delegate.to_string())),
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            any_el: Some(convert_to_span(account.voting_for.to_string())),
            copiable: true,
        },
    ]
}
