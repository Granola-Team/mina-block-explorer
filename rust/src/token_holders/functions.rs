use super::graphql::{TokenHoldersQuery, token_holders_query};
use crate::common::{constants::GRAPHQL_ENDPOINT, models::MyError};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    account: String,
    token: String,
) -> Result<token_holders_query::ResponseData, MyError> {
    let query = token_holders_query::TokenHoldersQueryInput {
        token: Some(token),
        holder: Some(account),
    };

    let variables = token_holders_query::Variables { limit: 1, query };

    let client = reqwest::Client::new();

    let response = post_graphql::<TokenHoldersQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
