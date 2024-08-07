use serde::*;
use statrs::statistics::{Data, Distribution, OrderStatistics};

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
    pub sum: f64,
    pub mean: Option<f64>,
    pub median: f64,
    pub std_dev: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub lower_quartile: f64,
    pub upper_quartile: f64,
}

impl SnarkStats {
    pub fn new(data: Vec<SnarkFeeData>) -> Self {
        let snark_jobs: Vec<f64> = data
            .iter()
            .flat_map(|snark_data| snark_data.snark_jobs.iter().map(|j| j.fee as f64))
            .collect::<Vec<_>>();
        let snark_jobs_data = Data::new(snark_jobs.clone());
        Self {
            count: snark_jobs.len(),
            sum: snark_jobs.iter().sum(),
            mean: snark_jobs_data.mean(),
            median: snark_jobs_data.clone().median(),
            std_dev: snark_jobs_data.std_dev(),
            min: snark_jobs
                .clone()
                .into_iter()
                .reduce(|arg0, arg1| f32::min(arg0 as f32, arg1 as f32).into()),
            max: snark_jobs
                .into_iter()
                .reduce(|arg0, arg1| f32::max(arg0 as f32, arg1 as f32).into()),
            lower_quartile: snark_jobs_data.clone().lower_quartile(),
            upper_quartile: snark_jobs_data.clone().upper_quartile(),
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
