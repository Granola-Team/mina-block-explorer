use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub blockchain_length: u32,
    pub circulating_supply: String,
    pub epoch: u32,
    pub slot: u32,
    pub global_slot: i64,
    pub staking_epoch_ledger_hash: String,
    pub total_currency: String,
    pub total_num_blocks: i64,
    pub total_num_snarks: i64,
    pub total_num_user_commands: i64,
    pub total_num_internal_commands: i64,
    pub total_num_accounts: i64,
    pub indexer_version: String,
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
            blockchain_length: 314394,
            epoch: 67,
            slot: 4194,
            staking_epoch_ledger_hash: "jxKCrryFrvzBE4iUURcS9zNTKcRdejiE9K28Bqcu7Us7RQqNfdL"
                .to_owned(),
            total_currency: "1105297372.840039233".to_owned(),
            total_num_blocks: 1000,
            total_num_snarks: 1000,
            total_num_user_commands: 1000,
            total_num_internal_commands: 1000,
            total_num_accounts: 1000,
            global_slot: 1,
            indexer_version: "v1".to_string(),
        };
        assert_eq!(bs.circ_supply(), 2345345.4312431243);
        assert_eq!(bs.tot_currency(), 1_105_297_372.840_039_3)
    }
}
