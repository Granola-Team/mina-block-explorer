use crate::{
    common::{constants::*, models::*},
    internal_commands::graphql::{
        internal_commands_query, internal_commands_query::BlockQueryInput, InternalCommandsQuery,
    },
};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: i64,
    recipient: Option<String>,
    block_height: Option<i64>,
    state_hash: Option<String>,
    canonical: Option<bool>,
) -> Result<internal_commands_query::ResponseData, MyError> {
    let variables = internal_commands_query::Variables {
        sort_by: internal_commands_query::FeetransferSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: internal_commands_query::FeetransferQueryInput {
            block_height,
            block_state_hash: if let Some(sh) = state_hash {
                Some(BlockQueryInput {
                    state_hash: Some(sh),
                    ..Default::default()
                })
            } else {
                None
            },
            recipient,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
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
