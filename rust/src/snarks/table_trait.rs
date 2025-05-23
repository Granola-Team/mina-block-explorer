use super::{functions::*, graphql::snarks_query};
use crate::common::{functions::*, models::HasBlockHeight, table::*};
use leptos::*;
use snarks_query::SnarksQuerySnarks;

impl TableData for Vec<Option<SnarksQuerySnarks>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_span(get_block_height(snark)),
                    convert_to_copy_link(
                        get_block_state_hash(snark),
                        format!("/blocks/{}/snark-jobs", get_block_state_hash(snark)),
                    ),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&get_date_time(snark)),
                        get_date_time(snark),
                    ),
                    convert_to_linkable_address(get_prover_username(snark), get_prover(snark)),
                    convert_to_span(get_fee(snark)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl HasBlockHeight for SnarksQuerySnarks {
    fn block_height(&self) -> Option<i64> {
        self.block_height
    }
}
