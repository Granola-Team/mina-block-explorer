use super::models::BlockchainSummary;
use crate::common::{constants::REST_ENDPOINT, models::*};

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
