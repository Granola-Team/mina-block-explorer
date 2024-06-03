use super::{functions::*, graphql::staking_ledgers_query};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;
use staking_ledgers_query::StakingLedgersQueryStakes;

impl TableData for Vec<Option<StakingLedgersQueryStakes>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_stake| match opt_stake {
                Some(stake) => vec![
                    convert_to_link(
                        get_public_key(stake),
                        format!("/addresses/accounts/{}", get_public_key(stake)),
                    ),
                    decorate_with_mina_tag(get_stake(stake)),
                    convert_to_span(get_stake_percentage(stake)),
                    convert_to_link(
                        if get_public_key(stake) == get_delegate(stake) {
                            "Self".to_string()
                        } else {
                            get_delegate(stake)
                        },
                        format!("/addresses/accounts/{}", get_delegate(stake)),
                    ),
                    convert_to_pill(get_delegators_count(stake), ColorVariant::Blue),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
