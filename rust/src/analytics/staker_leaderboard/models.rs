use crate::common::{
    functions::round_to_two_decimals,
    table::{AnySort, CycleSort, SortDirection},
};

use super::graphql::top_stakers_query::TopStakersSortByInput;
use serde::*;
use std::fmt;

impl TryFrom<String> for TopStakersSortByInput {
    type Error = &'static str;
    fn try_from(str: String) -> Result<TopStakersSortByInput, Self::Error> {
        match str.as_str() {
            "NUM_CANONICAL_BLOCKS_PRODUCED_ASC" => {
                Ok(TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_ASC)
            }
            "NUM_CANONICAL_BLOCKS_PRODUCED_DESC" => {
                Ok(TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC)
            }
            _ => Err("Unable to parse the TopStakersSortByInput from string"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DelegationTotals {
    pub total_stake_percentage: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct StakerStats {
    pub username: Option<String>,
    pub public_key: String,
    pub num_blocks_produced: u32,
    pub num_canonical_blocks_produced: u32,
    pub num_supercharged_blocks_produced: u32,
    pub num_slots_produced: u32,
    pub epoch_num_canonical_blocks: Option<u32>,
    pub epoch_num_blocks: Option<u32>,
    pub epoch_num_slots_produced: Option<u32>,
    pub delegation_totals: DelegationTotals,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TopStakerBlocks {
    pub epoch_num_canonical_blocks: u32,
    pub epoch_num_blocks: u32,
    pub epoch_num_slots_produced: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopStakers {
    pub top_stakers: Vec<StakerStats>,
    pub blocks: Vec<TopStakerBlocks>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StakerLeaderboardResponse {
    pub data: TopStakers,
}

impl SortDirection for TopStakersSortByInput {
    fn is_desc(&self) -> bool {
        matches!(
            self,
            TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC
        ) || matches!(self, TopStakersSortByInput::NUM_SLOTS_PRODUCED_DESC)
    }
    fn is_active(&self) -> bool {
        true
    }
}

impl CycleSort for TopStakersSortByInput {
    fn cycle(&self) -> AnySort {
        match self {
            TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC => {
                AnySort::TopStakersSortByInput(
                    TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_ASC,
                )
            }
            TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_ASC => {
                AnySort::TopStakersSortByInput(
                    TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC,
                )
            }
            TopStakersSortByInput::NUM_SLOTS_PRODUCED_ASC => {
                AnySort::TopStakersSortByInput(TopStakersSortByInput::NUM_SLOTS_PRODUCED_DESC)
            }
            TopStakersSortByInput::NUM_SLOTS_PRODUCED_DESC => {
                AnySort::TopStakersSortByInput(TopStakersSortByInput::NUM_SLOTS_PRODUCED_ASC)
            }
            &Self::Other(_) => AnySort::TopStakersSortByInput(
                TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC,
            ),
        }
    }
}

impl fmt::Display for TopStakersSortByInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_ASC => {
                write!(f, "NUM_CANONICAL_BLOCKS_PRODUCED_ASC")
            }
            TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC => {
                write!(f, "NUM_CANONICAL_BLOCKS_PRODUCED_DESC")
            }
            TopStakersSortByInput::NUM_SLOTS_PRODUCED_ASC => {
                write!(f, "NUM_SLOTS_PRODUCED_ASC")
            }
            TopStakersSortByInput::NUM_SLOTS_PRODUCED_DESC => {
                write!(f, "NUM_SLOTS_PRODUCED_DESC")
            }
            &Self::Other(_) => {
                write!(f, "NUM_CANONICAL_BLOCKS_PRODUCED_DESC")
            }
        }
    }
}

impl StakerStats {
    pub fn orphan_rate(&self) -> Option<String> {
        if self.num_slots_produced == 0 {
            return None;
        }

        let num_orphans = self
            .num_slots_produced
            .checked_sub(self.num_canonical_blocks_produced)?;

        let orphan_rate = num_orphans as f64 / self.num_slots_produced as f64 * 100.0;
        round_to_two_decimals(orphan_rate)
    }

    pub fn get_percent_of_canonical_blocks(&self) -> Option<String> {
        let total_blocks = self.epoch_num_canonical_blocks?;
        if total_blocks == 0 {
            return None;
        }

        let percent_of_blocks_produced =
            self.num_canonical_blocks_produced as f64 / total_blocks as f64 * 100.0;
        round_to_two_decimals(percent_of_blocks_produced)
    }

    pub fn get_percent_of_produced_slots(&self) -> Option<String> {
        let total_slots = self.epoch_num_slots_produced?;
        if total_slots == 0 {
            return None;
        }
        let percent_of_winnable_slots = self.num_slots_produced as f64 / total_slots as f64 * 100.0;
        round_to_two_decimals(percent_of_winnable_slots)
    }
}

#[cfg(test)]
mod orphan_rate_tests {
    use super::*;

    #[test]
    fn test_orphan_rate_zero_slots_produced() {
        let stats = StakerStats {
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
            num_canonical_blocks_produced: 999_000,
            num_slots_produced: 1_000_000,
            ..Default::default()
        };
        // (1,000,000 - 999,000) / 1,000,000 * 100 = 0.1%
        assert_eq!(stats.orphan_rate(), Some("0.10".to_string()));
    }
}
