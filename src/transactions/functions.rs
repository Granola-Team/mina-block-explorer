use super::graphql::*;
use crate::common::{constants::GRAPHQL_ENDPOINT, models::MyError};
use graphql_client::reqwest::post_graphql;
use std::error::Error;

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
    from_account: Option<String>,
    to_account: Option<String>,
    state_hash: Option<String>,
    canonical: Option<bool>,
) -> Result<transactions_query::ResponseData, MyError> {
    let variables = transactions_query::Variables {
        sort_by: transactions_query::TransactionSortByInput::BLOCKHEIGHT_DESC,
        limit: Some(limit.into()),
        query: transactions_query::TransactionQueryInput {
            from: from_account,
            to: to_account,
            hash: state_hash,
            canonical: if canonical.is_none() {
                Some(true)
            } else {
                canonical
            },
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
