use super::{functions::*, graphql::transactions_query::TransactionsQueryTransactions};
use crate::common::{functions::*, models::PillVariant, table::*};
use leptos::*;

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Height",
            "State Hash",
            "Age",
            "Type",
            "From",
            "To",
            "Nonce",
            "Fee",
            "Amount",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(
                            get_canonical(transaction),
                            get_failure_reason(transaction),
                        ),
                        convert_to_span(get_block_height(transaction)),
                    ]),
                    convert_to_link(
                        get_hash(transaction),
                        format!("/transactions/{}", get_hash(transaction)),
                    ),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_block_datetime(transaction))),
                        convert_to_span(get_block_datetime(transaction))
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(get_kind(transaction), PillVariant::Grey),
                    if !get_memo(transaction).is_empty() {
                        convert_array_to_span(vec![
                            convert_to_link(
                                get_from(transaction),
                                format!("/addresses/accounts/{}", get_from(transaction)),
                            ),
                            convert_to_span(get_memo(transaction))
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_link(
                            get_from(transaction),
                            format!("/addresses/accounts/{}", get_from(transaction)),
                        )
                    },
                    convert_to_link(
                        get_receiver_public_key(transaction),
                        format!(
                            "/addresses/accounts/{}",
                            get_receiver_public_key(transaction)
                        ),
                    ),
                    convert_to_pill(get_nonce(transaction), PillVariant::Grey),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_fee(transaction), "mina".to_string()),
                        PillVariant::Orange,
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_amount(transaction), "mina".to_string()),
                        PillVariant::Green,
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
