use leptos::{HtmlElement, html};

use crate::common::functions::*;
use crate::common::components::*;
use super::models::*;

impl TableData for StakesResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Key"),
            String::from("Name"),
            String::from("Stake"),
            String::from("% Of Total Stake"),
            String::from("Block Win %"),
            String::from("Delegators"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        let mut rows = Vec::new();
        for stake in &self.data {
            let data = vec![
                stake.id.delegate.to_string(),
                stake.name.to_string(),
                stake.stake.to_string(),
                stake.percent_of_stake.to_string(),
                stake.block_chance.to_string(),
                stake.delegates.to_string(),
            ]
            .into_iter()
            .map(convert_to_span)
            .collect();
            rows.push(data);
        }
        rows
    }
}

