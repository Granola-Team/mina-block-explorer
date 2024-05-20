use super::functions::*;
use crate::common::{components::*, constants::*, models::*, table::*};
use leptos::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};

const QP_TXN_HASH: &str = "q-txn-hash";
const QP_TXN_TYPE: &str = "txn-type";
const QP_HEIGHT: &str = "q-height";
const QP_FROM: &str = "q-from";
const QP_TO: &str = "q-to";

#[component]
pub fn TransactionsSection() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    let (txn_type_qp, _) = create_query_signal::<String>(QP_TXN_TYPE);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<i64>(QP_HEIGHT);
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                txn_type_qp.get(),
                block_height_sig.get(),
            )
        },
        move |(_, url_query_map, txn_type, block_height)| async move {
            match txn_type {
                Some(ref txn_type_str) if txn_type_str == "Pending" => load_pending_txn().await,
                Some(ref txn_type_str) if txn_type_str == "Canonical" => {
                    load_data(
                        TABLE_ROW_LIMIT,
                        url_query_map.get(QP_FROM).cloned(),
                        url_query_map.get(QP_TO).cloned(),
                        url_query_map.get(QP_TXN_HASH).cloned(),
                        block_height,
                        Some(true),
                    )
                    .await
                }
                Some(ref txn_type_str) if txn_type_str == "Non-Canonical" => {
                    load_data(
                        TABLE_ROW_LIMIT,
                        url_query_map.get(QP_FROM).cloned(),
                        url_query_map.get(QP_TO).cloned(),
                        url_query_map.get(QP_TXN_HASH).cloned(),
                        block_height,
                        Some(false),
                    )
                    .await
                }
                Some(_) | None => {
                    load_data(
                        TABLE_ROW_LIMIT,
                        url_query_map.get(QP_FROM).cloned(),
                        url_query_map.get(QP_TO).cloned(),
                        url_query_map.get(QP_TXN_HASH).cloned(),
                        block_height,
                        Some(true),
                    )
                    .await
                }
            }
        },
    );

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Txn Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Type".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "From".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "To".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Nonce".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Fee".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Amount".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let get_data = move || resource.get().and_then(|res| res.ok());

    view! {
        <TableSection
            metadata
            section_heading="User Commands"
            controls=move || {
                view! {
                    <UrlParamSelectMenu
                        id="transaction-type-selection"
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
        >

            <TableContainer>
                <Table>
                    <TableHeader columns=table_columns/>
                    <Suspense fallback=move || {
                        view! {
                            <TableRows data=vec![
                                vec![LoadingPlaceholder; table_cols_length];
                                TABLE_ROW_LIMIT.try_into().unwrap()
                            ]/>
                        }
                    }>
                        {move || {
                            get_data()
                                .map(|data| {
                                    view! { <TableRows data=data.transactions/> }
                                })
                        }}

                    </Suspense>
                </Table>
            </TableContainer>
        </TableSection>
    }
}
