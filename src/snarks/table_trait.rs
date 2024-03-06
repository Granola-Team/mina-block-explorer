use super::{functions::*, graphql::snarks_query};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;
use snarks_query::SnarksQuerySnarks;

impl TableData for Vec<Option<SnarksQuerySnarks>> {
    fn get_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Age", "Prover", "Work Ids", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_span(get_block_height(snark)),
                    convert_to_link(
                        get_block_state_hash(snark),
                        format!("/blocks/{}", get_block_state_hash(snark)),
                    ),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_date_time(snark))),
                        convert_to_span(get_date_time(snark))
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_link(
                        get_prover(snark),
                        format!("/addresses/accounts/{}", get_prover(snark)),
                    ),
                    convert_array_to_span(
                        get_work_ids(snark)
                            .iter()
                            .map(|w| convert_to_pill(w.to_string(), PillVariant::Grey))
                            .collect::<Vec<_>>(),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_fee(snark), "mina".to_string()),
                        PillVariant::Orange,
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
