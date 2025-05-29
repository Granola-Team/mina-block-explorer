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
    UseIntervalReturn, storage::use_local_storage, use_document_visibility, use_interval,
};
use web_sys::VisibilityState;

const BACKSCAN_LIMIT: u64 = 2000;

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
    let (txn_type_qp, _) = create_query_signal::<String>(QUERY_PARAM_TXN_TYPE);
    let (row_limit_sig, _) = create_query_signal::<u64>(QUERY_PARAM_ROW_LIMIT);
    let (txn_applied_sig, _) = create_query_signal::<String>(QUERY_PARAM_TXN_APPLIED);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<u64>(QUERY_PARAM_HEIGHT);
    let (q_token_sig, _) = create_query_signal::<String>(QUERY_PARAM_TOKEN);
    let (q_type_sig, _) = create_query_signal::<TransactionKind>(QUERY_PARAM_TYPE);
    let (token_sig, set_token) = create_signal(None);
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
                q_token_sig.get(),
            )
        },
        move |(_, url_query_map, txn_type, block_height, row_limit, txn_applied, q_type, token)| async move {
            if visibility.get_untracked() != VisibilityState::Visible {
                logging::log!("Document not visible. Data polling skipped for user commands.");
                return Ok(transactions_query::ResponseData {
                    transactions: data_sig.get().unwrap_or_default(),
                    other_transactions: vec![],
                    tokens: vec![],
                });
            }

            let (canonical, load_fn) = match txn_type.as_deref() {
                Some("Canonical") => (Some(true), load_data),
                Some("Non-Canonical") => (Some(false), load_data),
                _ => (Some(true), load_data),
            };

            let is_txn_applied = match txn_applied {
                None => None, // by default, assume all txn
                Some(txn) if txn == STATUS_SEARCH_OPTION_APPLIED => Some(true),
                Some(txn) if txn == STATUS_SEARCH_OPTION_FAILED => Some(false),
                _ => None,
            };

            load_fn(
                row_limit,
                url_query_map.get(QUERY_PARAM_FROM).cloned(),
                url_query_map.get(QUERY_PARAM_TO).cloned(),
                url_query_map.get(QUERY_PARAM_TXN_HASH).cloned(),
                block_height,
                if is_txn_applied.is_some_and(|t| !t) {
                    Some(BACKSCAN_LIMIT)
                } else {
                    None
                },
                None,
                canonical,
                is_txn_applied,
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
                STATUS_SEARCH_OPTION_ALL.to_string(),
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
            set_data.set(Some(data.transactions));
            set_token.set(data.tokens.first().cloned().flatten());
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            metadata=Signal::derive(move || {
                let url_query_map = query_params_map.get();
                let indexes_not_available = url_query_map.get(QUERY_PARAM_HEIGHT).is_some()
                    || url_query_map
                        .get(QUERY_PARAM_TYPE)
                        .is_some_and(|q_type| { *q_type != TransactionKind::Zkapp.to_string() })
                    || url_query_map.get(QUERY_PARAM_FROM).is_some()
                    || url_query_map.get(QUERY_PARAM_TO).is_some()
                    || url_query_map.get(QUERY_PARAM_TXN_HASH).is_some();
                let indexes_available = !indexes_not_available;
                let is_zk_app = q_type_sig.get().is_some_and(|p| p == TransactionKind::Zkapp);
                let txn_all = txn_applied_sig.get().is_none();
                let applied_opt = txn_applied_sig
                    .get()
                    .is_some_and(|txn_applied| txn_applied == STATUS_SEARCH_OPTION_APPLIED);
                let is_canonical = txn_type_qp
                    .get()
                    .as_ref()
                    .map(|tt| tt != "Non-Canonical")
                    .unwrap_or(true);
                Some(
                    TableMetadataBuilder::new()
                        .displayed_records_value(
                            data_sig.get().map(|d| d.len() as u64).unwrap_or_default(),
                            None,
                        )
                        .available_records(
                            move || q_token_sig.get().is_some(),
                            token_sig
                                .get()
                                .and_then(|t| t.total_num_txns.try_into().ok())
                                .unwrap_or_default(),
                            None,
                        )
                        .available_records(
                            move || { indexes_available && !is_zk_app && is_canonical && txn_all },
                            summary_sig.get().total_num_canonical_user_commands,
                            None,
                        )
                        .available_records(
                            move || { indexes_available && !is_zk_app && !is_canonical && txn_all },
                            summary_sig.get().get_total_num_non_canonical_user_commands(),
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && !is_zk_app && is_canonical && applied_opt
                            },
                            summary_sig.get().total_num_applied_canonical_user_commands,
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && !is_zk_app && is_canonical && !applied_opt
                            },
                            summary_sig.get().total_num_failed_canonical_user_commands,
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && !is_zk_app && !is_canonical && applied_opt
                            },
                            summary_sig.get().get_total_num_non_canonical_applied_user_commands(),
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && !is_zk_app && !is_canonical && !applied_opt
                            },
                            summary_sig.get().get_total_num_non_canonical_failed_user_commands(),
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && is_zk_app && is_canonical && applied_opt
                            },
                            summary_sig.get().total_num_applied_canonical_zkapp_commands,
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && is_zk_app && is_canonical && !applied_opt
                            },
                            summary_sig.get().total_num_failed_canonical_zkapp_commands,
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && is_zk_app && !is_canonical && applied_opt
                            },
                            summary_sig.get().get_total_num_non_canonical_applied_zkapp_commands(),
                            None,
                        )
                        .available_records(
                            move || {
                                indexes_available && is_zk_app && !is_canonical && !applied_opt
                            },
                            summary_sig.get().get_total_num_non_canonical_failed_zkapp_commands(),
                            None,
                        )
                        .total_records_value(
                            summary_sig
                                .get()
                                .total_num_user_commands
                                .try_into()
                                .ok()
                                .unwrap_or_default(),
                            None,
                        )
                        .build(),
                )
            })

            is_loading=resource.loading()
            section_heading=MaybeSignal::derive(move || {
                token_sig
                    .get_untracked()
                    .and_then(|token| {
                        (token.token != MINA_TOKEN_ADDRESS)
                            .then(|| {
                                token.symbol.map(|symbol| symbol.to_string()).unwrap_or_default()
                            })
                    })
                    .map(|symbol| format!("User Commands ({symbol})"))
                    .unwrap_or("User Commands".to_string())
            })
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
            if visibility.get_untracked() != VisibilityState::Visible {
                logging::log!("Document not visible. Data polling skipped for user commands.");
                return Ok(transactions_query::ResponseData {
                    transactions: vec![],
                    other_transactions: vec![],
                    tokens: vec![],
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
                otherQps.remove(QUERY_PARAM_TXN_TYPE);
                otherQps.remove(QUERY_PARAM_TXN_APPLIED);
                otherQps.remove(QUERY_PARAM_ROW_LIMIT);
                Some(TableMetadata {
                    total_records: None,
                    available_records: None,
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    ..Default::default()
                })
            })

            is_loading=resource.loading()
            section_heading="Pending Commands"
        />
    }
}
