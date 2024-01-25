use graphql_client::reqwest::post_graphql;

use crate::common::models::*;
use crate::next_stakes::graphql::{next_stakes_query, *};

pub async fn load_data(limit: i64) -> Result<next_stakes_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = next_stakes_query::Variables {
        sort_by: next_stakes_query::NextstakeSortByInput::BALANCE_DESC,
        limit: Some(limit),
        query: next_stakes_query::NextstakeQueryInput {
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<NextStakesQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
