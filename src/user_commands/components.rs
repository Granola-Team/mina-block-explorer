use super::functions::*;
use crate::{
    common::{components::*, constants::*, models::*, table::*},
    summary::models::BlockchainSummary,
    user_commands::{graphql::transactions_query, models::PendingTxn},
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage, use_document_visibility, use_interval, UseIntervalReturn,
};
use web_sys::VisibilityState;

const QP_TXN_HASH: &str = "q-txn-hash";
const QP_TXN_TYPE: &str = "txn-type";
const QP_TXN_APPLIED: &str = "txn-applied";
const QP_ROW_LIMIT: &str = "row-limit";
const QP_HEIGHT: &str = "q-height";
const QP_FROM: &str = "q-from";
const QP_TO: &str = "q-to";
const BACKSCAN_LIMIT: u64 = 2000;

fn get_available_records(
    summary: BlockchainSummary,
    canonical_opt: Option<String>,
    applied_opt: Option<bool>,
    other_qps: bool,
) -> Option<u64> {
    if other_qps {
        None
    } else {
        match (canonical_opt, applied_opt) {
            (Some(tt), Some(true)) if &tt == "Canonical" => {
                Some(summary.total_num_applied_canonical_user_commands)
            }

            (None, Some(true)) | (None, None) => {
                Some(summary.total_num_applied_canonical_user_commands)
            }

            (Some(tt), Some(false)) if &tt == "Canonical" => {
                Some(summary.total_num_failed_canonical_user_commands)
            }

            (None, Some(false)) => Some(summary.total_num_failed_canonical_user_commands),

            (Some(tt), Some(true)) if &tt == "Non-Canonical" => Some(
                summary.total_num_applied_user_commands
                    - summary.total_num_applied_canonical_user_commands,
            ),

            (Some(tt), Some(false)) if &tt == "Non-Canonical" => Some(
                summary.total_num_failed_user_commands
                    - summary.total_num_failed_canonical_user_commands,
            ),

            _ => None,
        }
    }
}

#[component]
pub fn TransactionsSection() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let visibility = use_document_visibility();
    let (data_sig, set_data) = create_signal(None);
    let (txn_type_qp, _) = create_query_signal::<String>(QP_TXN_TYPE);
    let (row_limit_sig, _) = create_query_signal::<u64>(QP_ROW_LIMIT);
    let (txn_applied_sig, _) = create_query_signal::<bool>(QP_TXN_APPLIED);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<u64>(QP_HEIGHT);
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                txn_type_qp.get(),
                block_height_sig.get(),
                row_limit_sig.get(),
                txn_applied_sig.get(),
            )
        },
        move |(_, url_query_map, txn_type, block_height, row_limit, txn_applied)| async move {
            if visibility.get() != VisibilityState::Visible {
                logging::log!("Document not visible. Data polling skipped for user commands.");
                return Ok(transactions_query::ResponseData {
                    transactions: data_sig.get().unwrap_or_default(),
                    other_transactions: vec![],
                });
            }

            let (canonical, load_fn) = match txn_type.as_deref() {
                Some("Canonical") => (Some(true), load_data),
                Some("Non-Canonical") => (Some(false), load_data),
                _ => (Some(true), load_data),
            };

            load_fn(
                row_limit,
                url_query_map.get(QP_FROM).cloned(),
                url_query_map.get(QP_TO).cloned(),
                url_query_map.get(QP_TXN_HASH).cloned(),
                block_height,
                txn_applied.and_then(|a| if !a { Some(BACKSCAN_LIMIT) } else { None }),
                None,
                canonical,
                txn_applied,
            )
            .await
        },
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Txn Hash".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Status".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "From".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "To".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Nonce".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Amount".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            ..Default::default()
        },
    ];
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.transactions))
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            metadata=Signal::derive(move || {
                let mut otherQps = query_params_map.get();
                otherQps.remove(QP_TXN_TYPE);
                otherQps.remove(QP_TXN_APPLIED);
                otherQps.remove(QP_ROW_LIMIT);
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_user_commands).ok(),
                    available_records: get_available_records(
                        summary_sig.get(),
                        txn_type_qp.get(),
                        txn_applied_sig.get(),
                        !otherQps.to_query_string().is_empty(),
                    ),
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                })
            })

            is_loading=resource.loading()
            section_heading="User Commands"
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                    <UrlParamSelectMenu
                        id="txn-status"
                        query_str_key="txn-applied"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Applied".to_string(), "Failed".to_string()],
                        }
                    />
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="txn-type"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: false,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        />
    }
}

#[component]
pub fn PendingTransactionsSection() -> impl IntoView {
    let visibility = use_document_visibility();
    let (data_sig, set_data) = create_signal::<Option<Vec<Option<PendingTxn>>>>(None);
    let query_params_map = use_query_map();
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || counter.get(),
        move |_| async move {
            if visibility.get() != VisibilityState::Visible {
                logging::log!("Document not visible. Data polling skipped for user commands.");
                return Ok(transactions_query::ResponseData {
                    transactions: vec![],
                    other_transactions: vec![],
                });
            }

            load_pending_txn().await
        },
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Txn Hash".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "From".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "To".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Nonce".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Amount".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            ..Default::default()
        },
    ];
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(
                data.transactions
                    .into_iter()
                    .map(|opt_t| opt_t.map(PendingTxn::from))
                    .collect::<Vec<Option<PendingTxn>>>(),
            ))
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            metadata=Signal::derive(move || {
                let mut otherQps = query_params_map.get();
                otherQps.remove(QP_TXN_TYPE);
                otherQps.remove(QP_TXN_APPLIED);
                otherQps.remove(QP_ROW_LIMIT);
                Some(TableMetadata {
                    total_records: None,
                    available_records: None,
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                })
            })

            is_loading=resource.loading()
            section_heading="Pending Commands"
            controls=|| ().into_view()
        />
    }
}

#[cfg(test)]
mod get_available_records_tests {
    use super::*;

    #[test]
    fn test_canonical_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result =
            get_available_records(summary, Some("Canonical".to_string()), Some(true), false);
        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_non_canonical_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Non-Canonical".to_string()),
            Some(true),
            false,
        );
        assert_eq!(result, Some(100)); // 200 - 100
    }

    #[test]
    fn test_canonical_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result =
            get_available_records(summary, Some("Canonical".to_string()), Some(false), false);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_non_canonical_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Non-Canonical".to_string()),
            Some(false),
            false,
        );
        assert_eq!(result, Some(25)); // 75 - 50
    }

    #[test]
    fn test_none_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(summary, None, Some(true), false);
        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_none_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(summary, None, Some(false), false);
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_none_none() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(summary, None, None, false);
        assert_eq!(result, Some(100));
    }

    #[test]
    fn test_other_qps_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result =
            get_available_records(summary, Some("Canonical".to_string()), Some(true), true);
        assert_eq!(result, None);
    }
}
