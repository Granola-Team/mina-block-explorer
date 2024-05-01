use super::{functions::*, graphql::next_staking_ledgers_query};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;
use next_staking_ledgers_query::NextStakingLedgersQueryNextstakes;

impl TableData for Vec<Option<NextStakingLedgersQueryNextstakes>> {
    fn get_columns(&self) -> Vec<String> {
        ["Key", "Stake", "Delegate", "Delegators", "Ledger Hash"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Key", "Delegate"]
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
                    decorate_with_currency_tag(get_balance(stake), "mina".to_string()),
                    convert_to_link(
                        if get_public_key(stake) == get_delegate(stake) {
                            "Self".to_string()
                        } else {
                            get_delegate(stake)
                        },
                        format!("/addresses/accounts/{}", get_delegate(stake)),
                    ),
                    convert_to_pill(get_delegators_count(stake), ColorVariant::Blue),
                    convert_to_span(get_ledger_hash(stake)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
