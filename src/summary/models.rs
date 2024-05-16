use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub blockchain_length: u32,
    pub circulating_supply: String,
    pub epoch: u32,
    pub slot: u32,
    pub staking_epoch_ledger_hash: String,
    pub total_currency: String,
}

impl BlockchainSummary {
    pub fn circ_supply(&self) -> f64 {
        self.circulating_supply
            .trim()
            .parse()
            .expect("Cannot parse circulating_supply")
    }
    pub fn tot_currency(&self) -> f64 {
        self.total_currency
            .trim()
            .parse()
            .expect("Cannot parse total_currency")
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
        };
        assert_eq!(bs.circ_supply(), 2345345.4312431243);
        assert_eq!(bs.tot_currency(), 1_105_297_372.840_039_3)
    }
}
