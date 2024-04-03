use super::{
    graphql::{account_activity_query, AccountActivityQuery},
    models::*,
};
use crate::common::{constants::GRAPHQL_ENDPOINT, functions::*, models::*, spotlight::*};
use graphql_client::reqwest::post_graphql;
use leptos::*;
use leptos_router::*;
use rand::distributions::{Distribution, Uniform};

pub fn get_base_page_path(location: Location) -> String {
    let path = location.pathname.with(|path| path.clone());
    let path_parts: Vec<&str> = path.split("/accounts").collect();
    match path_parts.first() {
        Some(base) => base.to_string(),
        None => "/".to_string(),
    }
}

pub async fn load_account_data(id: &str) -> Result<AccountResponse, MyError> {
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

pub fn stub_account_summaries(size: usize) -> Vec<AllAccountSummary> {
    let mut rng = rand::thread_rng();
    let int_dist = Uniform::from(0..=1000);

    (0..size)
        .map(|_| {
            let balance = generate_random_mina_price();

            AllAccountSummary {
                pk: generate_base58_string(44),
                balance,
                delegate: generate_base58_string(44),
                token: int_dist.sample(&mut rng),
                nonce: int_dist.sample(&mut rng),
                receipt_chain_hash: generate_base58_string(44),
                voting_for: generate_base58_string(44),
                public_key: generate_base58_string(44),
                username: generate_random_string(10),
            }
        })
        .collect()
}

pub async fn load_data(
    public_key: Option<String>,
    blocks_limit: Option<i64>,
    snarks_limit: Option<i64>,
    trans_limit: Option<i64>,
    canonical: Option<bool>,
) -> Result<account_activity_query::ResponseData, MyError> {
    let variables = account_activity_query::Variables {
        blocks_sort_by: account_activity_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        snarks_sort_by: account_activity_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        trans_sort_by: account_activity_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        blocks_limit: Some(blocks_limit.unwrap_or_default()),
        snarks_limit: Some(snarks_limit.unwrap_or_default()),
        trans_limit: Some(trans_limit.unwrap_or_default()),
        blocks_query: account_activity_query::BlockQueryInput {
            creator: public_key.clone(),
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        snarks_query: account_activity_query::SnarkQueryInput {
            prover: public_key.clone(),
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        outgoing_trans_query: account_activity_query::TransactionQueryInput {
            from: public_key.clone(),
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
        incoming_trans_query: account_activity_query::TransactionQueryInput {
            to: public_key,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountActivityQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
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
            any_el: Some(wrap_in_pill(
                decorate_with_currency_tag(account.balance.total.clone(), "mina".to_string()),
                ColorVariant::Green,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Nonce"),
            any_el: Some(convert_to_pill(
                account.nonce.to_string(),
                ColorVariant::Grey,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: String::from("Receipt Chain Hash"),
            any_el: Some(convert_to_span(account.receipt_chain_hash.to_string())),
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Delegate"),
            any_el: Some({
                let account = account.delegate.to_string();
                convert_to_link(account.clone(), format!("/addresses/accounts/{}", account))
            }),
            copiable: true,
        },
        SpotlightEntry {
            label: String::from("Voting For"),
            any_el: Some(convert_to_span(account.voting_for.to_string())),
            copiable: true,
        },
    ]
}
