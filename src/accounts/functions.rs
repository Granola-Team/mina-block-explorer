use super::{
    graphql::{
        accounts_query::{self, AccountSortByInput},
        AccountsQuery,
    },
    models::TokenSymbolResponse,
};
use crate::common::{constants::GRAPHQL_ENDPOINT, models::*};
use graphql_client::reqwest::post_graphql;

pub async fn load_token_symbol(token_id: Option<String>) -> Result<TokenSymbolResponse, MyError> {
    if token_id.is_none() {
        return Err(MyError::ParseError("token id must be set".into()));
    }
    let query_body = format!(
        r#"{{"query": "{{ tokens(limit: 1, query: {{ token: \"{}\" }}) {{ symbol }} }}"}}"#,
        token_id.unwrap(),
    );
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        Ok(response
            .json::<TokenSymbolResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn load_data(
    limit: Option<i64>,
    public_key: Option<String>,
    username: Option<String>,
    mina_balance: Option<f64>,
    delegate: Option<String>,
    sort_by: Option<accounts_query::AccountSortByInput>,
    zk_app: Option<bool>,
    token: Option<String>,
) -> Result<accounts_query::ResponseData, MyError> {
    let query = if public_key.is_none()
        && username.is_none()
        && mina_balance.is_none()
        && delegate.is_none()
        && token.is_none()
        && zk_app.is_none()
    {
        None
    } else {
        Some(accounts_query::AccountQueryInput {
            public_key,
            username,
            token,
            balance_lte: mina_balance
                .map(|mb| mb * 1_000_000_000f64)
                .map(|nmb| nmb as i64),
            delegate,
            zkapp: zk_app,
        })
    };

    let variables = accounts_query::Variables {
        limit,
        query,
        sort_by: sort_by.unwrap_or(AccountSortByInput::BALANCE_DESC),
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
