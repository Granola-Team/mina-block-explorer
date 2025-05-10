use super::graphql::{snarks_query::SnarksQuerySnarks, *};
use crate::common::{constants::*, functions::*, models::*};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: Option<i64>,
    prover: Option<String>,
    block_state_hash: Option<String>,
    block_height: Option<u64>,
    canonical: Option<bool>,
) -> Result<snarks_query::ResponseData, MyError> {
    let variables = snarks_query::Variables {
        sort_by: snarks_query::SnarkSortByInput::BLOCKHEIGHT_DESC,
        limit,
        query: snarks_query::SnarkQueryInput {
            block_height_lte: block_height.map(|x| x as i64),
            prover,
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

    let response = post_graphql::<SnarksQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}

pub fn get_block_height(snark: &SnarksQuerySnarks) -> String {
    snark
        .block_height
        .map_or_else(String::new, |height| format_number(height.to_string()))
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

pub fn get_prover_username(snark: &SnarksQuerySnarks) -> String {
    snark
        .prover_username
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_block_state_hash(snark: &SnarksQuerySnarks) -> String {
    snark.block.as_ref().map_or_else(String::new, |blk| {
        blk.state_hash
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    })
}

pub fn get_fee(snark: &SnarksQuerySnarks) -> String {
    snark
        .fee
        .map(|f| f.round() as u64)
        .map(nanomina_to_mina)
        .map(|number| format_number_for_html(&number, LHS_MAX_SPACE_FEES))
        .unwrap_or_default()
}
