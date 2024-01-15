use super::models::BlockchainSummary;
use crate::common::models::*;

pub async fn load_data() -> Result<BlockchainSummary, MyError> {
    let response = reqwest::get("https://api.minaexplorer.com/summary")
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
