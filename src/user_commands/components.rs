use super::{functions::*, table_trait::TransactionsTrait};
use crate::{
    common::{
        components::*,
        constants::{TABLE_RECORD_SIZE, *},
        functions::*,
        models::*,
        table::*,
    },
    icons::*,
};
use leptos::*;
use leptos_router::*;

const QP_TXN_HASH: &str = "q-txn-hash";
const QP_TXN_TYPE: &str = "txn-type";
const QP_HEIGHT: &str = "q-height";
const QP_FROM: &str = "q-from";
const QP_TO: &str = "q-to";

#[component]
pub fn TransactionsSection(#[prop(default = false)] with_link: bool) -> impl IntoView {
    let (txn_type_qp, _) = create_query_signal::<String>(QP_TXN_TYPE);
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<i64>(QP_HEIGHT);

    let resource = create_resource(
        move || {
            (
                query_params_map.get(),
                txn_type_qp.get(),
                block_height_sig.get(),
            )
        },
        move |(url_query_map, txn_type, block_height)| async move {
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

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                view! {
                    <TransactionSection
                        transactions=Some(data.transactions)
                        with_link
                        public_key=None
                    />
                }
            }
            _ => {
                view! {
                    <TableSection section_heading="User Commands" controls=|| ().into_view()>
                        <DeprecatedTable data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        }}
    }
}

#[component]
fn TransactionSection<T>(
    #[prop(into)] public_key: Option<String>,
    #[prop(default = false)] with_link: bool,
    transactions: Option<Vec<Option<T>>>,
) -> impl IntoView
where
    T: TransactionsTrait + Clone + 'static,
    Vec<Option<T>>: TableData,
{
    let (pk, _set_public_key) = create_signal(public_key);
    let (current_page, set_current_page) = create_signal(1);
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        // we know we just provided this in the parent component
        .expect("there to be a `PageDimensions` signal provided");

    let transactions_show_condition = transactions.clone();
    let transactions_inner = transactions.clone();
    view! {
        <Show
            when=move || transactions_show_condition.is_some()
            fallback=move || {
                view! {
                    <TableSection section_heading="User Commands" controls=|| ().into_view()>
                        <DeprecatedTable data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        >

            {
                let data = transactions_inner.clone().unwrap();
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

                        {move || {
                            let pag = build_pagination(
                                data.len(),
                                TABLE_DEFAULT_PAGE_SIZE,
                                current_page.get(),
                                set_current_page,
                                page_dim.get().height.map(|h| h as usize),
                                Some(
                                    Box::new(|container_height: usize| {
                                        (container_height
                                            - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                                            / ESTIMATED_ROW_HEIGHT
                                    }),
                                ),
                            );
                            let subset = get_subset(
                                &data,
                                pag.records_per_page,
                                current_page.get() - 1,
                            );
                            view! {
                                <DeprecatedTable data=subset pagination=pag/>
                                <Show
                                    when=move || pk.get().is_some() && with_link
                                    fallback=move || ().into_view()
                                >

                                    {
                                        let pk_i = pk.get().unwrap();
                                        view! {
                                            <TableLink
                                                href=format!("/commands?account={}", pk_i)
                                                text="See all transactions"
                                            >
                                                <TransactionIcon/>
                                            </TableLink>
                                        }
                                    }

                                </Show>
                            }
                        }}

                    </TableSection>
                }
            }

        </Show>
    }
}
