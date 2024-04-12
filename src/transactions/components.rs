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

#[component]
pub fn TransactionsSection(
    #[prop(default = None,into)] state_hash: Option<String>,
    #[prop(default = false)] with_link: bool,
) -> impl IntoView {
    let (state_hash_sig, _) = create_signal(state_hash);
    let (txn_type_qp, _) = create_query_signal::<String>("txn-type");

    let resource = create_resource(
        move || (state_hash_sig.get(), txn_type_qp.get()),
        move |(state_hash, txn_type)| async move {
            match txn_type {
                Some(ref txn_type_str) if txn_type_str == "Pending" => load_pending_txn().await,
                Some(ref txn_type_str) if txn_type_str == "Canonical" => {
                    load_data(TABLE_RECORD_SIZE, None, None, state_hash, Some(true)).await
                }
                Some(ref txn_type_str) if txn_type_str == "Non-Canonical" => {
                    load_data(TABLE_RECORD_SIZE, None, None, state_hash, Some(false)).await
                }
                Some(_) | None => {
                    load_data(TABLE_RECORD_SIZE, None, None, state_hash, Some(true)).await
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
                    <TableSection section_heading="Transactions" controls=|| ().into_view()>
                        <Table data=LoadingPlaceholder {}/>
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
                    <TableSection section_heading="Transactions" controls=|| ().into_view()>
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        >

            {
                let data = transactions_inner.clone().unwrap();
                view! {
                    <TableSection
                        section_heading="Transactions"
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
                                        (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
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
                                <Table data=subset pagination=pag/>
                                <Show
                                    when=move || pk.get().is_some() && with_link
                                    fallback=move || ().into_view()
                                >

                                    {
                                        let pk_i = pk.get().unwrap();
                                        view! {
                                            <TableLink
                                                href=format!("/transactions?account={}", pk_i)
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
