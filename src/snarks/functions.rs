use super::graphql::{snarks_query::SnarksQuerySnarks, *};
use crate::common::{functions::*, models::*};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: i32,
    public_key: Option<String>,
    block_state_hash: Option<String>,
    canonical: Option<bool>,
) -> Result<snarks_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = snarks_query::Variables {
        sort_by: snarks_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit.into()),
        query: snarks_query::SnarkQueryInput {
            prover: public_key,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
            block: if block_state_hash.is_none() {
                None
            } else {
                Some(snarks_query::BlockQueryInput {
                    state_hash: block_state_hash,
                    ..Default::default()
                })
            },
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<SnarksQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}

pub fn get_snark_date_time(snark: &SnarksQuerySnarks) -> String {
    snark.date_time.map_or_else(String::new, |o| o.to_string())
}

pub fn get_block_height(snark: &SnarksQuerySnarks) -> String {
    snark
        .block_height
        .map_or_else(String::new, |height| height.to_string())
}

pub fn get_date_time(snark: &SnarksQuerySnarks) -> String {
    snark
        .date_time
        .map_or_else(String::new, |dt| dt.to_string())
}

pub fn get_prover(snark: &SnarksQuerySnarks) -> String {
    snark
        .prover
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_canonical(snark: &SnarksQuerySnarks) -> Option<bool> {
    snark.canonical
}

pub fn get_work_ids(snark: &SnarksQuerySnarks) -> Vec<String> {
    snark.work_ids.as_ref().map_or_else(Vec::new, |ids| {
        ids.iter()
            .map(|id| id.map_or_else(String::new, |id| id.to_string()))
            .collect::<Vec<_>>()
    })
}

pub fn get_block_state_hash(snark: &SnarksQuerySnarks) -> String {
    snark.block.as_ref().map_or_else(String::new, |blk| {
        blk.state_hash
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    })
}

pub fn get_fee(snark: &SnarksQuerySnarks) -> String {
    snark.fee.map(nanomina_to_mina).unwrap_or_default()
}
