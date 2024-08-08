use super::models::*;
use crate::common::{constants::*, models::MyError};

pub async fn load_snark_fees(limit: Option<u64>) -> Result<SnarkFeesResponse, MyError> {
    let query_body = format!(
        r#"{{"query":"query SnarkFeesQuery(\n  $limit: Int = 100\n) {{\n  blocks(limit: $limit) {{\n    blockHeight\n    snarkFees\n    snarkJobs{{\n fee }}\n}}\n}}\n","variables":{{"limit": {}}},"operationName":"SnarkFeesQuery"}}"#,
        limit.unwrap_or(100)
    );
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        Ok(response
            .json::<SnarkFeesResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

pub async fn load_block_summary_data() -> Result<BlocksAnalyticsResponse, MyError> {
    let query_body = r#"{"query":"query BlocksQuery(\n  $limit: Int = 1\n) {\n  blocks(limit: $limit) {\n    epoch_num_blocks\n    total_num_blocks\n  }\n}\n","variables":{"limit":1},"operationName":"BlocksQuery"}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlocksAnalyticsResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}
