use super::graphql::{accounts_query, AccountsQuery};
use crate::common::{constants::GRAPHQL_ENDPOINT_2, models::*};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: i64,
    public_key: Option<String>,
) -> Result<accounts_query::ResponseData, MyError> {
    let variables = accounts_query::Variables {
        limit: Some(limit),
        query: public_key.map(|pk| accounts_query::AccountQueryInput {
            public_key: Some(pk),
        }),
        sort_by: accounts_query::AccountSortByInput::BALANCE_DESC,
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountsQuery, _>(&client, GRAPHQL_ENDPOINT_2, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}