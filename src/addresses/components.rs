use crate::{
    account_dialog::{functions::load_data, models::AccountActivityQueryDirectionalTransactions},
    common::{components::*, functions::*, models::*, table::*},
};
use leptos::*;
use leptos_router::*;
#[component]
pub fn AccountTransactionsSection(public_key: Option<String>) -> impl IntoView {
    let (pk, _set_public_key) = create_signal(public_key);
    let (transactions, set_transactions) = create_signal(None);
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    let account_activity_resource = create_resource(
        move || (pk.get(), canonical_qp.get()),
        |(id_opt, canonical)| async move {
            if let Some(id) = id_opt {
                load_data(
                    Some(id),
                    None,
                    None,
                    Some(100),
                    Some(canonical.unwrap_or(true)),
                )
                .await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    create_effect(move |_| {
        if let Some(res) = account_activity_resource.get().and_then(|res| res.ok()) {
            let mut transactions: Vec<_> = res
                .incoming_transactions
                .into_iter()
                .filter(|t| t.is_some())
                .map(|r| r.map(|t| t.into()))
                .chain(
                    res.outgoing_transactions
                        .into_iter()
                        .filter(|t| t.is_some())
                        .map(|r| r.map(|t| t.into())),
                )
                .collect();
            transactions.sort_by(|a, b| {
                match (
                        <std::option::Option<
                            AccountActivityQueryDirectionalTransactions,
                        > as Clone>::clone(a)
                            .unwrap()
                            .date_time,
                        <std::option::Option<
                            AccountActivityQueryDirectionalTransactions,
                        > as Clone>::clone(b)
                            .unwrap()
                            .date_time,
                    ) {
                        (Some(date_time_a), Some(date_time_b)) => {
                            date_time_b.cmp(&date_time_a)
                        }
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
            });
            set_transactions.set(Some(transactions));
        };
    });

    view! {
        <ErrorBoundary fallback=move |_| {
            view! { <NullView/> }
        }>

            {move || match transactions.get() {
                Some(data) => {
                    view! {
                        <TableSection
                            section_heading="Transactions".to_owned()
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
                                view! { <Table data=subset pagination=pag/> }
                            }}

                        </TableSection>
                    }
                }
                None => {
                    view! {
                        <TableSection
                            section_heading="Transactions".to_string()
                            controls=|| ().into_view()
                        >
                            <Table data=LoadingPlaceholder {}/>
                        </TableSection>
                    }
                }
            }}

        </ErrorBoundary>
    }
}
