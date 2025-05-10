use super::{
    functions::*, graphql::blocks_query::BlocksQueryBlocks,
    models::BlocksQueryBlocksTransactionsUserCommandsExt,
};
use crate::{
    blocks::graphql::blocks_query::{
        BlocksQueryBlocksSnarkJobs, BlocksQueryBlocksTransactionsFeeTransfer,
    },
    common::{
        constants::{TXN_STATUS_APPLIED, TXN_STATUS_FAILED},
        functions::*,
        models::*,
        table::*,
    },
};
use heck::ToTitleCase;
use leptos::*;

impl TableData for Vec<Option<BlocksQueryBlocks>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_blocks| match opt_blocks {
                Some(block) => vec![
                    convert_to_canonical(
                        get_block_height(block),
                        block.canonical.unwrap_or_default(),
                    ),
                    convert_to_copy_link(
                        get_state_hash(block),
                        format!("/blocks/{}/spotlight", get_state_hash(block)),
                    ),
                    convert_to_pill(get_global_slot(block), ColorVariant::Grey),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&get_date_time(block)),
                        get_date_time(block),
                    ),
                    convert_to_linkable_address(
                        &get_creator_username(block),
                        &get_creator_account(block),
                    ),
                    decorate_with_mina_tag(get_coinbase(block)),
                    convert_to_pill(
                        format!(
                            "{}/{}",
                            get_transaction_count(block)
                                .map_or_else(String::new, |o| o.to_string()),
                            block.block_num_zkapp_commands
                        ),
                        ColorVariant::Blue,
                    ),
                    convert_to_pill(
                        get_snark_job_count(block).map_or_else(String::new, |o| o.to_string()),
                        ColorVariant::Blue,
                    ),
                    convert_to_copy_link(
                        get_coinbase_receiver(block),
                        format!("/addresses/accounts/{}", get_coinbase_receiver(block)),
                    ),
                ],
                None => vec![],
            })
            .collect()
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksTransactionsUserCommandsExt>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_user_command| match opt_user_command {
                Some(user_command) => vec![
                    if !user_command.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_copy_link(
                                user_command.get_txn_hash(),
                                format!(
                                    "/commands/{}?q-state-hash={}",
                                    user_command.get_txn_hash(),
                                    user_command.get_block_state_hash()
                                ),
                            ),
                            convert_to_span(user_command.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_copy_link(
                            user_command.get_txn_hash(),
                            format!(
                                "/commands/{}?q-state-hash={}",
                                user_command.get_txn_hash(),
                                user_command.get_block_state_hash()
                            ),
                        )
                    },
                    convert_to_pill(
                        user_command.get_kind(),
                        match user_command.get_kind().as_str() {
                            "Zkapp" => ColorVariant::DarkBlue,
                            "Payment" => ColorVariant::DarkGreen,
                            _ => ColorVariant::DarkGrey,
                        },
                    ),
                    convert_to_pill(
                        if user_command.get_failure_reason().is_none() {
                            TXN_STATUS_APPLIED.to_string()
                        } else {
                            TXN_STATUS_FAILED.to_string()
                        },
                        if user_command.get_failure_reason().is_none() {
                            ColorVariant::Green
                        } else {
                            ColorVariant::Orange
                        },
                    ),
                    convert_to_copy_link(
                        user_command.get_from(),
                        format!("/addresses/accounts/{}", user_command.get_from()),
                    ),
                    convert_to_copy_link(
                        user_command.get_to(),
                        format!("/addresses/accounts/{}", user_command.get_to()),
                    ),
                    convert_to_pill(format_number(user_command.get_nonce()), ColorVariant::Grey),
                    decorate_with_mina_tag(nanomina_to_mina(
                        user_command.get_fee().parse::<u64>().ok().unwrap_or(0),
                    )),
                    decorate_with_mina_tag(nanomina_to_mina(
                        user_command.get_amount().parse::<u64>().ok().unwrap_or(0),
                    )),
                ],
                None => vec![],
            })
            .collect()
    }
}

pub trait UserCommandTrait {
    fn get_txn_hash(&self) -> String;
    fn get_block_state_hash(&self) -> String;
    fn get_memo(&self) -> String;
    fn get_kind(&self) -> String;
    fn get_failure_reason(&self) -> Option<String>;
    fn get_from(&self) -> String;
    fn get_to(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_amount(&self) -> String;
}

impl UserCommandTrait for BlocksQueryBlocksTransactionsUserCommandsExt {
    fn get_txn_hash(&self) -> String {
        self.hash
            .as_ref()
            .map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_block_state_hash(&self) -> String {
        self.block_state_hash
            .as_ref()
            .map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo.as_ref().map_or("".to_string(), |o| o.to_string())
    }

    fn get_kind(&self) -> String {
        self.kind.as_ref().map_or("".to_string(), |o| {
            ToTitleCase::to_title_case(o.as_str()).to_string()
        })
    }

    fn get_failure_reason(&self) -> Option<String> {
        self.failure_reason.clone()
    }

    fn get_from(&self) -> String {
        self.from
            .as_ref()
            .map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_to(&self) -> String {
        self.to
            .as_ref()
            .map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_nonce(&self) -> String {
        self.nonce.map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_fee(&self) -> String {
        self.fee.map_or_else(|| "".to_string(), |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        self.amount
            .map_or_else(|| "".to_string(), |o| o.to_string())
    }
}

impl TableData for Vec<Option<BlocksQueryBlocksSnarkJobs>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_copy_link(
                        get_snark_block_state_hash(snark),
                        format!("/blocks/{}", get_snark_block_state_hash(snark)),
                    ),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&get_snark_date_time(snark)),
                        get_snark_date_time(snark),
                    ),
                    convert_to_copy_link(
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
                    convert_to_copy_link(
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

impl HasBlockHeight for BlocksQueryBlocks {
    fn block_height(&self) -> Option<i64> {
        self.block_height
    }
}
