use super::models::*;
use crate::common::models::*;

pub async fn load_data() -> Result<StakesResponse, MyError> {
    let response = reqwest::get("https://minaexplorer.com/staking-data/")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<StakesResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}