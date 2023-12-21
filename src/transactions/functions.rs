
use crate::api_models::MyError;
use chrono::{DateTime, Utc, Duration};

use super::models::*;


pub async fn load_data(limit: i32, public_key: Option<String>) -> Result<TransactionsResponse, MyError> {
    let mut query = String::from(r#"{"query":"query MyQuery {\n  transactions(limit: ::limit::, sortBy: DATETIME_DESC::query::) {\n    amount\n    fee\n    to\n    from\n    hash\n    block {\n      dateTime\n    }\n    receiver {\n      publicKey\n    }\n  }\n}\n","variables":null,"operationName":"MyQuery"}"#);
    query = query.replace("::limit::", &limit.to_string());
    if let Some(key) = public_key {
        let substring_string = ", query: {from: \\\"::public_key::\\\", canonical: true}".replace("::public_key::", &key);
        query = query.replace("::query::", &substring_string);
    } else {
        query = query.replace("::query::", ", query: {canonical: true}");
    }
    
    let client = reqwest::Client::new();
    let response = client.post("https://graphql.minaexplorer.com")
        .body(query)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<TransactionsResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}


// Function to calculate and print the time elapsed since the given timestamp
pub fn print_time_since(timestamp: &str) -> String {
    // Parse the input timestamp
    let past_time = match timestamp.parse::<DateTime<Utc>>() {
        Ok(time) => time,
        Err(_e) => return String::from("Unknown")
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
        },
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



