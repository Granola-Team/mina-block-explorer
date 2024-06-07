use super::graphql::{
    staking_ledgers_query, staking_ledgers_query::StakingLedgersQueryStakes, StakingLedgersQuery,
};
use crate::common::{constants::*, functions::*, models::*};
use graphql_client::reqwest::post_graphql;

pub fn get_public_key(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .public_key
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_stake(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegation_totals
        .as_ref()
        .and_then(|delegation_totals| delegation_totals.total_delegated_nanomina)
        .map(|stake| nanomina_to_mina(stake as u64))
        .unwrap_or("0".to_string())
}

pub fn get_stake_percentage(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegation_totals
        .as_ref()
        .and_then(|delegation_totals| delegation_totals.total_stake_percentage.as_ref())
        .map(|total_stake_percentage| format!("{}%", total_stake_percentage))
        .unwrap_or("0".to_string())
}

pub fn get_block_win_percentage(stake: &StakingLedgersQueryStakes) -> String {
    let pk_epoch_num_blocks = stake.pk_epoch_num_blocks.unwrap_or(0) as f64;
    stake.epoch_num_blocks.map_or_else(
        || "0".to_string(),
        |epoch_num_blocks| {
            if epoch_num_blocks != 0 {
                format!(
                    "{:.2}%",
                    100.0 * pk_epoch_num_blocks / epoch_num_blocks as f64
                )
            } else {
                "0".to_string()
            }
        },
    )
}

pub fn get_delegate(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegate
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_username(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .username
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

pub async fn load_data(
    epoch: Option<i64>,
    public_key: Option<String>,
    delegate: Option<String>,
) -> Result<staking_ledgers_query::ResponseData, MyError> {
    let variables = staking_ledgers_query::Variables {
        sort_by: staking_ledgers_query::StakeSortByInput::STAKE_DESC,
        limit: Some(TABLE_ROW_LIMIT),
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
