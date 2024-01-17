use graphql_client::reqwest::post_graphql;

use super::graphql::{stakes_query, stakes_query::StakesQueryStakes, StakesQuery};
use crate::common::functions::*;
use crate::common::models::*;

pub fn get_public_key(stake: &StakesQueryStakes) -> String {
    stake
        .public_key
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_balance(stake: &StakesQueryStakes) -> String {
    stake
        .balance
        .and_then(nanomina_to_mina)
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_delegate(stake: &StakesQueryStakes) -> String {
    stake
        .delegate
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}
pub fn get_delegators_count(stake: &StakesQueryStakes) -> String {
    stake
        .delegation_totals
        .as_ref()
        .and_then(|o| o.count_delegates)
        .map_or("0".to_string(), |o| o.to_string())
}
pub fn get_ledger_hash(stake: &StakesQueryStakes) -> String {
    stake
        .ledger_hash
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub async fn load_data(
    limit: i64,
    epoch: Option<i64>,
    public_key: Option<String>,
) -> Result<stakes_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = stakes_query::Variables {
        sort_by: stakes_query::StakeSortByInput::BALANCE_DESC,
        limit: Some(limit),
        query: stakes_query::StakeQueryInput {
            public_key,
            epoch,
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<StakesQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
