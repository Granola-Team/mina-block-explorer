use super::{functions::*, graphql::stakes_query};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;
use stakes_query::StakesQueryStakes;

impl TableData for Vec<Option<StakesQueryStakes>> {
    fn get_columns(&self) -> Vec<String> {
        ["Key", "Stake", "Delegate", "Delegators", "Ledger Hash"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_stake| match opt_stake {
                Some(stake) => vec![
                    convert_to_link(
                        get_public_key(stake),
                        format!("/addresses/accounts/{}", get_public_key(stake)),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_balance(stake), "mina".to_string()),
                        PillVariant::Green,
                    ),
                    convert_to_link(
                        if get_public_key(stake) == get_delegate(stake) {
                            "Self".to_string()
                        } else {
                            get_delegate(stake)
                        },
                        format!("/addresses/accounts/{}", get_delegate(stake)),
                    ),
                    convert_to_pill(get_delegators_count(stake), PillVariant::Blue),
                    convert_to_span(get_ledger_hash(stake)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
