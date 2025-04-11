use super::functions::*;
use crate::{
    common::{components::*, constants::*, models::*, table::*},
    summary::models::BlockchainSummary,
    user_commands::{
        graphql::transactions_query::{self, TransactionsQueryTransactionsZkappAccountsUpdated},
        models::PendingTxn,
    },
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
const QP_ROW_LIMIT: &str = "row-limit";
const QP_HEIGHT: &str = "q-height";
const QP_FROM: &str = "q-from";
const QP_TO: &str = "q-to";
const QP_TOKEN: &str = "q-token";
const BACKSCAN_LIMIT: u64 = 2000;
const STATUS_SEARCH_OPTION_APPLIED: &str = "Applied";
const STATUS_SEARCH_OPTION_FAILED: &str = "Failed";

fn get_available_records(
    summary: BlockchainSummary,
    canonical_opt: Option<String>,
    applied_opt: Option<bool>,
    zk_app: Option<bool>,
    other_qps: bool,
) -> Option<u64> {
    if other_qps {
        None
    } else {
        match (canonical_opt, applied_opt, zk_app) {
            // zk_app = Some(true) cases
            (Some(tt), Some(true), Some(true)) if &tt == "Canonical" => {
                Some(summary.total_num_applied_canonical_zkapp_commands)
            }
            (Some(tt), Some(false), Some(true)) if &tt == "Canonical" => {
                Some(summary.total_num_failed_canonical_zkapp_commands)
            }
            (Some(tt), Some(true), Some(true)) if &tt == "Non-Canonical" => Some(
                summary.total_num_applied_zkapp_commands
                    - summary.total_num_applied_canonical_zkapp_commands,
            ),
            (Some(tt), Some(false), Some(true)) if &tt == "Non-Canonical" => Some(
                summary.total_num_failed_zkapp_commands
                    - summary.total_num_failed_canonical_zkapp_commands,
            ),
            (None, Some(true), Some(true)) | (None, None, Some(true)) => {
                Some(summary.total_num_applied_canonical_zkapp_commands)
            }
            (None, Some(false), Some(true)) => {
                Some(summary.total_num_failed_canonical_zkapp_commands)
            }

            // zk_app = None or Some(false) cases (original behavior)
            (Some(tt), Some(true), _) if &tt == "Canonical" => {
                Some(summary.total_num_applied_canonical_user_commands)
            }
            (None, Some(true), _) | (None, None, _) => {
                Some(summary.total_num_applied_canonical_user_commands)
            }
            (Some(tt), Some(false), _) if &tt == "Canonical" => {
                Some(summary.total_num_failed_canonical_user_commands)
            }
            (None, Some(false), _) => Some(summary.total_num_failed_canonical_user_commands),
            (Some(tt), Some(true), _) if &tt == "Non-Canonical" => Some(
                summary.total_num_applied_user_commands
                    - summary.total_num_applied_canonical_user_commands,
            ),
            (Some(tt), Some(false), _) if &tt == "Non-Canonical" => Some(
                summary.total_num_failed_user_commands
                    - summary.total_num_failed_canonical_user_commands,
            ),

            _ => None,
        }
    }
}

#[component]
pub fn AccountsUpdatedSection(
    zkapp: Option<transactions_query::TransactionsQueryTransactionsZkapp>,
) -> impl IntoView {
    let (account_updates_sig, _) = create_signal::<
        Option<Vec<TransactionsQueryTransactionsZkappAccountsUpdated>>,
    >(zkapp.map(|zk| zk.accounts_updated.clone()));
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Account".to_string(),
            alignment: Some(ColumnTextAlignment::Left),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Balance Change".to_string(),
            alignment: Some(ColumnTextAlignment::Left),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            ..Default::default()
        },
        TableColumn {
            column: "Increment Nonce".to_string(),
            alignment: Some(ColumnTextAlignment::Left),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Token ID".to_string(),
            alignment: Some(ColumnTextAlignment::Left),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=account_updates_sig
            metadata=metadata.into()
            section_heading="Accounts Updated"
            is_loading=Signal::derive(move || account_updates_sig.get().is_none())
            controls=|| ().into_view()
        />
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
    let (txn_applied_sig, _) = create_query_signal::<String>(QUERY_PARAM_TXN_APPLIED);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<u64>(QP_HEIGHT);
    let (token_sig, _) = create_query_signal::<String>(QP_TOKEN);
    let (q_type_sig, _) = create_query_signal::<TransactionKind>(QUERY_PARAM_TYPE);
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
                q_type_sig.get(),
                token_sig.get(),
            )
        },
        move |(_, url_query_map, txn_type, block_height, row_limit, txn_applied, q_type, token)| async move {
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

            let is_txn_applied =
                txn_applied.is_none_or(|txn_applied| txn_applied != STATUS_SEARCH_OPTION_FAILED);

            load_fn(
                row_limit,
                url_query_map.get(QP_FROM).cloned(),
                url_query_map.get(QP_TO).cloned(),
                url_query_map.get(QP_TXN_HASH).cloned(),
                block_height,
                if !is_txn_applied {
                    Some(BACKSCAN_LIMIT)
                } else {
                    None
                },
                None,
                canonical,
                Some(is_txn_applied),
                q_type,
                token,
            )
            .await
        },
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Txn Hash".to_string(),
            search_type: ColumnSearchType::Text,
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
            search_type: ColumnSearchType::Select,
            search_options: Some(vec![
                "".to_string(),
                TransactionKind::Payment.to_string(),
                TransactionKind::Zkapp.to_string(),
                TransactionKind::StakeDelegation.to_string(),
            ]),
            ..Default::default()
        },
        TableColumn {
            column: "Status".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            search_type: ColumnSearchType::Select,
            search_options: Some(vec![
                "".to_string(),
                STATUS_SEARCH_OPTION_APPLIED.to_string(),
                STATUS_SEARCH_OPTION_FAILED.to_string(),
            ]),
            ..Default::default()
        },
        TableColumn {
            column: "From".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "To".to_string(),
            search_type: ColumnSearchType::Text,
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
                otherQps.remove(QUERY_PARAM_TXN_APPLIED);
                otherQps.remove(QP_ROW_LIMIT);
                otherQps.remove(QUERY_PARAM_TYPE);
                let mut available_records = None;
                let url_query_map = query_params_map.get();
                let from = url_query_map.get(QP_FROM).cloned();
                let to = url_query_map.get(QP_TO).cloned();
                let txn_hash = url_query_map.get(QP_TXN_HASH).cloned();
                let is_zk_app = q_type_sig.get().is_some_and(|p| p == TransactionKind::Zkapp);
                if block_height_sig.get().is_none() && from.is_none() && to.is_none()
                    && txn_hash.is_none()
                {
                    available_records = get_available_records(
                        summary_sig.get(),
                        txn_type_qp.get(),
                        Some(
                            txn_applied_sig
                                .get()
                                .is_none_or(|txn_applied| {
                                    txn_applied != STATUS_SEARCH_OPTION_FAILED
                                }),
                        ),
                        Some(is_zk_app),
                        !otherQps.to_query_string().is_empty(),
                    );
                }
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_user_commands).ok(),
                    available_records,
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                })
            })

            is_loading=resource.loading()
            section_heading="User Commands"
            footer=move || {
                view! {
                    <NextBlockPage
                        data=data_sig.get().unwrap_or(vec![])
                        row_limit=row_limit_sig.get()
                    />
                }
            }
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
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
            search_type: ColumnSearchType::Text,
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
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "To".to_string(),
            search_type: ColumnSearchType::Text,
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
                otherQps.remove(QUERY_PARAM_TXN_APPLIED);
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
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(true),
            None, // zk_app = None
            false,
        );
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
            None, // zk_app = None
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
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(false),
            None, // zk_app = None
            false,
        );
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
            None, // zk_app = None
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
        let result = get_available_records(
            summary,
            None,
            Some(true),
            None, // zk_app = None
            false,
        );
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
        let result = get_available_records(
            summary,
            None,
            Some(false),
            None, // zk_app = None
            false,
        );
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
        let result = get_available_records(summary, None, None, None, false); // zk_app = None
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
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(true),
            None, // zk_app = None
            true,
        );
        assert_eq!(result, None);
    }

    // New tests for zk_app = Some(true)
    #[test]
    fn test_zkapp_canonical_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(true),
            Some(true),
            false,
        );
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_zkapp_non_canonical_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Non-Canonical".to_string()),
            Some(true),
            Some(true),
            false,
        );
        assert_eq!(result, Some(70)); // 150 - 80
    }

    #[test]
    fn test_zkapp_canonical_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(false),
            Some(true),
            false,
        );
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_zkapp_non_canonical_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Non-Canonical".to_string()),
            Some(false),
            Some(true),
            false,
        );
        assert_eq!(result, Some(20)); // 60 - 40
    }

    #[test]
    fn test_zkapp_none_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(summary, None, Some(true), Some(true), false);
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_zkapp_none_applied_false() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(summary, None, Some(false), Some(true), false);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_zkapp_none_none() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_zkapp_commands: 80,
            total_num_failed_canonical_zkapp_commands: 40,
            total_num_applied_zkapp_commands: 150,
            total_num_failed_zkapp_commands: 60,
            ..Default::default()
        };
        let result = get_available_records(summary, None, None, Some(true), false);
        assert_eq!(result, Some(80));
    }

    // Test for zk_app = Some(false) to confirm it matches None behavior
    #[test]
    fn test_zkapp_false_canonical_applied_true() {
        let summary = BlockchainSummary {
            total_num_applied_canonical_user_commands: 100,
            total_num_failed_canonical_user_commands: 50,
            total_num_applied_user_commands: 200,
            total_num_failed_user_commands: 75,
            ..Default::default()
        };
        let result = get_available_records(
            summary,
            Some("Canonical".to_string()),
            Some(true),
            Some(false),
            false,
        );
        assert_eq!(result, Some(100));
    }
}
