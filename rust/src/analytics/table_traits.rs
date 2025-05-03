use super::models::{SnarkStats, SnarkStatsContainer, StakerStats, TopSnarkerStat};
use crate::common::{functions::*, table::TableData};
use leptos::*;

impl TableData for Vec<StakerStats> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|stat| {
                vec![
                    convert_array_to_span(vec![
                        convert_to_link(
                            stat.username.clone(),
                            format!("/addresses/accounts/{}/spotlight", stat.public_key),
                        ),
                        convert_to_copy_link(
                            stat.public_key.clone(),
                            format!("/addresses/accounts/{}/spotlight", stat.public_key),
                        )
                        .attr("class", "text-xs text-slate-400"),
                    ])
                    .attr("class", "flex flex-col items-start"),
                    convert_to_span(format_number(
                        stat.num_canonical_blocks_produced.to_string(),
                    )),
                    convert_to_span(format_number(
                        stat.num_supercharged_blocks_produced.to_string(),
                    )),
                    convert_to_span(format_number(stat.num_slots_produced.to_string())),
                    convert_to_span(format_number_for_html(
                        &stat
                            .orphan_rate()
                            .map(|r| format!("{r}%"))
                            .unwrap_or("n/a".to_string()),
                        5,
                    )),
                    convert_to_span(format_number_for_html(
                        &stat
                            .win_rate()
                            .map(|r| format!("{r}%"))
                            .unwrap_or("n/a".to_string()),
                        5,
                    )),
                ]
            })
            .collect()
    }
}

const TOP_SNARKER_FEE_MAX_LHS_DIGITS: usize = 3;
impl TableData for Vec<TopSnarkerStat> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|stat| {
                vec![
                    convert_to_span(stat.username.clone().unwrap_or("Unknown".to_string())),
                    convert_to_copy_link(
                        stat.public_key.clone(),
                        format!("/addresses/accounts/{}/spotlight", stat.public_key),
                    ),
                    convert_to_span(format_number_for_html(
                        nanomina_to_mina(stat.total_fees).as_str(),
                        TOP_SNARKER_FEE_MAX_LHS_DIGITS,
                    )),
                    convert_to_span(format_number_for_html(
                        nanomina_to_mina(stat.min_fee).as_str(),
                        TOP_SNARKER_FEE_MAX_LHS_DIGITS,
                    )),
                    convert_to_span(format_number_for_html(
                        nanomina_to_mina(stat.max_fee).as_str(),
                        TOP_SNARKER_FEE_MAX_LHS_DIGITS,
                    )),
                    convert_to_span(stat.snarks_sold.to_string()),
                ]
            })
            .collect()
    }
}

impl TableData for Option<SnarkStatsContainer> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.clone().map_or(vec![], |stats_cont| {
            vec![
                vec![
                    convert_to_span("Count".to_string()),
                    convert_to_span(format_number(stats_cont.all.get_count())),
                    convert_to_span(format_number(stats_cont.non_zero.get_count())),
                ],
                vec![
                    convert_to_span("Sum".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont.all.get_sum().parse::<u64>().ok().unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_sum()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("Mean".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont.all.get_mean().parse::<u64>().ok().unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_mean()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("Median".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont.all.get_median().parse::<u64>().ok().unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_median()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("Min".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont.all.get_min().parse::<u64>().ok().unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_min()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("Max".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont.all.get_max().parse::<u64>().ok().unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_max()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("25%".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .all
                            .get_lower_quartile()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_lower_quartile()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
                vec![
                    convert_to_span("75%".to_string()),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .all
                            .get_upper_quartile()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                    convert_to_span(nanomina_to_mina(
                        stats_cont
                            .non_zero
                            .get_upper_quartile()
                            .parse::<u64>()
                            .ok()
                            .unwrap_or(0),
                    )),
                ],
            ]
        })
    }
}

pub trait SnarkTableData {
    fn get_count(&self) -> String;
    fn get_sum(&self) -> String;
    fn get_mean(&self) -> String;
    fn get_median(&self) -> String;
    fn get_min(&self) -> String;
    fn get_max(&self) -> String;
    fn get_lower_quartile(&self) -> String;
    fn get_upper_quartile(&self) -> String;
}

impl SnarkTableData for SnarkStats {
    fn get_count(&self) -> String {
        self.count.to_string()
    }
    fn get_sum(&self) -> String {
        self.sum.to_string()
    }
    fn get_mean(&self) -> String {
        self.mean.map_or("-".to_string(), |mean| mean.to_string())
    }
    fn get_median(&self) -> String {
        self.median.to_string()
    }
    fn get_min(&self) -> String {
        self.min.map_or("-".to_string(), |min| min.to_string())
    }
    fn get_max(&self) -> String {
        self.max.map_or("-".to_string(), |max| max.to_string())
    }
    fn get_lower_quartile(&self) -> String {
        self.lower_quartile.to_string()
    }
    fn get_upper_quartile(&self) -> String {
        self.upper_quartile.to_string()
    }
}
