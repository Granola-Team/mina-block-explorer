use super::{
    graphql::transactions_query::{
        TransactionsQueryOtherTransactions, TransactionsQueryTransactions,
    },
    models::PendingTxn,
};
use crate::common::{
    constants::{LHS_MAX_DIGIT_PADDING, LHS_MAX_SPACE_FEES, TXN_STATUS_APPLIED, TXN_STATUS_FAILED},
    functions::*,
    models::{ColorVariant, HasBlockHeight},
    table::*,
};
use heck::ToTitleCase;
use leptos::*;

impl TableData for Vec<Option<TransactionsQueryOtherTransactions>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_to_span(transaction.get_block_height()),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_clickable_link(
                                transaction.get_hash(),
                                format!(
                                    "/commands/{}?q-state-hash={}",
                                    transaction.get_hash(),
                                    transaction.get_block_state_hash()
                                ),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_clickable_link(
                            transaction.get_hash(),
                            format!(
                                "/commands/{}?q-state-hash={}",
                                transaction.get_hash(),
                                transaction.get_block_state_hash()
                            ),
                        )
                    },
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<TransactionsQueryTransactions>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_to_span(transaction.get_block_height()),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_clickable_link(
                                transaction.get_hash(),
                                format!(
                                    "/commands/{}?q-state-hash={}",
                                    transaction.get_hash(),
                                    transaction.get_block_state_hash()
                                ),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_clickable_link(
                            transaction.get_hash(),
                            format!(
                                "/commands/{}?q-state-hash={}",
                                transaction.get_hash(),
                                transaction.get_block_state_hash()
                            ),
                        )
                    },
                    convert_to_title(
                        convert_to_local_timezone_formatted(&transaction.get_block_datetime()),
                        transaction.get_block_datetime(),
                    ),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    if transaction.get_failure_reason().is_none() {
                        convert_to_pill(TXN_STATUS_APPLIED.to_string(), ColorVariant::Green)
                    } else {
                        convert_to_pill(TXN_STATUS_FAILED.to_string(), ColorVariant::Orange)
                    },
                    convert_to_clickable_link(
                        transaction.get_from(),
                        format!("/addresses/accounts/{}", transaction.get_from()),
                    ),
                    convert_to_clickable_link(
                        transaction.get_receiver_public_key(),
                        format!(
                            "/addresses/accounts/{}",
                            transaction.get_receiver_public_key()
                        ),
                    ),
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    convert_to_span(transaction.get_fee()),
                    convert_to_span(transaction.get_amount()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<PendingTxn>> {
    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_to_span(transaction.get_hash()),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    convert_to_clickable_link(
                        transaction.get_from(),
                        format!("/addresses/accounts/{}", transaction.get_from()),
                    ),
                    convert_to_clickable_link(
                        transaction.get_to(),
                        format!("/addresses/accounts/{}", transaction.get_to()),
                    ),
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    convert_to_span(transaction.get_fee()),
                    convert_to_span(transaction.get_amount()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

pub trait TransactionsTrait {
    fn get_failure_reason(&self) -> Option<String>;
    fn get_block_datetime(&self) -> String;
    fn get_block_height(&self) -> String;
    fn get_canonical(&self) -> Option<bool>;
    fn get_kind(&self) -> String;
    fn get_nonce(&self) -> String;
    fn get_memo(&self) -> String;
    fn get_block_state_hash(&self) -> String;
    fn get_from(&self) -> String;
    fn get_receiver_public_key(&self) -> String;
    fn get_fee(&self) -> String;
    fn get_hash(&self) -> String;
    fn get_amount(&self) -> String;
    fn get_to(&self) -> String;
}

impl TransactionsTrait for TransactionsQueryTransactions {
    fn get_failure_reason(&self) -> Option<String> {
        self.failure_reason.clone()
    }

    fn get_block_datetime(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.date_time)
            .map_or_else(String::new, |o1| o1.to_string())
    }

    fn get_block_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }

    fn get_canonical(&self) -> Option<bool> {
        self.canonical
    }

    fn get_kind(&self) -> String {
        self.kind.as_ref().map_or_else(String::new, |o| {
            ToTitleCase::to_title_case(o.as_str()).to_string()
        })
    }

    fn get_nonce(&self) -> String {
        self.nonce
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    }

    fn get_block_state_hash(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.state_hash.as_ref())
            .map_or_else(String::new, |o1| o1.to_string())
    }

    fn get_from(&self) -> String {
        self.from
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_receiver_public_key(&self) -> String {
        self.receiver.as_ref().map_or_else(String::new, |o| {
            o.public_key
                .as_ref()
                .map_or_else(String::new, |o| o.to_string())
        })
    }

    fn get_fee(&self) -> String {
        self.fee
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map(|number| format_number_for_html(&number, LHS_MAX_SPACE_FEES))
            .unwrap_or_default()
    }

    fn get_hash(&self) -> String {
        self.hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        self.amount
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map(|number| format_number_for_html(&number, LHS_MAX_DIGIT_PADDING))
            .unwrap_or_default()
    }

    fn get_to(&self) -> String {
        self.to.as_ref().map_or_else(String::new, |o| o.to_string())
    }
}

impl TransactionsTrait for PendingTxn {
    fn get_failure_reason(&self) -> Option<String> {
        None
    }

    fn get_block_datetime(&self) -> String {
        String::new()
    }

    fn get_block_height(&self) -> String {
        String::new()
    }

    fn get_canonical(&self) -> Option<bool> {
        None
    }

    fn get_kind(&self) -> String {
        self.kind.as_ref().map_or_else(String::new, |o| {
            ToTitleCase::to_title_case(o.as_str()).to_string()
        })
    }

    fn get_nonce(&self) -> String {
        self.nonce
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }

    fn get_memo(&self) -> String {
        String::new()
    }

    fn get_block_state_hash(&self) -> String {
        String::new()
    }

    fn get_from(&self) -> String {
        self.from
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_receiver_public_key(&self) -> String {
        String::new()
    }

    fn get_fee(&self) -> String {
        self.fee
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map(|number| format_number_for_html(&number, LHS_MAX_SPACE_FEES))
            .unwrap_or_default()
    }

    fn get_hash(&self) -> String {
        self.txn_hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        self.amount
            .map(|f| f.round() as u64)
            .map(nanomina_to_mina)
            .map(|number| format_number_for_html(&number, LHS_MAX_DIGIT_PADDING))
            .unwrap_or_default()
    }

    fn get_to(&self) -> String {
        self.to.as_ref().map_or_else(String::new, |o| o.to_string())
    }
}

impl TransactionsTrait for TransactionsQueryOtherTransactions {
    fn get_failure_reason(&self) -> Option<String> {
        None
    }

    fn get_block_datetime(&self) -> String {
        String::new()
    }

    fn get_block_height(&self) -> String {
        self.block_height
            .map_or_else(String::new, |o| format_number(o.to_string()))
    }

    fn get_canonical(&self) -> Option<bool> {
        self.canonical
    }

    fn get_kind(&self) -> String {
        String::new()
    }

    fn get_nonce(&self) -> String {
        String::new()
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, ToString::to_string)
    }

    fn get_block_state_hash(&self) -> String {
        self.block
            .as_ref()
            .and_then(|b| b.state_hash.as_ref())
            .map_or_else(String::new, |o1| o1.to_string())
    }

    fn get_from(&self) -> String {
        String::new()
    }

    fn get_receiver_public_key(&self) -> String {
        String::new()
    }

    fn get_fee(&self) -> String {
        String::new()
    }

    fn get_hash(&self) -> String {
        self.hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        String::new()
    }

    fn get_to(&self) -> String {
        String::new()
    }
}

impl HasBlockHeight for TransactionsQueryTransactions {
    fn block_height(&self) -> Option<i64> {
        self.block_height
    }
}
