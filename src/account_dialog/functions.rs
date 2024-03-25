use super::graphql::{account_activity_query, AccountActivityQuery};
use crate::common::models::MyError;
use graphql_client::reqwest::post_graphql;
use leptos::*;
use leptos_router::*;

pub fn get_base_page_path(location: Location) -> String {
    let path = location.pathname.with(|path| path.clone());
    let path_parts: Vec<&str> = path.split("/accounts").collect();
    match path_parts.first() {
        Some(base) => base.to_string(),
        None => "/".to_string(),
    }
}

pub async fn load_data(
    public_key: Option<String>,
    blocks_limit: Option<i64>,
    snarks_limit: Option<i64>,
    trans_limit: Option<i64>,
    canonical: Option<bool>,
) -> Result<account_activity_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = account_activity_query::Variables {
        blocks_sort_by: account_activity_query::BlockSortByInput::BLOCKHEIGHT_DESC,
        snarks_sort_by: account_activity_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        trans_sort_by: account_activity_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        blocks_limit: Some(blocks_limit.unwrap_or_default()),
        snarks_limit: Some(snarks_limit.unwrap_or_default()),
        trans_limit: Some(trans_limit.unwrap_or_default()),
        blocks_query: account_activity_query::BlockQueryInput {
            creator: public_key.clone(),
            canonical,
            ..Default::default()
        },
        snarks_query: account_activity_query::SnarkQueryInput {
            prover: public_key.clone(),
            canonical,
            ..Default::default()
        },
        outgoing_trans_query: account_activity_query::TransactionQueryInput {
            from: public_key.clone(),
            canonical,
            ..Default::default()
        },
        incoming_trans_query: account_activity_query::TransactionQueryInput {
            to: public_key,
            canonical,
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountActivityQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
