use graphql_client::reqwest::post_graphql;

use crate::common::models::*;
use crate::next_stakes::graphql::{next_stakes_query, *};

use next_stakes_query::NextStakesQueryNextstakes;

pub fn get_public_key(nextstakes: &NextStakesQueryNextstakes) -> String {
    nextstakes
        .public_key
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_balance(nextstakes: &NextStakesQueryNextstakes) -> String {
    nextstakes.balance.map_or("".to_string(), |o| o.to_string())
}

pub fn get_delegate(nextstakes: &NextStakesQueryNextstakes) -> String {
    nextstakes
        .delegate
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_delegators_count(nextstakes: &NextStakesQueryNextstakes) -> String {
    nextstakes
        .next_delegation_totals
        .as_ref()
        .and_then(|o| o.count_delegates)
        .map_or("0".to_string(), |o| o.to_string())
}

pub fn get_ledger_hash(nextstakes: &NextStakesQueryNextstakes) -> String {
    nextstakes
        .ledger_hash
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

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
