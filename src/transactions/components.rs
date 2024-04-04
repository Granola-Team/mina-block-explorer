use super::{functions::*, table_trait::TransactionsTrait};
use crate::{
    common::{components::*, functions::*, models::*, table::*},
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
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let resource = create_resource(
        move || (state_hash_sig.get(), canonical_qp.get()),
        move |(state_hash, canonical)| async move {
            load_data(50, None, None, state_hash, canonical).await
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
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

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
                                <BooleanUrlParamSelectMenu
                                    id="canonical-selection"
                                    query_str_key="canonical"
                                    labels=BooleanUrlParamSelectOptions {
                                        true_case: String::from("Canonical"),
                                        false_case: String::from("Non-Canonical"),
                                    }
                                />
                            }
                        }
                    >

                        {move || {
                            let pag = build_pagination(
                                data.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let subset = get_subset(
                                &data,
                                records_per_page,
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
