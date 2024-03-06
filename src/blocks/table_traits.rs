use super::{
    functions::*,
    graphql::blocks_query::{BlocksQueryBlocks, BlocksQueryBlocksTransactionsUserCommands},
    models::*,
};
use crate::common::{functions::*, models::*, table::*};
use leptos::*;

fn shared_get_columns() -> Vec<String> {
    vec![
        String::from("Height"),
        String::from("State Hash"),
        String::from("Slot"),
        String::from("Age"),
        String::from("Block Producer"),
        String::from("Coinbase"),
        String::from("Transactions"),
        String::from("SNARKs"),
        String::from("Coinbase Receiver"),
    ]
}

impl TableData for Vec<Option<BlocksQueryBlocks>> {
    fn get_columns(&self) -> Vec<String> {
        shared_get_columns()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_blocks| match opt_blocks {
                Some(block) => vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(if get_canonical(block) {
                            None
                        } else {
                            Some("Non-Canonical".to_string())
                        }),
                        convert_to_span(get_block_height(block)),
                    ]),
                    convert_to_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_slot(block), PillVariant::Orange),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_date_time(block))),
                        convert_to_span(get_date_time(block))
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_link(
                        get_creator_account(block),
                        format!("/blocks/accounts/{}", get_creator_account(block)),
                    ),
                    wrap_in_pill(decorate_with_currency_tag(get_coinbase(block), "mina".to_string()),PillVariant::Green),
                    convert_to_pill(
                        get_transaction_count(block).map_or_else(String::new, |o| o.to_string()),
                        PillVariant::Green,
                    ),
                    convert_to_pill(
                        get_snark_job_count(block).map_or_else(String::new, |o| o.to_string()),
                        PillVariant::Blue,
                    ),
                    convert_to_link(
                        get_coinbase_receiver(block),
                        format!("/blocks/accounts/{}", get_coinbase_receiver(block)),
                    ),
                ],
                None => vec![],
            })
            .collect()
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksTransactionsUserCommands>> {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("From"),
            String::from("To"),
            String::from("Hash"),
            String::from("Fee"),
            String::from("Amount"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_user_command| match opt_user_command {
                Some(user_command) => vec![
                    convert_to_link(
                        get_user_command_from(user_command),
                        format!(
                            "/addresses/accounts/{}",
                            get_user_command_from(user_command)
                        ),
                    ),
                    convert_to_link(
                        get_user_command_to(user_command),
                        format!("/addresses/accounts/{}", get_user_command_to(user_command)),
                    ),
                    convert_to_link(
                        get_user_command_hash(user_command),
                        format!("/transactions/{}", get_user_command_hash(user_command)),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            get_user_command_fee(user_command),
                            "mina".to_string(),
                        ),
                        PillVariant::Orange,
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            get_user_command_amount(user_command),
                            "mina".to_string(),
                        ),
                        PillVariant::Green,
                    ),
                ],
                None => vec![],
            })
            .collect()
    }
}

impl TableData for SummaryPageBlocksQueryBlocks {
    fn get_columns(&self) -> Vec<String> {
        shared_get_columns()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.0
            .iter()
            .filter_map(|block_opt| block_opt.as_ref())
            .map(|block| {
                vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(if get_canonical(block) {
                            None
                        } else {
                            Some("Non-Canonical".to_string())
                        }),
                        convert_to_span(get_block_height(block)),
                    ]),
                    convert_to_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_slot(block), PillVariant::Grey),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_date_time(block))),
                        convert_to_span(get_date_time(block))
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_link(
                        get_creator_account(block),
                        format!("/summary/accounts/{}", get_creator_account(block)),
                    ),
                    decorate_with_currency_tag(get_coinbase(block), "mina".to_string()),
                    convert_to_pill(
                        get_transaction_count(block).map_or_else(String::new, |o| o.to_string()),
                        PillVariant::Green,
                    ),
                    convert_to_pill(
                        get_snark_job_count(block).map_or_else(String::new, |o| o.to_string()),
                        PillVariant::Blue,
                    ),
                    convert_to_link(
                        get_coinbase_receiver(block),
                        format!("/summary/accounts/{}", get_coinbase_receiver(block)),
                    ),
                ]
            })
            .collect()
    }
}
