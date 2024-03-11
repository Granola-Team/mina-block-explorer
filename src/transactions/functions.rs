use super::graphql::{transactions_query::TransactionsQueryTransactions, *};
use crate::common::{functions::*, models::MyError};
use graphql_client::reqwest::post_graphql;
use std::error::Error;

pub fn get_failure_reason(transaction: &TransactionsQueryTransactions) -> Option<String> {
    transaction.failure_reason.clone()
}

pub fn get_block_datetime(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .block
        .as_ref()
        .and_then(|b| b.date_time)
        .map_or_else(String::new, |o1| o1.to_string())
}

pub fn get_block_height(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .block_height
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_canonical(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .canonical
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_kind(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .kind
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_payment_id(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .id
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_nonce(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .nonce
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_memo(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .memo
        .as_ref()
        .map_or_else(String::new, |o| decode_memo(o).unwrap_or("".to_string()))
}

pub fn get_block_state_hash(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .block
        .as_ref()
        .and_then(|b| b.state_hash.as_ref())
        .map_or_else(String::new, |o1| o1.to_string())
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
    transaction.fee.map(nanomina_to_mina).unwrap_or_default()
}

pub fn get_hash(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .hash
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

pub fn get_amount(transaction: &TransactionsQueryTransactions) -> String {
    transaction.amount.map(nanomina_to_mina).unwrap_or_default()
}

pub fn get_to(transaction: &TransactionsQueryTransactions) -> String {
    transaction
        .to
        .as_ref()
        .map_or_else(String::new, |o| o.to_string())
}

/// 0th byte is a tag to distinguish digests from other data
/// 1st byte is length, always 32 for digests
/// bytes 2 to 33 are data,
/// 0-right-padded if length is less than 32
pub fn decode_memo(encoded: &str) -> Result<String, Box<dyn Error>> {
    let decoded = bs58::decode(encoded).into_vec()?;
    if decoded.len() < 3 {
        return Err(Box::from("Decoded data is too short"));
    }
    let length = decoded[2] as usize;
    if decoded.len() < 3 + length {
        return Err(Box::from("Invalid length specified"));
    }

    Ok(String::from_utf8(decoded[3..3 + length].to_vec())?)
}

pub async fn load_data(
    limit: i32,
    public_key: Option<String>,
    state_hash: Option<String>,
    payment_id: Option<String>,
    canonical: Option<bool>,
) -> Result<transactions_query::ResponseData, MyError> {
    let url = "https://graphql.minaexplorer.com";
    let variables = transactions_query::Variables {
        sort_by: transactions_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit.into()),
        query: transactions_query::TransactionQueryInput {
            from: public_key,
            hash: state_hash,
            id: payment_id,
            canonical,
            ..Default::default()
        },
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

#[cfg(test)]
mod tests {
    use crate::transactions::functions::decode_memo;

    #[test]
    fn test_b58_decoding() {
        let memo_hash = "E4Yf2t3NSjf3NC3D7MxX2QvXWXt1p8rxKgJxHHQjhCjdsqu795neB";
        let memo_str = decode_memo(memo_hash).unwrap();
        assert_eq!("Bon matin", memo_str);
    }
}
