
use serde::{Deserialize, Serialize};

use crate::table::TableData;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SnarksResponse {
    data: Data
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Data {
    snarks: Vec<Snark>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Snark {
    workIds: Vec<u64>,
    block: Block,
    blockHeight: u64,
    dateTime: String,
    fee: u64,
    prover: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
struct Block {
    stateHash: String
}

impl TableData for SnarksResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Height"),
            String::from("Date"),
            String::from("Prover"),
            String::from("Work Ids"),
            String::from("State Hash"),
            String::from("Fee"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        for snark in &self.data.snarks {
            let data = vec![
                snark.blockHeight.to_string(),
                snark.dateTime.to_string(),
                snark.prover.to_string(),
                snark.workIds.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "),
                snark.block.stateHash.to_string(),
                snark.fee.to_string()
            ];
            rows.push(data);
        }
        rows
    }
}