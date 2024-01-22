use graphql_client::reqwest::post_graphql;

use crate::fee_transfers::graphql::fee_transfers_query;
use crate::common::models::MyError;

use super::graphql::FeeTransfersQuery;

pub async fn load_data(
    limit: i64,
    state_hash: Option<String>,
) -> Result<fee_transfers_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = fee_transfers_query::Variables {
        sort_by: fee_transfers_query::FeetransferSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: fee_transfers_query::FeetransferQueryInput {
            block_state_hash: Some(fee_transfers_query::BlockQueryInput {
                state_hash,
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<FeeTransfersQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}