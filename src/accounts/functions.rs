use leptos::*;
use leptos_router::*;

use crate::common::models::MyError;

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

pub fn get_summary_items(account: AccountSummary) -> Vec<(String, String, bool)>{
    vec![
        (String::from("Balance"), account.balance.total ,true),
        (String::from("Nonce"), account.nonce.to_string(),true),
        (
            String::from("Receipt Chain Hash"),
            account.receipt_chain_hash,
            false
        ),
        (
            String::from("Delegate"),
            account.delegate,
            false
        ),
        (
            String::from("Voting For"),
            account.voting_for,
            false
        ),
    ]
}