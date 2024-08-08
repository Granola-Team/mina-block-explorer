use serde::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlockAnalyticsData {
    pub epoch_num_blocks: i64,
    pub total_num_blocks: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkJob {
    fee: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnarkFeeData {
    pub block_height: i64,
    pub snark_fees: String,
    pub snark_jobs: Vec<SnarkJob>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkStatsContainer {
    pub all: SnarkStats,
    pub non_zero: SnarkStats,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkStats {
    pub count: usize,
    pub sum: usize,
    pub mean: Option<f64>,
}

impl SnarkStats {
    pub fn new(data: Vec<SnarkFeeData>) -> Self {
        let count = data.iter().fold(0, |count, d| count + d.snark_jobs.len());
        let sum = data.iter().fold(0, |count, d| {
            d.snark_fees
                .parse::<usize>()
                .ok()
                .map(|fees| count + fees)
                .unwrap_or(count)
        });
        Self {
            count,
            sum,
            mean: if count == 0 {
                None // Return None if division by zero
            } else {
                Some((sum as f64) / (count as f64))
            },
        }
    }
}

impl From<Vec<SnarkFeeData>> for SnarkStatsContainer {
    fn from(data: Vec<SnarkFeeData>) -> Self {
        let filtered_data = data
            .iter()
            .map(|snark_fee_data| SnarkFeeData {
                block_height: snark_fee_data.block_height,
                snark_fees: snark_fee_data.snark_fees.clone(),
                snark_jobs: snark_fee_data
                    .snark_jobs
                    .clone()
                    .into_iter()
                    .filter(|snark_job| snark_job.fee > 0)
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();
        SnarkStatsContainer {
            all: SnarkStats::new(data),
            non_zero: SnarkStats::new(filtered_data),
        }
    }
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
