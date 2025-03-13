use super::graphql::{
    tokens_query::{self, TokensSortByInput},
    TokensQuery,
};
use crate::common::{constants::GRAPHQL_ENDPOINT, models::MyError};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: Option<i64>,
    name: Option<String>,
    sort_by: Option<TokensSortByInput>,
) -> Result<tokens_query::ResponseData, MyError> {
    let query = name.map(|name| tokens_query::TokensQueryInput {
        token: Some(name),
        fetch_all_holders: Some(true),
        ..Default::default()
    });

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
