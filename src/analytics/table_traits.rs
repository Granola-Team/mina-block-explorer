use super::models::SnarkStatsContainer;
use crate::common::{functions::convert_to_span, table::TableData};
use leptos::*;

impl TableData for Option<SnarkStatsContainer> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.clone().map_or(vec![], |stats_cont| {
            vec![vec![
                convert_to_span("Count".to_string()),
                convert_to_span(stats_cont.get_all_count()),
                convert_to_span(stats_cont.get_non_zero_count()),
            ]]
        })
    }
}

pub trait SnarkTableData {
    fn get_all_count(&self) -> String;
    fn get_non_zero_count(&self) -> String;
}

impl SnarkTableData for SnarkStatsContainer {
    fn get_all_count(&self) -> String {
        self.all.count.to_string()
    }
    fn get_non_zero_count(&self) -> String {
        self.non_zero.count.to_string()
    }
}
