use super::models::{SnarkStats, SnarkStatsContainer};
use crate::common::{
    functions::*,
    table::TableData,
};
use leptos::*;

impl TableData for Option<SnarkStatsContainer> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.clone().map_or(vec![], |stats_cont| {
            vec![
                vec![
                    convert_to_span("Count".to_string()),
                    convert_to_span(stats_cont.all.get_count()),
                    convert_to_span(stats_cont.non_zero.get_count()),
                ],
                vec![
                    convert_to_span("Sum".to_string()),
                    convert_to_span(stats_cont.all.get_sum()),
                    convert_to_span(stats_cont.non_zero.get_sum()),
                ],
                vec![
                    convert_to_span("Mean".to_string()),
                    convert_to_span(stats_cont.all.get_mean()),
                    convert_to_span(stats_cont.non_zero.get_mean()),
                ],
            ]
        })
    }
}

pub trait SnarkTableData {
    fn get_count(&self) -> String;
    fn get_sum(&self) -> String;
    fn get_mean(&self) -> String;
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
}
