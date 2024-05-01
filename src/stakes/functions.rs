use super::graphql::{
    staking_ledgers_query, staking_ledgers_query::StakingLedgersQueryStakes, StakingLedgersQuery,
};
use crate::common::{constants::GRAPHQL_ENDPOINT, functions::*, models::*};
use graphql_client::reqwest::post_graphql;

pub fn get_public_key(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .public_key
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_balance(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .balance
        .map(|b| format_mina(b.to_string()))
        .unwrap_or_default()
}

pub fn get_delegate(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegate
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}
pub fn get_delegators_count(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegation_totals
        .as_ref()
        .and_then(|o| o.count_delegates)
        .map_or("0".to_string(), |o| o.to_string())
}
pub fn get_ledger_hash(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .ledger_hash
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub async fn load_data(
    limit: i64,
    epoch: Option<i64>,
    public_key: Option<String>,
    delegate: Option<String>,
) -> Result<staking_ledgers_query::ResponseData, MyError> {
    let variables = staking_ledgers_query::Variables {
        sort_by: staking_ledgers_query::StakeSortByInput::BALANCE_DESC,
        limit: Some(limit),
        query: staking_ledgers_query::StakeQueryInput {
            public_key,
            delegate,
            epoch,
            ..Default::default()
        },
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<StakingLedgersQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}
