use crate::common::functions::*;
use crate::common::table::*;

use super::{functions::*, graphql::snarks_query};
use crate::common::models::*;
use leptos::*;
use snarks_query::SnarksQuerySnarks;

trait SnarksQuerySnarksContainer {}

impl SnarksQuerySnarksContainer for Vec<Option<SnarksQuerySnarks>> {}
impl SnarksQuerySnarksContainer for &[Option<SnarksQuerySnarks>] {}

impl<T> TableData for T
where
    T: AsRef<[Option<SnarksQuerySnarks>]> + SnarksQuerySnarksContainer,
{
    fn get_columns(&self) -> Vec<String> {
        ["Height", "Date", "Prover", "Work Ids", "State Hash", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.as_ref()
            .iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_span(get_block_height(snark)),
                    convert_to_span(get_date_time(snark)),
                    convert_to_link(
                        get_prover(snark),
                        format!("/accounts/{}", get_prover(snark)),
                    ),
                    convert_array_to_span(
                        get_work_ids(snark)
                            .iter()
                            .map(|w| convert_to_pill(w.to_string(), PillVariant::Grey))
                            .collect::<Vec<_>>(),
                    ),
                    convert_to_link(
                        get_block_state_hash(snark),
                        format!("/blocks/{}", get_block_state_hash(snark)),
                    ),
                    convert_to_pill(get_fee(snark), PillVariant::Orange),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
