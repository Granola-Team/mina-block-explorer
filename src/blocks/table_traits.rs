use super::{
    functions::*,
    graphql::blocks_query::{BlocksQueryBlocks, BlocksQueryBlocksTransactionsUserCommands},
    models::*,
};
use crate::{
    blocks::graphql::blocks_query::{
        BlocksQueryBlocksSnarkJobs, BlocksQueryBlocksTransactionsFeeTransfer,
    },
    common::{functions::*, models::*, table::*},
};
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
                        convert_to_status_bubble(get_canonical(block), None),
                        convert_to_span(get_block_height(block)),
                    ]),
                    convert_to_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_slot(block), ColorVariant::Grey),
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
                    wrap_in_pill(
                        decorate_with_currency_tag(get_coinbase(block), "mina".to_string()),
                        ColorVariant::Green,
                    ),
                    convert_to_pill(
                        get_transaction_count(block).map_or_else(String::new, |o| o.to_string()),
                        ColorVariant::Blue,
                    ),
                    convert_to_pill(
                        get_snark_job_count(block).map_or_else(String::new, |o| o.to_string()),
                        ColorVariant::Blue,
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
                        format!("/commands/{}", get_user_command_hash(user_command)),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            get_user_command_fee(user_command),
                            "mina".to_string(),
                        ),
                        ColorVariant::Orange,
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            get_user_command_amount(user_command),
                            "mina".to_string(),
                        ),
                        ColorVariant::Green,
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
                        convert_to_status_bubble(get_canonical(block), None),
                        convert_to_span(get_block_height(block)),
                    ]),
                    convert_to_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_slot(block), ColorVariant::Grey),
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
                    wrap_in_pill(
                        decorate_with_currency_tag(get_coinbase(block), "mina".to_string()),
                        ColorVariant::Green,
                    ),
                    convert_to_pill(
                        get_transaction_count(block).map_or_else(String::new, |o| o.to_string()),
                        ColorVariant::Blue,
                    ),
                    convert_to_pill(
                        get_snark_job_count(block).map_or_else(String::new, |o| o.to_string()),
                        ColorVariant::Blue,
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

impl TableData for Vec<Option<BlocksQueryBlocksSnarkJobs>> {
    fn get_columns(&self) -> Vec<String> {
        ["State Hash", "Age", "Prover", "Work Ids", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_link(
                        get_snark_block_state_hash(snark),
                        format!("/blocks/{}", get_snark_block_state_hash(snark)),
                    ),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&get_snark_date_time(snark))),
                        convert_to_span(get_snark_date_time(snark))
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_link(
                        get_snark_prover(snark),
                        format!("/addresses/accounts/{}", get_snark_prover(snark)),
                    ),
                    convert_array_to_span(
                        get_snark_work_ids(snark)
                            .iter()
                            .map(|w| convert_to_pill(w.to_string(), ColorVariant::Grey))
                            .collect::<Vec<_>>(),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(get_snark_fee(snark), "mina".to_string()),
                        ColorVariant::Orange,
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksTransactionsFeeTransfer>> {
    fn get_columns(&self) -> Vec<String> {
        ["Recipient", "Fee", "Type"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_fee_transfer| match opt_fee_transfer {
                Some(fee_transfer) => vec![
                    convert_to_link(
                        fee_transfer.get_receipient(),
                        format!("/addresses/accounts/{}", fee_transfer.get_receipient()),
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(fee_transfer.get_fee(), "mina".to_string()),
                        ColorVariant::Orange,
                    ),
                    convert_to_pill(fee_transfer.get_type(), ColorVariant::Grey),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait FeeTransferTrait {
    fn get_receipient(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_type(&self) -> String;
}

impl FeeTransferTrait for BlocksQueryBlocksTransactionsFeeTransfer {
    fn get_receipient(&self) -> String {
        self.recipient
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_fee(&self) -> String {
        self.fee
            .as_deref()
            .map(nanomina_str_to_mina)
            .unwrap_or_default()
    }
    fn get_type(&self) -> String {
        self.type_
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
}
