use crate::api_models::MyError;
use chrono::{DateTime, Duration, Utc};
use graphql_client::reqwest::post_graphql;

use super::graphql::transactions_query::TransactionsQueryTransactions;
use super::graphql::{transactions_query, TransactionsQuery};
use super::models::*;

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
        sort_by: transactions_query::TransactionSortByInput::DATETIME_DESC,
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

// Function to calculate and print the time elapsed since the given timestamp
pub fn print_time_since(timestamp: &str) -> String {
    // Parse the input timestamp
    let past_time = match timestamp.parse::<DateTime<Utc>>() {
        Ok(time) => time,
        Err(_e) => return String::from("Unknown"),
    };

    // Get the current time
    let now = Utc::now();

    // Calculate the duration since the given timestamp
    let duration_since = now.signed_duration_since(past_time);

    // Format and return the duration
    format_duration(&duration_since)
}

fn format_duration(duration: &Duration) -> String {
    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else {
        format!("{} minutes ago", duration.num_minutes())
    }
}

pub fn get_status(timestamp: &str) -> Status {
    match timestamp.parse::<DateTime<Utc>>() {
        Ok(parsed_timestamp) => {
            if Utc::now() < parsed_timestamp {
                Status::Pending
            } else {
                Status::Complete
            }
        }
        Err(_) => Status::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_duration_days() {
        let duration = Duration::days(3);
        assert_eq!(format_duration(&duration), "3 days ago");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = Duration::hours(5);
        assert_eq!(format_duration(&duration), "5 hours ago");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::minutes(45);
        assert_eq!(format_duration(&duration), "45 minutes ago");
    }

    #[test]
    fn test_format_duration_mix() {
        let duration = Duration::hours(26);
        assert_eq!(format_duration(&duration), "1 days ago");
    }
}

fn build_query(public_key: Option<String>) -> transactions_query::TransactionQueryInput {
    transactions_query::TransactionQueryInput {
        from: public_key,
        canonical: Some(true),
        fee_in: None,
        canonical_exists: None,
        memo_lt: None,
        from_account: None,
        memo_gte: None,
        fee_gt: None,
        to_account_exists: None,
        kind_lte: None,
        fee_token_in: None,
        token_lt: None,
        fee_exists: None,
        memo_gt: None,
        token_nin: None,
        token_gte: None,
        canonical_ne: None,
        hash_gt: None,
        receiver_exists: None,
        failure_reason_exists: None,
        date_time_exists: None,
        nonce_nin: None,
        fee_token_gte: None,
        id_in: None,
        is_delegation_exists: None,
        fee_payer: None,
        date_time_ne: None,
        kind_gt: None,
        amount_ne: None,
        to_gte: None,
        fee_payer_exists: None,
        kind_lt: None,
        id_lt: None,
        hash_ne: None,
        to_nin: None,
        date_time_nin: None,
        block_height_exists: None,
        nonce_lte: None,
        fee_token_nin: None,
        id: None,
        fee_token: None,
        to_account: None,
        block_height_lte: None,
        and: None,
        amount: None,
        fee: None,
        fee_token_lt: None,
        nonce_gt: None,
        amount_gt: None,
        receiver: None,
        hash_gte: None,
        token_ne: None,
        to_exists: None,
        source: None,
        fee_lt: None,
        fee_gte: None,
        hash_lt: None,
        amount_gte: None,
        hash_exists: None,
        failure_reason_ne: None,
        id_gte: None,
        kind_exists: None,
        block_height_gte: None,
        fee_ne: None,
        amount_lte: None,
        from_lte: None,
        failure_reason_lte: None,
        memo_ne: None,
        hash: None,
        nonce_ne: None,
        failure_reason_lt: None,
        from_in: None,
        block_height_nin: None,
        id_ne: None,
        amount_nin: None,
        kind_gte: None,
        from_gte: None,
        from_nin: None,
        is_delegation: None,
        nonce_lt: None,
        from_account_exists: None,
        to_gt: None,
        token: None,
        failure_reason_in: None,
        kind_ne: None,
        token_exists: None,
        id_nin: None,
        fee_token_ne: None,
        date_time_gte: None,
        to_in: None,
        block_exists: None,
        date_time_lt: None,
        from_exists: None,
        kind_nin: None,
        to_ne: None,
        block_height: None,
        failure_reason_gt: None,
        id_gt: None,
        date_time_lte: None,
        block_height_ne: None,
        hash_nin: None,
        to_lte: None,
        nonce: None,
        memo_in: None,
        fee_token_exists: None,
        fee_token_gt: None,
        memo: None,
        from_gt: None,
        failure_reason_nin: None,
        token_gt: None,
        fee_nin: None,
        kind_in: None,
        fee_lte: None,
        or: None,
        kind: None,
        memo_exists: None,
        from_lt: None,
        date_time_in: None,
        source_exists: None,
        hash_lte: None,
        id_lte: None,
        hash_in: None,
        block_height_gt: None,
        amount_lt: None,
        block_height_lt: None,
        amount_in: None,
        failure_reason: None,
        memo_nin: None,
        nonce_exists: None,
        failure_reason_gte: None,
        fee_token_lte: None,
        token_lte: None,
        is_delegation_ne: None,
        date_time: None,
        memo_lte: None,
        block: None,
        date_time_gt: None,
        from_ne: None,
        nonce_in: None,
        id_exists: None,
        block_height_in: None,
        amount_exists: None,
        nonce_gte: None,
        token_in: None,
        to_lt: None,
        to: None,
    }
}
