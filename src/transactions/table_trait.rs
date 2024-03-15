use super::{
    functions::*, graphql::transactions_query::TransactionsQueryTransactions,
    models::DirectionalTransactionsQueryTransactions,
};
use crate::{
    common::{functions::*, models::ColorVariant, table::*},
};
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
                            transaction.get_canonical(),
                            transaction.get_failure_reason(),
                        ),
                        convert_to_span(transaction.get_block_height()),
                    ]),
                    convert_to_link(
                        transaction.get_hash(),
                        format!("/transactions/{}", transaction.get_hash()),
                    ),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&transaction.get_block_datetime())),
                        convert_to_span(transaction.get_block_datetime())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_link(
                                transaction.get_from(),
                                format!("/addresses/accounts/{}", transaction.get_from()),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_link(
                            transaction.get_from(),
                            format!("/addresses/accounts/{}", transaction.get_from()),
                        )
                    },
                    convert_to_link(
                        transaction.get_receiver_public_key(),
                        format!(
                            "/addresses/accounts/{}",
                            transaction.get_receiver_public_key()
                        ),
                    ),
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    wrap_in_pill(
                        decorate_with_currency_tag(transaction.get_fee(), "mina".to_string()),
                        ColorVariant::Orange,
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(transaction.get_amount(), "mina".to_string()),
                        ColorVariant::Green,
                    ),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<DirectionalTransactionsQueryTransactions>> {
    fn get_columns(&self) -> Vec<String> {
        [
            "Height",
            "State Hash",
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

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_trans| match opt_trans {
                Some(transaction) => vec![
                    convert_array_to_span(vec![
                        convert_to_status_bubble(
                            transaction.get_canonical(),
                            transaction.get_failure_reason(),
                        ),
                        convert_to_span(transaction.get_block_height()),
                    ]),
                    if !transaction.get_memo().is_empty() {
                        convert_array_to_span(vec![
                            convert_to_link(
                                transaction.get_hash(),
                                format!("/transactions/{}", transaction.get_hash()),
                            ),
                            convert_to_span(transaction.get_memo())
                                .attr("class", "block text-xs font-light text-slate-400"),
                        ])
                        .attr("class", "block")
                    } else {
                        convert_to_link(
                            transaction.get_hash(),
                            format!("/transactions/{}", transaction.get_hash()),
                        )
                    },
                    convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                    convert_array_to_span(vec![
                        convert_to_span(print_time_since(&transaction.get_block_datetime())),
                        convert_to_span(transaction.get_block_datetime())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    convert_to_pill(
                        if transaction.outbound {
                            "OUT".to_string()
                        } else {
                            "IN".to_string()
                        },
                        if transaction.outbound {
                            ColorVariant::Blue
                        } else {
                            ColorVariant::DarkBlue
                        },
                    ),
                    if transaction.outbound {
                        convert_to_link(
                            transaction.get_to(),
                            format!("/addresses/accounts/{}", transaction.get_to()),
                        )
                    } else {
                        convert_to_link(
                            transaction.get_to(),
                            format!("/addresses/accounts/{}", transaction.get_from()),
                        )
                    },
                    convert_array_to_span(vec![
                        wrap_in_pill(
                            decorate_with_currency_tag(
                                transaction.get_amount(),
                                "mina".to_string(),
                            ),
                            ColorVariant::Green,
                        ),
                        wrap_in_pill(
                            convert_array_to_span(vec![decorate_with_currency_tag(
                                transaction.get_fee(),
                                "mina".to_string(),
                            )])
                            .attr("class", "text-xs text-slate-400"),
                            ColorVariant::Transparent,
                        ),
                    ])
                    .attr("class", "flex flex-col items-start"),
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
    fn get_payment_id(&self) -> String;
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
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_canonical(&self) -> Option<bool> {
        self.canonical
    }

    fn get_kind(&self) -> String {
        self.kind
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_payment_id(&self) -> String {
        self.id.as_ref().map_or_else(String::new, |o| o.to_string())
    }

    fn get_nonce(&self) -> String {
        self.nonce.map_or_else(String::new, |o| o.to_string())
    }

    fn get_memo(&self) -> String {
        self.memo
            .as_ref()
            .map_or_else(String::new, |o| decode_memo(o).unwrap_or("".to_string()))
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
        self.fee.map(nanomina_to_mina).unwrap_or_default()
    }

    fn get_hash(&self) -> String {
        self.hash
            .as_ref()
            .map_or_else(String::new, |o| o.to_string())
    }

    fn get_amount(&self) -> String {
        self.amount.map(nanomina_to_mina).unwrap_or_default()
    }

    fn get_to(&self) -> String {
        self.to.as_ref().map_or_else(String::new, |o| o.to_string())
    }
}

impl TransactionsTrait for DirectionalTransactionsQueryTransactions {
    fn get_failure_reason(&self) -> Option<String> {
        self.base_transaction.get_failure_reason()
    }
    fn get_block_datetime(&self) -> String {
        self.base_transaction.get_block_datetime()
    }
    fn get_block_height(&self) -> String {
        self.base_transaction.get_block_height()
    }
    fn get_canonical(&self) -> Option<bool> {
        self.base_transaction.get_canonical()
    }
    fn get_kind(&self) -> String {
        self.base_transaction.get_kind()
    }
    fn get_payment_id(&self) -> String {
        self.base_transaction.get_payment_id()
    }
    fn get_nonce(&self) -> String {
        self.base_transaction.get_nonce()
    }
    fn get_memo(&self) -> String {
        self.base_transaction.get_memo()
    }
    fn get_block_state_hash(&self) -> String {
        self.base_transaction.get_block_state_hash()
    }
    fn get_from(&self) -> String {
        self.base_transaction.get_from()
    }
    fn get_receiver_public_key(&self) -> String {
        self.base_transaction.get_receiver_public_key()
    }
    fn get_fee(&self) -> String {
        self.base_transaction.get_fee()
    }
    fn get_hash(&self) -> String {
        self.base_transaction.get_hash()
    }
    fn get_amount(&self) -> String {
        self.base_transaction.get_amount()
    }
    fn get_to(&self) -> String {
        self.base_transaction.get_to()
    }
}
