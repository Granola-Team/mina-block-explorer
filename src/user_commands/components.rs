use super::functions::*;
use crate::{
    common::{components::*, constants::*, models::*, table::*},
    summary::models::BlockchainSummary,
    user_commands::graphql::transactions_query,
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
const QP_HEIGHT: &str = "q-height";
const QP_FROM: &str = "q-from";
const QP_TO: &str = "q-to";

#[component]
pub fn TransactionsSection() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let visibility = use_document_visibility();
    let (data_sig, set_data) = create_signal(None);
    let (txn_type_qp, _) = create_query_signal::<String>(QP_TXN_TYPE);
    let (row_limit_sig, _) = create_query_signal::<u64>("row-limit");
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
            )
        },
        move |(_, url_query_map, txn_type, block_height, row_limit)| async move {
            if visibility.get() == VisibilityState::Visible {
                match txn_type {
                    Some(ref txn_type_str) if txn_type_str == "Pending" => load_pending_txn().await,
                    Some(ref txn_type_str) if txn_type_str == "Canonical" => {
                        load_data(
                            row_limit,
                            url_query_map.get(QP_FROM).cloned(),
                            url_query_map.get(QP_TO).cloned(),
                            url_query_map.get(QP_TXN_HASH).cloned(),
                            block_height,
                            None,
                            Some(true),
                        )
                        .await
                    }
                    Some(ref txn_type_str) if txn_type_str == "Non-Canonical" => {
                        load_data(
                            row_limit,
                            url_query_map.get(QP_FROM).cloned(),
                            url_query_map.get(QP_TO).cloned(),
                            url_query_map.get(QP_TXN_HASH).cloned(),
                            block_height,
                            None,
                            Some(false),
                        )
                        .await
                    }
                    Some(_) | None => {
                        load_data(
                            row_limit,
                            url_query_map.get(QP_FROM).cloned(),
                            url_query_map.get(QP_TO).cloned(),
                            url_query_map.get(QP_TXN_HASH).cloned(),
                            block_height,
                            None,
                            Some(true),
                        )
                        .await
                    }
                }
            } else {
                logging::log!("Document not visible. Data polling skipped for user commands.");
                Ok(transactions_query::ResponseData {
                    transactions: data_sig.get().unwrap_or_default(),
                    other_transactions: vec![],
                })
            }
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
            column: "Age".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
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
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_user_commands).ok(),
                    available_records: None,
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
                        <UrlParamSelectMenu
                            label="Rows"
                            id="row-limit"
                            query_str_key="row-limit"
                            labels=UrlParamSelectOptions {
                                is_boolean_option: false,
                                cases: vec![
                                    "25".to_string(),
                                    "50".to_string(),
                                    "100".to_string(),
                                    "250".to_string(),
                                ],
                            }
                        />
                    </div>
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="txn-type"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: false,
                            cases: vec![
                                "Canonical".to_string(),
                                "Non-Canonical".to_string(),
                                "Pending".to_string(),
                            ],
                        }
                    />
                }
            }
        />
    }
}
