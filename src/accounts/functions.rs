use super::graphql::{accounts_query, AccountsQuery};
use crate::{
    common::{constants::GRAPHQL_ENDPOINT, models::*},
    logging,
};
use graphql_client::reqwest::post_graphql;

pub async fn load_data(
    limit: u64,
    public_key: Option<String>,
    username: Option<String>,
    balance: Option<i64>,
    delegate: Option<String>,
) -> Result<accounts_query::ResponseData, MyError> {
    logging::log!("{:#?}", delegate.clone());
    let query =
        if public_key.is_none() && username.is_none() && balance.is_none() && delegate.is_none() {
            None
        } else {
            Some(accounts_query::AccountQueryInput {
                public_key,
                username,
                balance_lte: balance,
                delegate,
            })
        };

    let variables = accounts_query::Variables {
        limit: Some(limit as i64),
        query,
        sort_by: accounts_query::AccountSortByInput::BALANCE_DESC,
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<AccountsQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
