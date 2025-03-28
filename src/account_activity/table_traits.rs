use super::{
    graphql::account_activity_query::{
        AccountActivityQuerySnarks, AccountActivityQueryTokenHolders,
    },
    models::AccountActivityQueryDelegatorExt,
};
use crate::{
    account_activity::{
        graphql::account_activity_query::{
            AccountActivityQueryBlocks, AccountActivityQueryFeetransfers,
        },
        models::{
            AccountActivityQueryDirectionalTransactionTrait,
            AccountActivityQueryDirectionalTransactions,
        },
    },
    common::{
        constants::{LHS_MAX_DIGIT_PADDING, LHS_MAX_SPACE_FEES, QUERY_PARAM_TOKEN},
        functions::*,
        models::*,
        table::TableData,
    },
};
use leptos::*;

impl TableData for Vec<Option<AccountActivityQueryDirectionalTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Height",
            "Txn Hash",
            "Nonce",
            "Age",
            "Type",
            "Direction",
            "Counterparty",
            "Amount/Fee",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "Txn Hash", "Nonce", "Counterparty"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_to_span(transaction.get_height()),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_copy_link(
                                transaction.get_hash(),
                                format!("/commands/{}", transaction.get_hash()),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_copy_link(
                            transaction.get_hash(),
                            format!("/commands/{}", transaction.get_hash()),
                        )
                    },
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&transaction.get_date_time()),
                        transaction.get_date_time(),
                    ),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    convert_to_pill(
                        transaction.get_direction(),
                        if transaction.get_direction() == *"OUT".to_string() {
                            ColorVariant::Blue
                        } else {
                            ColorVariant::DarkBlue
                        },
                    ),
                    convert_to_copy_link(
                        transaction.get_counterparty(),
                        format!("/addresses/accounts/{}", transaction.get_counterparty()),
                    ),
                    convert_array_to_span(vec![
                        decorate_with_mina_tag(transaction.get_amount()),
                        convert_array_to_span(vec![decorate_with_mina_tag(transaction.get_fee())])
                            .attr("class", "text-xs text-slate-400"),
                    ])
                    .attr("class", "flex flex-col items-start"),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<AccountActivityQuerySnarks>> {
    fn get_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Age", "Prover", "Fee"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Prover"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_snark| match opt_snark {
                Some(snark) => vec![
                    convert_to_span(snark.get_block_height()),
                    convert_to_copy_link(
                        snark.get_block_state_hash(),
                        format!("/blocks/{}/snark-jobs", snark.get_block_state_hash()),
                    ),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&snark.get_date_time()),
                        snark.get_date_time(),
                    ),
                    convert_to_span(snark.get_prover()),
                    decorate_with_mina_tag(snark.get_fee()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<AccountActivityQueryTokenHolders>> {
    fn get_exact_search_columns(&self) -> Vec<String> {
        vec![]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_token| match opt_token {
                Some(token) => vec![
                    convert_to_link(
                        token.symbol.to_string(),
                        format!("tokens?{}={}", QUERY_PARAM_TOKEN, token.token),
                    ),
                    convert_to_span(format_number_helper(
                        &token.account.balance.to_string(),
                        Some(0),
                    )),
                    convert_to_pill(token.account.nonce.to_string(), ColorVariant::Grey),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait SnarkTrait {
    #[allow(dead_code)]
    fn get_canonical(&self) -> String;
    fn get_block_height(&self) -> String;
    fn get_block_state_hash(&self) -> String;
    fn get_date_time(&self) -> String;
    fn get_prover(&self) -> String;
    fn get_fee(&self) -> String;
}

impl SnarkTrait for AccountActivityQuerySnarks {
    fn get_canonical(&self) -> String {
        self.canonical.unwrap_or_default().to_string()
    }
    fn get_block_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }
    fn get_block_state_hash(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.state_hash.as_ref())
            .map_or_else(String::new, |o1| o1.to_string())
    }
    fn get_date_time(&self) -> String {
        self.date_time.map_or(String::new(), |f| f.to_string())
    }
    fn get_prover(&self) -> String {
        self.prover
            .as_ref()
            .map_or_else(String::new, |f| f.to_string())
    }
    fn get_fee(&self) -> String {
        self.fee
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .unwrap_or_default()
    }
}

impl TableData for Vec<Option<AccountActivityQueryBlocks>> {
    fn get_columns(&self) -> Vec<String> {
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
        .map(ToString::to_string)
        .collect::<Vec<_>>()
    }

    fn get_exact_search_columns(&self) -> Vec<String> {
        ["Height", "State Hash", "Slot", "Block Producer"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_blocks| match opt_blocks {
                Some(block) => vec![
                    convert_to_span(block.get_block_height()),
                    convert_to_copy_link(
                        block.get_state_hash(),
                        format!("/blocks/{}/spotlight", block.get_state_hash()),
                    ),
                    convert_to_pill(block.get_slot(), ColorVariant::Grey),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&block.get_date_time()),
                        block.get_date_time(),
                    ),
                    convert_to_span(block.get_creator_account()),
                    decorate_with_mina_tag(block.get_coinbase()),
                    convert_to_pill(block.get_transaction_count(), ColorVariant::Blue),
                    convert_to_pill(block.get_snark_job_count(), ColorVariant::Blue),
                    convert_to_copy_link(
                        block.get_coinbase_receiver(),
                        format!("/addresses/accounts/{}", block.get_coinbase_receiver()),
                    ),
                ],
                None => vec![],
            })
            .collect()
    }
}

pub trait BlockTrait {
    #[allow(dead_code)]
    fn get_canonical(&self) -> String;
    fn get_block_height(&self) -> String;
    fn get_state_hash(&self) -> String;
    fn get_slot(&self) -> String;
    fn get_date_time(&self) -> String;
    fn get_creator_account(&self) -> String;
    fn get_coinbase(&self) -> String;
    fn get_transaction_count(&self) -> String;
    fn get_snark_job_count(&self) -> String;
    fn get_coinbase_receiver(&self) -> String;
}

impl BlockTrait for AccountActivityQueryBlocks {
    fn get_canonical(&self) -> String {
        self.canonical.unwrap_or_default().to_string()
    }
    fn get_block_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }
    fn get_state_hash(&self) -> String {
        self.state_hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }
    fn get_date_time(&self) -> String {
        self.date_time.map_or(String::new(), |f| f.to_string())
    }
    fn get_slot(&self) -> String {
        self.protocol_state.as_ref().map_or_else(String::new, |o| {
            o.consensus_state.as_ref().map_or_else(String::new, |o| {
                o.slot_since_genesis
                    .map_or_else(String::new, |o| format_number(o.to_string()))
            })
        })
    }
    fn get_creator_account(&self) -> String {
        self.creator_account.as_ref().map_or_else(String::new, |o| {
            o.public_key
                .as_ref()
                .map_or_else(String::new, |o1| o1.to_string())
        })
    }
    fn get_coinbase(&self) -> String {
        self.transactions
            .as_ref()
            .and_then(|o| o.coinbase.as_deref())
            .map(nanomina_str_to_mina)
            .unwrap_or_default()
    }
    fn get_transaction_count(&self) -> String {
        self.transactions
            .as_ref()
            .and_then(|o| o.user_commands.as_ref().map(|o1| o1.len()))
            .map_or_else(String::new, |c| c.to_string())
    }
    fn get_snark_job_count(&self) -> String {
        self.snark_jobs
            .as_ref()
            .map(|o| o.len())
            .map_or_else(String::new, |c| c.to_string())
    }
    fn get_coinbase_receiver(&self) -> String {
        self.transactions.as_ref().map_or_else(String::new, |o| {
            o.coinbase_receiver_account
                .as_ref()
                .map_or_else(String::new, |o| {
                    o.public_key
                        .as_ref()
                        .map_or_else(String::new, |o| o.to_string())
                })
        })
    }
}

impl TableData for Vec<Option<AccountActivityQueryDelegatorExt>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_stake| match opt_stake {
                Some(stake) => vec![
                    convert_to_copy_link(
                        stake.get_public_key(),
                        format!("/addresses/accounts/{}", stake.get_public_key()),
                    ),
                    convert_to_copy_link(
                        stake.get_username(),
                        format!("/addresses/accounts/{}", stake.get_public_key()),
                    ),
                    convert_to_span(stake.get_delegated_balance()),
                    convert_to_span(stake.get_percent_of_delegation()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<AccountActivityQueryFeetransfers>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_internal_command| match opt_internal_command {
                Some(internal_command) => vec![
                    convert_to_span(internal_command.get_height()),
                    convert_to_copy_link(
                        internal_command.get_state_hash(),
                        format!("/blocks/{}", internal_command.get_state_hash()),
                    ),
                    decorate_with_mina_tag(internal_command.get_fee()),
                    convert_to_pill(internal_command.get_type(), ColorVariant::Grey),
                    convert_to_title(
                        convert_to_local_timezone_formatted(&internal_command.get_block_datetime()),
                        internal_command.get_block_datetime(),
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait StakeTrait {
    fn get_public_key(&self) -> String;
    fn get_username(&self) -> String;
    fn get_delegated_balance(&self) -> String;
    fn get_percent_of_delegation(&self) -> String;
}

impl StakeTrait for AccountActivityQueryDelegatorExt {
    fn get_public_key(&self) -> String {
        self.public_key.clone().unwrap_or_default()
    }

    fn get_username(&self) -> String {
        self.username.clone().unwrap_or_default()
    }

    fn get_delegated_balance(&self) -> String {
        self.delegated_balance
            .unwrap_or(0)
            .try_into()
            .map(nanomina_to_mina)
            .map(|number| format_number_for_html(&number, LHS_MAX_DIGIT_PADDING))
            .unwrap_or_default()
    }

    fn get_percent_of_delegation(&self) -> String {
        format!("{:.2}", self.percent_of_delegation.unwrap_or(0.0))
    }
}

pub trait InternalCommandTrait {
    fn get_height(&self) -> String;
    fn get_state_hash(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_type(&self) -> String;
    fn get_block_datetime(&self) -> String;
}

impl InternalCommandTrait for AccountActivityQueryFeetransfers {
    fn get_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |t| format_number(t.to_string()))
    }
    fn get_state_hash(&self) -> String {
        self.block_state_hash
            .as_ref()
            .and_then(|bsh| bsh.state_hash.as_ref())
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_fee(&self) -> String {
        self.fee
            .map(|i| nanomina_to_mina(i as u64))
            .map(|number| format_number_for_html(&number, LHS_MAX_SPACE_FEES))
            .unwrap_or_default()
    }
    fn get_type(&self) -> String {
        self.type_
            .as_ref()
            .map_or_else(String::new, |t| t.to_string())
    }
    fn get_block_datetime(&self) -> String {
        self.date_time.map_or_else(String::new, |o| o.to_string())
    }
}
