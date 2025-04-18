use super::{
    graphql::{transactions_query::TransactionsQueryTransactions, *},
    models::PooledUserCommandsResponse,
};
use crate::common::{
    constants::*,
    functions::format_json_array_pretty,
    models::{MyError, TransactionKind},
};
use graphql_client::reqwest::post_graphql;

pub async fn load_pending_txn() -> Result<transactions_query::ResponseData, MyError> {
    let response = reqwest::get("https://proxy.minaexplorer.com/graphql?query={pooledUserCommands{id hash kind nonce source{publicKey}receiver{publicKey}amount fee memo failureReason feeToken}}")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let pending_txn = response
            .json::<PooledUserCommandsResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;

        let txn: Vec<Option<transactions_query::TransactionsQueryTransactions>> = pending_txn
            .data
            .pooled_user_commands
            .into_iter()
            .map(|pt| Some(transactions_query::TransactionsQueryTransactions::from(pt)))
            .collect::<Vec<_>>();

        Ok(transactions_query::ResponseData {
            transactions: txn,
            other_transactions: vec![],
            tokens: vec![],
        })
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn load_data(
    limit: Option<u64>,
    from_account: Option<String>,
    to_account: Option<String>,
    txn_hash: Option<String>,
    block_height: Option<u64>,
    block_backscan_limit: Option<u64>,
    state_hash: Option<String>,
    canonical: Option<bool>,
    is_applied: Option<bool>,
    kind: Option<TransactionKind>,
    token: Option<String>,
) -> Result<transactions_query::ResponseData, MyError> {
    let variables = transactions_query::Variables {
        sort_by: transactions_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        limit: limit.map_or(Some(25i64), |l| Some(l as i64)),
        txn_query: transactions_query::TransactionQueryInput {
            is_applied,
            from: from_account,
            to: to_account,
            hash: txn_hash.clone(),
            block_height_lte: block_height.map(|x| x as i64),
            block_height_gt: block_height
                .zip(block_backscan_limit)
                .map(|(h, l)| (h - l) as i64),
            canonical,
            block: state_hash
                .clone()
                .map(|sh| transactions_query::BlockQueryInput {
                    state_hash: Some(sh),
                    ..Default::default()
                }),
            kind: kind.map(|k| k.to_string()),
            token: token.clone(),
            ..Default::default()
        },
        other_txn_query: Some(transactions_query::TransactionQueryInput {
            hash: txn_hash,
            ..Default::default()
        }),
        token_query: Some(transactions_query::TokensQueryInput {
            token,
            owner: None,
            symbol: None,
            supply: None,
        }),
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<TransactionsQuery, _>(&client, GRAPHQL_ENDPOINT, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}

pub fn get_actions(txn: &TransactionsQueryTransactions) -> Result<String, serde_json::Error> {
    let json_arr = txn
        .clone()
        .zkapp
        .map(|zkapp| zkapp.actions.into_iter().map(Some).collect())
        .unwrap_or_default();

    format_json_array_pretty(json_arr)
}

pub fn get_events(txn: &TransactionsQueryTransactions) -> Result<String, serde_json::Error> {
    let json_arr = txn
        .clone()
        .zkapp
        .map(|zkapp| zkapp.events.into_iter().map(Some).collect())
        .unwrap_or_default();

    format_json_array_pretty(json_arr)
}
