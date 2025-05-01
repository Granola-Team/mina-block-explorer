use super::models::{BlockchainStatResponse, BlockchainSummary};
use crate::common::{
    constants::{GRAPHQL_ENDPOINT, REST_ENDPOINT},
    models::*,
};

pub async fn load_data() -> Result<BlockchainSummary, MyError> {
    let response = reqwest::get(format!("{}/summary", REST_ENDPOINT))
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlockchainSummary>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

pub async fn load_block_producers_stat(
    last_n_blocks: u64,
) -> Result<BlockchainStatResponse, MyError> {
    let query_body = format!(
        r#"{{"query":"query BlockProducersStat($limit: Int, $last_n_blocks: Int) {{ blocks(query: {{unique_block_producers_last_n_blocks: $last_n_blocks}}, limit: $limit) {{ num_unique_block_producers_last_n_blocks }}}}", "variables":{{"last_n_blocks":{}, "limit":{}}},"operationName":"BlockProducersStat"}}"#,
        last_n_blocks, 1,
    );
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlockchainStatResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}
