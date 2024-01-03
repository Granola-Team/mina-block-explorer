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
fn build_query(public_key: Option<String>) -> transactions_query::TransactionQueryInput {
    transactions_query::TransactionQueryInput {
        from: public_key,
        canonical: Some(true),
        ..Default::default()
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