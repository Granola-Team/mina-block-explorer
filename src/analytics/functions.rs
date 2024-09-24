use super::models::*;
use crate::common::{constants::*, models::MyError};

pub async fn load_snark_fees(limit: Option<u64>) -> Result<SnarkFeesResponse, MyError> {
    if limit.is_none() {
        return Err(MyError::ParseError("Limit must not be None".into()));
    }
    let query_body = format!(
        r#"{{"query":"query SnarkFeesQuery(\n  $limit: Int = 100\n) {{\n  blocks(limit: $limit) {{\n    blockHeight\n    snarkFees\n    snarkJobs{{\n fee }}\n}}\n}}\n","variables":{{"limit": {}}},"operationName":"SnarkFeesQuery"}}"#,
        limit.unwrap()
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

pub async fn load_staker_leaderboard_data(
    epoch: Option<u32>,
    sort_by: StakerLeaderboardSort,
) -> Result<StakerLeaderboardResponse, MyError> {
    if epoch.is_none() {
        return Err(MyError::ParseError("Epoch must not be None".into()));
    }
    let query_body = format!(
        r#"{{"query":"query TopStakers($query: TopStakersQueryInput!, $limit: Int = 50, $sort_by: TopStakersSortByInput!) {{ topStakers(query: $query, limit: $limit, sortBy: $sort_by) {{ username public_key num_blocks_produced num_canonical_blocks_produced num_supercharged_blocks_produced num_slots_produced }} }}","variables":{{"limit": 50, "sort_by": "{}", "query": {{ "epoch": {} }} }},"operationName":"TopStakers"}}"#,
        sort_by,
        epoch.unwrap()
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
            .json::<StakerLeaderboardResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}
