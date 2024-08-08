use serde::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockAnalyticsData {
    pub epoch_num_blocks: i64,
    pub total_num_blocks: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnarkFeeData {
    pub block_height: i64,
    pub snark_fees: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlocksAnalyticsData {
    pub blocks: Vec<BlockAnalyticsData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkFeesData {
    pub blocks: Vec<SnarkFeeData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlocksAnalyticsResponse {
    pub data: BlocksAnalyticsData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkFeesResponse {
    pub data: SnarkFeesData,
}
