use crate::common::table::{AnySort, CycleSort, SortDirection};
use serde::*;
use statrs::statistics::{Data, Distribution, OrderStatistics};
use std::fmt;

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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct StakerStats {
    pub username: String,
    pub public_key: String,
    pub num_blocks_produced: u32,
    pub num_canonical_blocks_produced: u32,
    pub num_supercharged_blocks_produced: u32,
    pub num_slots_produced: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopStakers {
    pub top_stakers: Vec<StakerStats>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StakerLeaderboardResponse {
    pub data: TopStakers,
}

impl fmt::Display for StakerLeaderboardCanonicalBlocks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedAsc => {
                write!(f, "NUM_CANONICAL_BLOCKS_PRODUCED_ASC")
            }
            StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc => {
                write!(f, "NUM_CANONICAL_BLOCKS_PRODUCED_DESC")
            }
        }
    }
}

impl StakerStats {
    pub fn orphan_rate(&self) -> Option<String> {
        if self.num_blocks_produced == 0 {
            return None;
        }

        let num_orphans = self
            .num_slots_produced
            .checked_sub(self.num_canonical_blocks_produced)?;

        let numerator = num_orphans.checked_mul(10000)?;

        let orphan_rate_scaled = numerator.checked_div(self.num_slots_produced)?;

        let whole = orphan_rate_scaled / 100;
        let frac = orphan_rate_scaled % 100;
        Some(format!("{}.{:02}", whole, frac))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TopSnarkerStat {
    pub username: Option<String>,
    pub public_key: String,
    pub total_fees: u64,
    pub min_fee: u64,
    pub max_fee: u64,
    pub snarks_sold: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopSnarkers {
    pub top_snarkers: Vec<TopSnarkerStat>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnarkerLeaderboardResponse {
    pub data: TopSnarkers,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum SnarkerLeaderboardTotalFees {
    TotalFeesAsc,
    TotalFeesDesc,
    Nil,
}

impl SortDirection for SnarkerLeaderboardTotalFees {
    fn is_desc(&self) -> bool {
        matches!(self, SnarkerLeaderboardTotalFees::TotalFeesDesc)
    }
    fn is_active(&self) -> bool {
        !matches!(self, SnarkerLeaderboardTotalFees::Nil)
    }
}

impl fmt::Display for SnarkerLeaderboardTotalFees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnarkerLeaderboardTotalFees::TotalFeesAsc => {
                write!(f, "TOTAL_FEES_ASC")
            }
            SnarkerLeaderboardTotalFees::TotalFeesDesc => {
                write!(f, "TOTAL_FEES_DESC")
            }
            SnarkerLeaderboardTotalFees::Nil => {
                write!(f, "")
            }
        }
    }
}

impl CycleSort for SnarkerLeaderboardTotalFees {
    fn cycle(&self) -> AnySort {
        match self {
            SnarkerLeaderboardTotalFees::Nil => {
                AnySort::SnarkerLeaderboardTotalFees(SnarkerLeaderboardTotalFees::TotalFeesDesc)
            }
            SnarkerLeaderboardTotalFees::TotalFeesDesc => {
                AnySort::SnarkerLeaderboardTotalFees(SnarkerLeaderboardTotalFees::TotalFeesAsc)
            }
            SnarkerLeaderboardTotalFees::TotalFeesAsc => {
                AnySort::SnarkerLeaderboardTotalFees(SnarkerLeaderboardTotalFees::Nil)
            }
        }
    }
}
impl TryFrom<String> for SnarkerLeaderboardTotalFees {
    type Error = &'static str;
    fn try_from(str: String) -> Result<SnarkerLeaderboardTotalFees, Self::Error> {
        match str.as_str() {
            "TOTAL_FEES_ASC" => Ok(SnarkerLeaderboardTotalFees::TotalFeesAsc),
            "TOTAL_FEES_DESC" => Ok(SnarkerLeaderboardTotalFees::TotalFeesDesc),
            _ => Err("Unable to parse the AccountsSort from string"),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum SnarkerLeaderboardHighestFees {
    HighestFeeAsc,
    HighestFeeDesc,
    Nil,
}

impl SortDirection for SnarkerLeaderboardHighestFees {
    fn is_desc(&self) -> bool {
        matches!(self, SnarkerLeaderboardHighestFees::HighestFeeDesc)
    }
    fn is_active(&self) -> bool {
        !matches!(self, SnarkerLeaderboardHighestFees::Nil)
    }
}

impl fmt::Display for SnarkerLeaderboardHighestFees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnarkerLeaderboardHighestFees::HighestFeeAsc => {
                write!(f, "MAX_FEE_ASC")
            }
            SnarkerLeaderboardHighestFees::HighestFeeDesc => {
                write!(f, "MAX_FEE_DESC")
            }
            SnarkerLeaderboardHighestFees::Nil => {
                write!(f, "")
            }
        }
    }
}

impl CycleSort for SnarkerLeaderboardHighestFees {
    fn cycle(&self) -> AnySort {
        match self {
            SnarkerLeaderboardHighestFees::Nil => {
                AnySort::SnarkerLeaderboardHighestFee(SnarkerLeaderboardHighestFees::HighestFeeDesc)
            }
            SnarkerLeaderboardHighestFees::HighestFeeDesc => {
                AnySort::SnarkerLeaderboardHighestFee(SnarkerLeaderboardHighestFees::HighestFeeAsc)
            }
            SnarkerLeaderboardHighestFees::HighestFeeAsc => {
                AnySort::SnarkerLeaderboardHighestFee(SnarkerLeaderboardHighestFees::Nil)
            }
        }
    }
}
impl TryFrom<String> for SnarkerLeaderboardHighestFees {
    type Error = &'static str;
    fn try_from(str: String) -> Result<SnarkerLeaderboardHighestFees, Self::Error> {
        match str.as_str() {
            "MAX_FEE_ASC" => Ok(SnarkerLeaderboardHighestFees::HighestFeeAsc),
            "MAX_FEE_DESC" => Ok(SnarkerLeaderboardHighestFees::HighestFeeDesc),
            _ => Err("Unable to parse the AccountsSort from string"),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum StakerLeaderboardCanonicalBlocks {
    NumberOfCanonicalBlocksProducedDesc,
    NumberOfCanonicalBlocksProducedAsc,
}

impl SortDirection for StakerLeaderboardCanonicalBlocks {
    fn is_desc(&self) -> bool {
        matches!(
            self,
            StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc
        )
    }
    fn is_active(&self) -> bool {
        true
    }
}

impl CycleSort for StakerLeaderboardCanonicalBlocks {
    fn cycle(&self) -> AnySort {
        match self {
            StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc => {
                AnySort::StakerLeaderboardCanonicalBlocks(
                    StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedAsc,
                )
            }
            StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedAsc => {
                AnySort::StakerLeaderboardCanonicalBlocks(
                    StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc,
                )
            }
        }
    }
}

impl TryFrom<String> for StakerLeaderboardCanonicalBlocks {
    type Error = &'static str;
    fn try_from(str: String) -> Result<StakerLeaderboardCanonicalBlocks, Self::Error> {
        match str.as_str() {
            "NUM_CANONICAL_BLOCKS_PRODUCED_ASC" => {
                Ok(StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedAsc)
            }
            "NUM_CANONICAL_BLOCKS_PRODUCED_DESC" => {
                Ok(StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc)
            }
            _ => Err("Unable to parse the StakerLeaderboardCanonicalBlocks from string"),
        }
    }
}

#[cfg(test)]
mod orphan_rate_tests {
    use super::*;

    #[test]
    fn test_orphan_rate_zero_blocks_produced() {
        let stats = StakerStats {
            num_blocks_produced: 0,
            num_canonical_blocks_produced: 0,
            num_slots_produced: 100,
            ..Default::default()
        };
        assert_eq!(stats.orphan_rate(), None);
    }

    #[test]
    fn test_orphan_rate_zero_slots_produced() {
        let stats = StakerStats {
            num_blocks_produced: 100,
            num_canonical_blocks_produced: 50,
            num_slots_produced: 0,
            ..Default::default()
        };
        // Division by zero in checked_div
        assert_eq!(stats.orphan_rate(), None);
    }

    #[test]
    fn test_orphan_rate_all_canonical() {
        let stats = StakerStats {
            num_blocks_produced: 100,
            num_canonical_blocks_produced: 100,
            num_slots_produced: 100,
            ..Default::default()
        };
        // (100 - 100) / 100 * 100 = 0%
        assert_eq!(stats.orphan_rate(), Some("0.00".to_string()));
    }

    #[test]
    fn test_orphan_rate_all_orphans() {
        let stats = StakerStats {
            num_blocks_produced: 100,
            num_canonical_blocks_produced: 0,
            num_slots_produced: 100,
            ..Default::default()
        };
        // (100 - 0) / 100 * 100 = 100%
        assert_eq!(stats.orphan_rate(), Some("100.00".to_string()));
    }

    #[test]
    fn test_orphan_rate_partial_orphans() {
        let stats = StakerStats {
            num_blocks_produced: 100,
            num_canonical_blocks_produced: 80,
            num_slots_produced: 100,
            ..Default::default()
        };
        // (100 - 80) / 100 * 100 = 20%
        assert_eq!(stats.orphan_rate(), Some("20.00".to_string()));
    }

    #[test]
    fn test_orphan_rate_slots_different_from_blocks() {
        let stats = StakerStats {
            num_blocks_produced: 50,
            num_canonical_blocks_produced: 80,
            num_slots_produced: 100,
            ..Default::default()
        };
        // (100 - 80) / 100 * 100 = 20%
        assert_eq!(stats.orphan_rate(), Some("20.00".to_string()));
    }

    #[test]
    fn test_orphan_rate_underflow() {
        let stats = StakerStats {
            num_blocks_produced: 100,
            num_canonical_blocks_produced: 101,
            num_slots_produced: 100,
            ..Default::default()
        };
        // num_slots_produced < num_canonical_blocks_produced causes underflow
        assert_eq!(stats.orphan_rate(), None);
    }

    #[test]
    fn test_orphan_rate_small_numbers() {
        let stats = StakerStats {
            num_blocks_produced: 3,
            num_canonical_blocks_produced: 2,
            num_slots_produced: 3,
            ..Default::default()
        };
        // (3 - 2) / 3 * 100 ≈ 33.33%
        assert_eq!(stats.orphan_rate(), Some("33.33".to_string()));
    }

    #[test]
    fn test_orphan_rate_large_numbers() {
        let stats = StakerStats {
            num_blocks_produced: 1_000_000,
            num_canonical_blocks_produced: 999_000,
            num_slots_produced: 1_000_000,
            ..Default::default()
        };
        // (1,000,000 - 999,000) / 1,000,000 * 100 = 0.1%
        assert_eq!(stats.orphan_rate(), Some("0.10".to_string()));
    }
}
