use super::{
    functions::*,
    graphql::blocks_query::{BlocksQueryBlocks, BlocksQueryBlocksTransactionsUserCommands},
};
use crate::{
    blocks::graphql::blocks_query::{
        BlocksQueryBlocksSnarkJobs, BlocksQueryBlocksTransactionsFeeTransfer,
    },
    common::{functions::*, models::*, table::*},
};
use leptos::*;

fn shared_get_columns() -> Vec<String> {
    [
        "Height",
        "State Hash",
        "Slot",
        "Age",
        "Block Producer",
        "Coinbase",
        "User Commands",
        "SNARKs",
        "Coinbase Receiver",
    ]
    .iter()
    .map(|slc| slc.to_string())
    .collect::<Vec<_>>()
}

impl TableData for Vec<Option<BlocksQueryBlocks>> {
    fn get_columns(&self) -> Vec<String> {
        shared_get_columns()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Slot", "Block Producer"]
            .iter()
            .map(|slc| slc.to_string())
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_blocks| match opt_blocks {
                Some(block) => vec![
                    convert_to_span(get_block_height(block)),
                    convert_to_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_slot(block), ColorVariant::Grey),
                    convert_to_title(
                        print_time_since(&get_date_time(block)),
                        get_date_time(block),
                    ),
                    convert_to_link(
                        get_creator_account(block),
                        format!("/blocks/accounts/{}", get_creator_account(block)),
                    ),
                    decorate_with_mina_tag(get_coinbase(block)),
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
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_user_command| match opt_user_command {
                Some(user_command) => vec![
                    if !get_memo(user_command).is_empty() {
                        convert_array_to_span(vec![
                            convert_to_link(
                                get_user_command_hash(user_command),
                                format!("/commands/{}", get_user_command_hash(user_command)),
                            ),
                            convert_to_span(get_memo(user_command))
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_link(
                            get_user_command_hash(user_command),
                            format!("/commands/{}", get_user_command_hash(user_command)),
                        )
                    },
                    convert_to_pill(get_kind(user_command), ColorVariant::Grey),
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
                    decorate_with_mina_tag(get_user_command_fee(user_command)),
                    decorate_with_mina_tag(get_user_command_amount(user_command)),
                ],
                None => vec![],
            })
            .collect()
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksSnarkJobs>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_link(
                        get_snark_block_state_hash(snark),
                        format!("/blocks/{}", get_snark_block_state_hash(snark)),
                    ),
                    convert_to_title(
                        print_time_since(&get_snark_date_time(snark)),
                        get_snark_date_time(snark),
                    ),
                    convert_to_link(
                        get_snark_prover(snark),
                        format!("/addresses/accounts/{}", get_snark_prover(snark)),
                    ),
                    decorate_with_mina_tag(get_snark_fee(snark)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksTransactionsFeeTransfer>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_fee_transfer| match opt_fee_transfer {
                Some(fee_transfer) => vec![
                    convert_to_link(
                        fee_transfer.get_receipient(),
                        format!("/addresses/accounts/{}", fee_transfer.get_receipient()),
                    ),
                    decorate_with_mina_tag(fee_transfer.get_fee()),
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
