use super::{components::*, functions::*, table_trait::*};
use crate::{
    common::{components::*, functions::*, models::*, search::*, spotlight::*},
    icons::*,
};
use leptos::*;
use leptos_meta::Title;
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
        <SearchBar placeholder="Exact search by state hash".to_string()/>
        <Title text="Transactions | Search For Transactions"/>
        <PageContainer>
            {move || {
                let qp_map = query_params_map.get();
                view! { <TransactionsSection state_hash=qp_map.get("query").cloned()/> }
            }}

        </PageContainer>
    }
}

#[component]
pub fn TransactionSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");
    let (trx_memo, set_trx_memo) = create_signal("No Memo".to_string());
    let resource = create_resource(
        move || (memo_params_map.get(), canonical_qp.get()),
        |(value, canonical)| async move {
            let state_hash = value.get("id");
            load_data(1, None, None, state_hash.cloned(), canonical).await
        },
    );

    create_effect(move |_| {
        if let Some(Ok(data)) = resource.get() {
            if let Some(Some(trx)) = data.transactions.first() {
                set_trx_memo.set(trx.get_memo());
            }
        }
    });

    view! {
        <Title
            formatter=move |text| format!("Transaction Overview | {text}")
            text=move || trx_memo.get()
        />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    match data.transactions.first() {
                        Some(Some(transaction)) => {
                            let state_hash = transaction.get_hash();
                            let date_time = transaction.get_block_datetime();
                            let spotlight_items = vec![
                                SpotlightEntry {
                                    label: "Date".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_block_datetime())),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Transaction Hash".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_hash())),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Payment ID".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_payment_id())),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Block Height".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_block_height())),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Canonical".to_string(),
                                    any_el: Some(
                                        convert_to_pill(
                                            transaction.get_canonical().unwrap_or_default().to_string(),
                                            ColorVariant::Grey,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Block State Hash".to_string(),
                                    any_el: Some(
                                        convert_to_span(transaction.get_block_state_hash()),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Amount".to_string(),
                                    any_el: Some(
                                        wrap_in_pill(
                                            decorate_with_currency_tag(
                                                transaction.get_amount(),
                                                "mina".to_string(),
                                            ),
                                            ColorVariant::Green,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Fee".to_string(),
                                    any_el: Some(
                                        wrap_in_pill(
                                            decorate_with_currency_tag(
                                                transaction.get_fee(),
                                                "mina".to_string(),
                                            ),
                                            ColorVariant::Orange,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "From".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_from())),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "To".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_to())),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Nonce".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Memo".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_memo(), ColorVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Kind".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
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
