use leptos::*;

use super::functions::*;
use super::graphql::blocks_query::BlocksQueryBlocks;
use super::graphql::blocks_query::BlocksQueryBlocksTransactionsUserCommands;
use super::models::*;
use crate::common::components::*;
use crate::common::functions::*;

fn shared_get_columns() -> Vec<String> {
    vec![
        String::from("Height"),
        String::from("Date"),
        String::from("Block Producer"),
        String::from("Coinbase"),
        String::from("Transactions"),
        String::from("SNARKs"),
        String::from("Slot"),
        String::from("State Hash"),
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
                    convert_to_span(get_block_height(block)),
                    convert_to_span(get_date_time(block)),
                    convert_to_link(
                        get_creator_account(block),
                        format!("/blocks/accounts/{}", get_creator_account(block)),
                    ),
                    convert_to_span(get_coinbase(block)),
                    convert_to_span(get_transaction_count(block)),
                    convert_to_span(get_snark_job_count(block)),
                    convert_to_span(get_slot(block)),
                    convert_to_span(get_state_hash(block)),
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

impl TableData for &[Option<BlocksQueryBlocksTransactionsUserCommands>] {
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
                    get_user_command_from(user_command), 
                    get_user_command_to(user_command),
                    get_user_command_hash(user_command),
                    get_user_command_fee(user_command),
                    get_user_command_amount(user_command),
                ].into_iter()
                .map(convert_to_span)
                .collect(),
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
                    convert_to_span(get_block_height(block)),
                    convert_to_span(get_date_time(block)),
                    convert_to_link(
                        get_creator_account(block),
                        format!("/summary/accounts/{}", get_creator_account(block)),
                    ),
                    convert_to_span(get_coinbase(block)),
                    convert_to_span(get_transaction_count(block)),
                    convert_to_span(get_snark_job_count(block)),
                    convert_to_span(get_slot(block)),
                    convert_to_span(get_state_hash(block)),
                    convert_to_link(
                        get_coinbase_receiver(block),
                        format!("/summary/accounts/{}", get_coinbase_receiver(block)),
                    ),
                ]
            })
            .collect()
    }
}
