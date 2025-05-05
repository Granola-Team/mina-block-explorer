use super::models::*;
use crate::common::{constants::*, models::MyError};

pub async fn load_snark_fees(
    blockheight_lte: Option<u64>,
    blockheight_gte: Option<u64>,
) -> Result<SnarkFeesResponse, MyError> {
    if blockheight_lte.is_none() || blockheight_gte.is_none() {
        return Err(MyError::ParseError("Block limits must be set".into()));
    }
    let query_body = format!(
        r#"{{"query":"query SnarkFeesQuery(\n  $limit: Int = 100\n, $query: BlockQueryInput!) {{\n  blocks(limit: $limit, query: $query) {{\n    blockHeight\n    snarkFees\n    snarkJobs{{\n fee }}\n}}\n}}\n","variables":{{"limit": 10000000, "query": {{ "blockHeight_lte": {}, "blockHeight_gte": {} }} }},"operationName":"SnarkFeesQuery"}}"#,
        blockheight_lte.unwrap(),
        blockheight_gte.unwrap()
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

pub async fn load_snarker_leaderboard_data(
    epoch: Option<u32>,
    sort_by_total_fees_opt: Option<SnarkerLeaderboardTotalFees>,
    sort_by_highest_fee_opt: Option<SnarkerLeaderboardHighestFees>,
) -> Result<SnarkerLeaderboardResponse, MyError> {
    if epoch.is_none() {
        return Err(MyError::ParseError("Epoch must not be None".into()));
    }
    let query_body = format!(
        r#"{{"query":"query TopSnarkers($query: TopSnarkersQueryInput!, $limit: Int = 50, $sort_by: TopSnarkersSortByInput!) {{ topSnarkers(query: $query, limit: $limit, sortBy: $sort_by) {{ username public_key total_fees min_fee max_fee snarks_sold }} }}","variables":{{"limit": 50, "sort_by": "{}", "query": {{ "epoch": {} }} }},"operationName":"TopSnarkers"}}"#,
        sort_by_total_fees_opt
            .map(|s| s.to_string())
            .or_else(|| sort_by_highest_fee_opt.map(|s| s.to_string()))
            .unwrap_or(SnarkerLeaderboardHighestFees::HighestFeeDesc.to_string()),
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
            .json::<SnarkerLeaderboardResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

pub async fn load_staker_leaderboard_data(
    epoch: Option<u32>,
    sort_by: StakerLeaderboardCanonicalBlocks,
) -> Result<StakerLeaderboardResponse, MyError> {
    if epoch.is_none() {
        return Err(MyError::ParseError("Epoch must not be None".into()));
    }
    let query_body = format!(
        r#"{{"query":"query TopStakers($query: TopStakersQueryInput!, $blocks_query: BlockQueryInput!, $limit: Int = 50, $sort_by: TopStakersSortByInput!) {{ blocks(limit: 1, query: $blocks_query) {{ epoch_num_canonical_blocks epoch_num_blocks }} topStakers(query: $query, limit: $limit, sortBy: $sort_by) {{ username public_key num_blocks_produced num_canonical_blocks_produced num_supercharged_blocks_produced num_slots_produced }} }}","variables":{{"limit": 50, "sort_by": "{}", "query": {{ "epoch": {} }}, "blocks_query": {{ "protocolState": {{ "consensusState": {{ "epoch": {} }} }} }} }},"operationName":"TopStakers"}}"#,
        sort_by,
        epoch.unwrap(),
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
