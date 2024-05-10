use super::functions::*;
use crate::common::{
    components::*,
    constants::{TABLE_RECORD_SIZE, *},
    functions::*,
    models::*,
    table::*,
};
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
    let (txn_type_qp, _) = create_query_signal::<String>(QP_TXN_TYPE);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<i64>(QP_HEIGHT);
    let (current_page, set_current_page) = create_signal(1);
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
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
                        TABLE_RECORD_SIZE,
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
                        TABLE_RECORD_SIZE,
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
                        TABLE_RECORD_SIZE,
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
    let get_data_and_pagination = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.transactions.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|h| h as usize),
                Some(Box::new(|container_height: usize| {
                    (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                        / ESTIMATED_ROW_HEIGHT
                })),
            );
            (data, pag)
        })
    };

    view! {
        <TableSection
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
                            <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                        }
                    }>
                        {move || {
                            get_data_and_pagination()
                                .map(|(data, pag)| {
                                    view! {
                                        <TableRows data=data
                                            .transactions[pag.start_index()-1..pag.end_index()]
                                            .to_vec()/>
                                    }
                                })
                        }}

                    </Suspense>
                </Table>
                {move || {
                    get_data_and_pagination()
                        .map(|(_, pag)| {
                            view! { <Pagination pagination=pag/> }
                        })
                }}

            </TableContainer>

        </TableSection>
    }
}
