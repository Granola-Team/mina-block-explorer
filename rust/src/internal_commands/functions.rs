use crate::{
    common::{constants::*, models::*},
    internal_commands::graphql::{
        InternalCommandsQuery, internal_commands_query, internal_commands_query::BlockQueryInput,
    },
};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    mut limit: Option<i64>,
    recipient: Option<String>,
    block_height: Option<u64>,
    state_hash: Option<String>,
    canonical: Option<bool>,
) -> Result<internal_commands_query::ResponseData, MyError> {
    let variables = internal_commands_query::Variables {
        sort_by: internal_commands_query::InternalCommandSortByInput::BLOCK_HEIGHT_DESC,
        limit: Some(*limit.get_or_insert(25i64)),
        query: internal_commands_query::InternalCommandQueryInput {
            block_height_lte: block_height.map(|x| x as i64),
            block_state_hash: state_hash.map(|sh| BlockQueryInput {
                state_hash: Some(sh),
                ..Default::default()
            }),
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
