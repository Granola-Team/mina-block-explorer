use super::{graphql::*, models::PooledUserCommandsResponse};
use crate::common::{constants::*, models::MyError};
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

        Ok(transactions_query::ResponseData { transactions: txn })
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

pub async fn load_data(
    limit: i64,
    from_account: Option<String>,
    to_account: Option<String>,
    state_hash: Option<String>,
    block_height: Option<i64>,
    canonical: Option<bool>,
) -> Result<transactions_query::ResponseData, MyError> {
    let variables = transactions_query::Variables {
        sort_by: transactions_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit),
        query: transactions_query::TransactionQueryInput {
            from: from_account,
            to: to_account,
            hash: state_hash,
            block_height_lte: block_height,
            canonical,
            ..Default::default()
        },
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
