use crate::common::components::*;

use super::graphql::snarks_query;
use snarks_query::SnarksQuerySnarks;

impl TableData for Vec<Option<SnarksQuerySnarks>> {
    fn get_columns(&self) -> Vec<String> {
        ["Height", "Date", "Prover", "Work Ids", "State Hash", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    snark
                        .block_height
                        .map_or_else(String::new, |o| o.to_string()),
                    snark.date_time.map_or_else(String::new, |o| o.to_string()),
                    snark
                        .prover
                        .as_ref()
                        .map_or_else(String::new, |o| o.to_string()),
                    snark
                        .work_ids
                        .as_ref()
                        .map_or_else(String::new, |work_ids| {
                            work_ids
                                .iter()
                                .map(|o| o.map_or_else(String::new, |o1| o1.to_string()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }),
                    snark.block.as_ref().map_or_else(String::new, |o| {
                        o.state_hash
                            .as_ref()
                            .map_or_else(String::new, |o| o.to_string())
                    }),
                    snark.fee.map_or_else(String::new, |o| o.to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
