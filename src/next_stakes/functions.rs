use crate::{
    common::{
        constants::{GRAPHQL_ENDPOINT, TABLE_ROW_LIMIT},
        functions::*,
        models::*,
    },
    next_stakes::graphql::{next_staking_ledgers_query, *},
};
use graphql_client::reqwest::post_graphql;
use next_staking_ledgers_query::NextStakingLedgersQueryNextstakes;

pub fn get_public_key(nextstakes: &NextStakingLedgersQueryNextstakes) -> String {
    nextstakes
        .public_key
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_stake(nextstakes: &NextStakingLedgersQueryNextstakes) -> String {
    nextstakes
        .next_delegation_totals
        .as_ref()
        .and_then(|delegation_totals| delegation_totals.total_delegated)
        .map(|stake| format_mina(stake.to_string()))
        .unwrap_or("0".to_string())
}

pub fn get_delegate(nextstakes: &NextStakingLedgersQueryNextstakes) -> String {
    nextstakes
        .delegate
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub fn get_delegators_count(nextstakes: &NextStakingLedgersQueryNextstakes) -> String {
    nextstakes
        .next_delegation_totals
        .as_ref()
        .and_then(|o| o.count_delegates)
        .map_or("0".to_string(), |o| o.to_string())
}

pub fn get_ledger_hash(nextstakes: &NextStakingLedgersQueryNextstakes) -> String {
    nextstakes
        .ledger_hash
        .as_ref()
        .map_or("".to_string(), |o| o.to_string())
}

pub async fn load_data(
    public_key: Option<String>,
    delegate: Option<String>,
) -> Result<next_staking_ledgers_query::ResponseData, MyError> {
    let variables = next_staking_ledgers_query::Variables {
        sort_by: next_staking_ledgers_query::NextstakeSortByInput::BALANCE_DESC,
        limit: Some(TABLE_ROW_LIMIT),
        query: next_staking_ledgers_query::NextstakeQueryInput {
            delegate,
            public_key,
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<NextStakingLedgersQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
