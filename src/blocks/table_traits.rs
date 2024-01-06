use super::functions::*;
use super::graphql::blocks_query::BlocksQueryBlocks;
use super::models::*;
use crate::common::components::*;
use std::collections::HashMap;

fn shared_get_columns() -> Vec<String> {
    vec![
        String::from("Height"),
        String::from("Date"),
        String::from("Block Producer"),
        String::from("Coinbase"),
        String::from("Transactions"),
        String::from("SNARKs"),
        String::from("Slot"),
        String::from("State Hash"),
        String::from("Coinbase Receiver"),
    ]
}

impl TableData for Vec<Option<BlocksQueryBlocks>> {
    fn get_columns(&self) -> Vec<String> {
        shared_get_columns()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|opt_blocks| match opt_blocks {
                Some(block) => vec![
                    get_block_height(block),
                    get_date_time(block),
                    get_creator_account(block),
                    get_coinbase(block),
                    get_transaction_count(block),
                    get_snark_job_count(block),
                    get_slot(block),
                    get_state_hash(block),
                    get_coinbase_receiver(block),
                ],
                None => vec![],
            })
            .collect()
    }

    fn get_linkable_cols(&self) -> HashMap<i32, String> {
        let mut linkcols: HashMap<i32, String> = HashMap::new();
        linkcols.insert(2, "/blocks/accounts/:token".to_owned());
        linkcols.insert(8, "/blocks/accounts/:token".to_owned());
        linkcols
    }
}

impl TableData for SummaryPageBlocksQueryBlocks {
    fn get_columns(&self) -> Vec<String> {
        shared_get_columns()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.0
            .iter()
            .filter_map(|block_opt| block_opt.as_ref())
            .map(|block| {
                vec![
                    get_block_height(block),
                    get_date_time(block),
                    get_creator_account(block),
                    get_coinbase(block),
                    get_transaction_count(block),
                    get_snark_job_count(block),
                    get_slot(block),
                    get_state_hash(block),
                    get_coinbase_receiver(block),
                ]
            })
            .collect()
    }

    fn get_linkable_cols(&self) -> HashMap<i32, String> {
        let mut linkcols: HashMap<i32, String> = HashMap::new();
        linkcols.insert(2, "/summary/accounts/:token".to_owned());
        linkcols.insert(8, "/summary/accounts/:token".to_owned());
        linkcols
    }
}
