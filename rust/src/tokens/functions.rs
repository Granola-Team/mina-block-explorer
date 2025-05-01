use super::graphql::{
    tokens_query::{self, TokensSortByInput},
    TokensQuery,
};
use crate::common::{constants::GRAPHQL_ENDPOINT, models::MyError};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: Option<i64>,
    name: Option<String>,
    token: Option<String>,
    sort_by: Option<TokensSortByInput>,
) -> Result<tokens_query::ResponseData, MyError> {
    let query = match (name, token) {
        (Some(name), Some(token)) => Some(tokens_query::TokensQueryInput {
            symbol: Some(name),
            token: Some(token),
            ..Default::default()
        }),
        (Some(name), None) => Some(tokens_query::TokensQueryInput {
            symbol: Some(name),
            ..Default::default()
        }),
        (None, Some(token)) => Some(tokens_query::TokensQueryInput {
            token: Some(token),
            ..Default::default()
        }),
        (None, None) => None,
    };

    let variables = tokens_query::Variables {
        limit,
        query,
        sort_by,
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<TokensQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
