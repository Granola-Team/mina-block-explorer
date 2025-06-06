use super::{functions::*, graphql::staking_ledgers_query};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;
use staking_ledgers_query::StakingLedgersQueryStakes;

impl TableData for Vec<Option<StakingLedgersQueryStakes>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_stake| match opt_stake {
                Some(stake) => vec![
                    convert_to_linkable_address(get_username(stake), get_public_key(stake)),
                    convert_to_span(get_balance(stake)),
                    convert_to_span(get_stake(stake)),
                    convert_to_span(get_stake_percentage(stake)),
                    convert_to_span(get_slot_win_likelihood(stake)),
                    convert_to_pill(get_delegators_count(stake), ColorVariant::Blue),
                    convert_to_linkable_address(get_delegate_username(stake), get_delegate(stake)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
