use crate::{
    common::{constants::*, models::*},
    internal_commands::graphql::{internal_commands_query, InternalCommandsQuery},
};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(limit: i64) -> Result<internal_commands_query::ResponseData, MyError> {
    let variables = internal_commands_query::Variables {
        sort_by: internal_commands_query::FeetransferSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: internal_commands_query::FeetransferQueryInput {
            canonical: Some(true),
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<InternalCommandsQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
