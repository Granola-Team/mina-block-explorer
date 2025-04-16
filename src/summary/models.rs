use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ChainSummary {
    pub latest_epoch: u64,
    pub latest_slot: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub chain_id: String,
    pub genesis_state_hash: Option<String>,
    pub blockchain_length: u64,
    pub circulating_supply: String,
    #[deprecated(note = "use chain.latest_epoch instead")]
    pub epoch: u64,
    #[deprecated(note = "use chain.latest_slot instead")]
    pub slot: u64,
    pub global_slot: i64,
    pub staking_epoch_ledger_hash: String,
    pub total_currency: String,
    pub total_num_blocks: u64,
    pub total_num_snarks: i64,
    pub total_num_canonical_snarks: u64,
    pub total_num_user_commands: i64,
    pub epoch_num_internal_commands: u64,
    pub total_num_internal_commands: i64,
    pub total_num_canonical_internal_commands: u64,
    pub total_num_applied_user_commands: u64,
    pub total_num_failed_user_commands: u64,
    pub total_num_canonical_user_commands: u64,
    pub total_num_applied_canonical_user_commands: u64,
    pub total_num_failed_canonical_user_commands: u64,
    pub total_num_zkapp_accounts: u64,
    pub epoch_num_zkapp_commands: u64,
    pub total_num_zkapp_commands: u64,
    pub total_num_applied_zkapp_commands: u64,
    pub total_num_failed_zkapp_commands: u64,
    pub total_num_canonical_zkapp_commands: u64,
    pub total_num_applied_canonical_zkapp_commands: u64,
    pub total_num_failed_canonical_zkapp_commands: u64,
    pub total_num_accounts: u64,
    pub indexer_version: String,
    pub chain: Option<HashMap<String, ChainSummary>>, // Keyed by chain ID hash
}

impl BlockchainSummary {
    pub fn get_total_num_non_canonical_applied_zkapp_commands(&self) -> u64 {
        self.total_num_applied_zkapp_commands - self.total_num_applied_canonical_zkapp_commands
    }

    pub fn get_total_num_non_canonical_failed_zkapp_commands(&self) -> u64 {
        self.total_num_failed_zkapp_commands - self.total_num_failed_canonical_zkapp_commands
    }

    pub fn get_total_num_non_canonical_applied_user_commands(&self) -> u64 {
        self.total_num_applied_user_commands - self.total_num_applied_canonical_user_commands
    }

    pub fn get_total_num_non_canonical_failed_user_commands(&self) -> u64 {
        self.total_num_failed_user_commands - self.total_num_failed_canonical_user_commands
    }

    pub fn get_total_num_non_canonical_blocks(&self) -> u64 {
        // when ingesting from hardfork, total number of blocks is less than blockchain
        // length therefore we use saturating_sub, to accomodate automated tests
        self.total_num_blocks.saturating_sub(self.blockchain_length)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BlockchainStatData {
    pub blocks: Vec<BlockchainStat>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BlockchainStatResponse {
    pub data: BlockchainStatData,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BlockchainStat {
    pub num_unique_block_producers_last_n_blocks: u64,
}

impl BlockchainSummary {
    pub fn circ_supply(&self) -> f64 {
        self.circulating_supply.trim().parse().map_or(0.0, |r| r)
    }
    pub fn tot_currency(&self) -> f64 {
        self.total_currency.trim().parse().map_or(0.0, |r| r)
    }
}

#[cfg(test)]
mod float_tests {

    use super::*;

    #[test]
    fn test_parsing_floats() {
        let bs = BlockchainSummary {
            circulating_supply: "2345345.4312431243".to_owned(),
            total_currency: "1105297372.840039233".to_owned(),
            ..Default::default()
        };
        assert_eq!(bs.circ_supply(), 2345345.4312431243);
        assert_eq!(bs.tot_currency(), 1_105_297_372.840_039_3)
    }
}

#[cfg(test)]
mod blockchain_summary_tests {
    use super::*;

    // Helper function to create a sample BlockchainSummary
    fn create_sample_summary() -> BlockchainSummary {
        BlockchainSummary {
            total_num_applied_zkapp_commands: 120,
            total_num_applied_canonical_zkapp_commands: 100,
            total_num_failed_zkapp_commands: 30,
            total_num_failed_canonical_zkapp_commands: 20,
            total_num_applied_user_commands: 250,
            total_num_applied_canonical_user_commands: 230,
            total_num_failed_user_commands: 50,
            total_num_failed_canonical_user_commands: 40,
            ..BlockchainSummary::default()
        }
    }

    #[test]
    fn test_get_total_num_non_canonical_applied_zkapp_commands() {
        let summary = create_sample_summary();
        let result = summary.get_total_num_non_canonical_applied_zkapp_commands();
        assert_eq!(result, 120 - 100); // 20
    }

    #[test]
    fn test_get_total_num_non_canonical_failed_zkapp_commands() {
        let summary = create_sample_summary();
        let result = summary.get_total_num_non_canonical_failed_zkapp_commands();
        assert_eq!(result, 30 - 20); // 10
    }

    #[test]
    fn test_get_total_num_non_canonical_applied_user_commands() {
        let summary = create_sample_summary();
        let result = summary.get_total_num_non_canonical_applied_user_commands();
        assert_eq!(result, 250 - 230); // 20
    }

    #[test]
    fn test_get_total_num_non_canonical_failed_user_commands() {
        let summary = create_sample_summary();
        let result = summary.get_total_num_non_canonical_failed_user_commands();
        assert_eq!(result, 50 - 40); // 10
    }
}
