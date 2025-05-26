use crate::common::{functions::*, table::TableData};
use leptos::*;

use super::models::StakerStats;

impl TableData for Vec<StakerStats> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|stat| {
                vec![
                    convert_to_linkable_address(stat.username.clone(), stat.public_key.clone()),
                    convert_array_to_span(vec![
                        convert_to_span(format_number(
                            stat.num_canonical_blocks_produced.to_string(),
                        )),
                        convert_to_span(
                            stat.get_percent_of_canonical_blocks()
                                .map(|r| format!("({r}%)"))
                                .unwrap_or("n/a".to_string()),
                        )
                        .attr("class", "w-20 text-slate-400 flex justify-end"),
                    ]),
                    convert_to_span(format_number_for_html(
                        &format!("{}%", stat.delegation_totals.total_stake_percentage),
                        5,
                    )),
                    convert_array_to_span(vec![
                        convert_to_span(format_number(stat.num_slots_produced.to_string())),
                        convert_to_span(
                            stat.get_percent_of_produced_slots()
                                .map(|r| format!("({r}%)"))
                                .unwrap_or("n/a".to_string()),
                        )
                        .attr("class", "w-20 text-slate-400 flex justify-end"),
                    ]),
                    convert_to_span(format_number_for_html(
                        &stat
                            .orphan_rate()
                            .map(|r| format!("{r}%"))
                            .unwrap_or("n/a".to_string()),
                        5,
                    )),
                    convert_to_span(format_number(
                        stat.num_supercharged_blocks_produced.to_string(),
                    )),
                ]
            })
            .collect()
    }
}
