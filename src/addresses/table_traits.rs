use super::models::AllAccountSummary;
use crate::{
    account_dialog::models::{
        AccountActivityQueryDirectionalTransactionTrait,
        AccountActivityQueryDirectionalTransactions,
    },
    common::{functions::*, models::*, table::TableData},
};
use leptos::*;

impl TableData for Vec<Option<AllAccountSummary>> {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Public Key"),
            String::from("Username"),
            String::from("Balance"),
            String::from("Nonce"),
            String::from("Delegate"),
            String::from("Time Locked"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|all_acct_sum_opt| match all_acct_sum_opt {
                Some(all_account_sum) => vec![
                    convert_to_link(
                        all_account_sum.pk.to_string(),
                        format!("/addresses/accounts/{}", all_account_sum.pk),
                    ),
                    convert_to_span(all_account_sum.username.to_string()),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            all_account_sum.balance.to_string(),
                            "mina".to_string(),
                        ),
                        ColorVariant::Green,
                    ),
                    wrap_in_pill(
                        decorate_with_currency_tag(
                            all_account_sum.nonce.to_string(),
                            "mina".to_string(),
                        ),
                        ColorVariant::Grey,
                    ),
                    convert_to_link(
                        all_account_sum.delegate.to_string(),
                        format!("/addresses/accounts/{}", all_account_sum.delegate),
                    ),
                    convert_to_span(false.to_string()),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}

impl TableData for Vec<Option<AccountActivityQueryDirectionalTransactions>> {
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
                            Some(transaction.get_canonical()),
                            if transaction.get_failure_reason().is_empty() {
                                None
                            } else {
                                Some(transaction.get_failure_reason())
                            },
                        ),
                        convert_to_span(transaction.get_height()),
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
                        convert_to_span(print_time_since(&transaction.get_date_time())),
                        convert_to_span(transaction.get_date_time())
                            .attr("class", "block text-xs font-light text-slate-400"),
                    ])
                    .attr("class", "block"),
                    convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                    convert_to_pill(
                        transaction.get_direction(),
                        if transaction.get_direction() == *"OUT".to_string() {
                            ColorVariant::Blue
                        } else {
                            ColorVariant::DarkBlue
                        },
                    ),
                    convert_to_link(
                        transaction.get_counterparty(),
                        format!("/addresses/accounts/{}", transaction.get_counterparty()),
                    ),
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
