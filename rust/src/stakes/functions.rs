use super::graphql::{
    StakingLedgersQuery, staking_ledgers_query, staking_ledgers_query::StakingLedgersQueryStakes,
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
        .map(|number| format_number_for_html(&number, 11))
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

pub fn get_slot_win_likelihood(stake: &StakingLedgersQueryStakes) -> String {
    let stake_fraction = stake.delegation_totals.as_ref().and_then(|dt| {
        let delegated = dt.total_delegated_nanomina? as f64;
        let total = dt.total_currency? as f64;
        if total == 0.0 {
            None
        } else {
            Some(delegated / total)
        }
    });

    let probability = stake_fraction.map(|fraction| {
        let base_factor = 1.0_f64;
        let decay_factor = 0.75_f64;
        base_factor * base_factor * (1.0_f64 - (1.0_f64 - decay_factor).powf(fraction))
    });

    probability
        .and_then(|p| round_to_two_decimals(p * 100.0))
        .map(|d| format!("{d}%"))
        .unwrap_or("n/a".to_string())
}

pub fn get_delegate(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegate
        .as_ref()
        .map_or_else(String::new, ToString::to_string)
}

pub fn get_delegate_username(stake: &StakingLedgersQueryStakes) -> Option<String> {
    stake.delegate_username.clone()
}

pub fn get_username(stake: &StakingLedgersQueryStakes) -> Option<String> {
    stake.username.clone()
}

pub fn get_balance(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .balance
        .map(|number| format!("{:.1}", number))
        .map(format_mina)
        .map(|number| format_number_for_html(&number, 11))
        .unwrap_or_default()
}

pub fn get_delegators_count(stake: &StakingLedgersQueryStakes) -> String {
    stake
        .delegation_totals
        .as_ref()
        .and_then(|o| o.count_delegates)
        .map_or("0".to_string(), |o| o.to_string())
}

#[allow(clippy::too_many_arguments)]
pub async fn load_data(
    limit: Option<i64>,
    epoch: Option<u64>,
    public_key: Option<PublicKey>,
    delegate: Option<String>,
    stake: Option<String>,
    sort_by: staking_ledgers_query::StakesSortByInput,
    genesis_state_hash: Option<String>,
    username: Option<String>,
) -> Result<staking_ledgers_query::ResponseData, MyError> {
    if stake.is_some() && normalize_number_format(stake.as_deref().unwrap()).is_err() {
        return Err(MyError::ParseError(
            "Unable to normalize stake input".to_string(),
        ));
    }
    let variables = staking_ledgers_query::Variables {
        sort_by,
        limit,
        query: staking_ledgers_query::StakesQueryInput {
            stake_lte: stake
                .as_deref()
                .map(|num| normalize_number_format(num).ok().unwrap()),
            public_key: public_key.map(|pk| pk.as_str().to_string()),
            username,
            delegate,
            genesis_state_hash,
            epoch: epoch.map(|x| x as i64),
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
