use super::{components::*, functions::*};
use crate::{
    common::{components::*, functions::*, models::*, search::*, spotlight::*},
    icons::*,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn TransactionTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/transactions".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/transactions/token".to_string(),
            text: "Token Transactions".to_string(),
            icon: NavIcon::Tokens,
            disabled: true,
            ..Default::default()
        },
        NavEntry {
            href: "/transactions/zkapp".to_string(),
            text: "zkApp Transactions".to_string(),
            icon: NavIcon::ZKApps,
            disabled: true,
            ..Default::default()
        },
    ];

    view! { <TabbedPage tabs=tabs/> }
}

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let query_params_map: Memo<ParamsMap> = use_query_map();

    view! {
        <SearchBar placeholder="Exact search by payment ID".to_string()/>
        <PageContainer>
            {move || {
                let qp_map = query_params_map.get();
                view! {
                    <TransactionsSection
                        public_key=qp_map.get("public_key").cloned()
                        payment_id=qp_map.get("query").cloned()
                    />
                }
            }}

        </PageContainer>
    }
}

#[component]
pub fn TransactionSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(10, None, state_hash.cloned(), None).await
        },
    );

    view! {
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    match data.transactions.first() {
                        Some(Some(transaction)) => {
                            let state_hash = get_hash(transaction);
                            let date_time = get_block_datetime(transaction);
                            let spotlight_items = vec![
                                SpotlightEntry {
                                    label: "Date".to_string(),
                                    any_el: Some(convert_to_span(get_block_datetime(transaction))),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Transaction Hash".to_string(),
                                    any_el: Some(convert_to_span(get_hash(transaction))),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Payment ID".to_string(),
                                    any_el: Some(convert_to_span(get_payment_id(transaction))),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Block Height".to_string(),
                                    any_el: Some(convert_to_span(get_block_height(transaction))),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Canonical".to_string(),
                                    any_el: Some(
                                        convert_to_pill(
                                            get_canonical(transaction),
                                            PillVariant::Grey,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Block State Hash".to_string(),
                                    any_el: Some(
                                        convert_to_span(get_block_state_hash(transaction)),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Amount".to_string(),
                                    any_el: Some(
                                        wrap_in_pill(
                                            decorate_with_currency_tag(
                                                get_amount(transaction),
                                                "mina".to_string(),
                                            ),
                                            PillVariant::Green,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Fee".to_string(),
                                    any_el: Some(
                                        wrap_in_pill(
                                            decorate_with_currency_tag(
                                                get_fee(transaction),
                                                "mina".to_string(),
                                            ),
                                            PillVariant::Orange,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "From".to_string(),
                                    any_el: Some(convert_to_span(get_from(transaction))),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "To".to_string(),
                                    any_el: Some(convert_to_span(get_to(transaction))),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Nonce".to_string(),
                                    any_el: Some(
                                        convert_to_pill(get_nonce(transaction), PillVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Memo".to_string(),
                                    any_el: Some(
                                        convert_to_pill(get_memo(transaction), PillVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Kind".to_string(),
                                    any_el: Some(
                                        convert_to_pill(get_kind(transaction), PillVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                            ];
                            view! {
                                <SpotlightSection
                                    header="Transaction Spotlight".to_string()
                                    spotlight_items=spotlight_items
                                    id=Some(state_hash)
                                    meta=Some(
                                        format!("{} ({})", date_time, print_time_since(&date_time)),
                                    )
                                >

                                    <TransactionIcon width=40/>
                                </SpotlightSection>
                            }
                                .into_view()
                        }
                        _ => view! { <NullView/> },
                    }
                }
                None => {
                    let spotlight_items = vec![
                        SpotlightEntry {
                            label: "Date".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Transaction Hash".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Payment ID".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Block Height".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Canonical".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Block State Hash".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Amount".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Fee".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "From".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "To".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Nonce".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Memo".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Kind".to_string(),
                            ..Default::default()
                        },
                    ];
                    view! {
                        <SpotlightSection
                            header="Transaction Spotlight".to_string()
                            spotlight_items=spotlight_items
                            id=None
                            meta=None
                        >
                            <TransactionIcon width=40/>
                        </SpotlightSection>
                    }
                        .into_view()
                }
                _ => view! { <NullView/> },
            }}

        </PageContainer>
    }
}
