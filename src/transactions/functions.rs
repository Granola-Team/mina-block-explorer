use crate::common::models::MyError;
use graphql_client::reqwest::post_graphql;

use super::graphql::transactions_query::TransactionsQueryTransactions;
use super::graphql::*;

pub fn get_block_datetime(transaction: &TransactionsQueryTransactions) -> String {
    transaction.block.as_ref().map_or_else(String::new, |o| {
        o.date_time.map_or_else(String::new, |o1| o1.to_string())
    })
}

pub fn get_from(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .from
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_receiver_public_key(transaction: &TransactionsQueryTransactions) -> String {
    transaction.receiver.as_ref().map_or_else(String::new, |o| {
        o.public_key
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    })
}

pub fn get_fee(transaction: &TransactionsQueryTransactions) -> String {
    transaction.fee.map_or_else(String::new, |o| o.to_string())
}

pub fn get_hash(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .hash
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_amount(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .amount
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_to(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .to
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub async fn load_data(
    limit: i32,
    public_key: Option<String>,
) -> Result<transactions_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = transactions_query::Variables {
        sort_by: transactions_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit.into()),
        query: build_query(public_key),
    };

    let client = reqwest::Client::new();

    let response = post_graphql::<TransactionsQuery, _>(&client, url, variables)
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if let Some(errors) = response.errors {
        return Err(MyError::GraphQLError(errors));
    }

    response
        .data
        .ok_or(MyError::GraphQLEmpty("No data available".to_string()))
}

fn build_query(public_key: Option<String>) -> transactions_query::TransactionQueryInput {
    transactions_query::TransactionQueryInput {
        from: public_key,
        canonical: Some(true),
        ..Default::default()
    }
}
