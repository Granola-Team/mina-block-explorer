use leptos::*;
use leptos_router::*;

use super::components::*;
use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::search::*;
use crate::common::spotlight::*;
use crate::icons::*;

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
                                    value: Some(get_block_datetime(transaction)),
                                    pill_variant: None,
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Transaction Hash".to_string(),
                                    value: Some(get_hash(transaction)),
                                    pill_variant: None,
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Payment ID".to_string(),
                                    value: Some(get_payment_id(transaction)),
                                    pill_variant: None,
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Block Height".to_string(),
                                    value: Some(get_block_height(transaction)),
                                    pill_variant: None,
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Canonical".to_string(),
                                    value: Some(get_canonical(transaction)),
                                    pill_variant: Some(PillVariant::Grey),
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Block State Hash".to_string(),
                                    value: Some(get_block_state_hash(transaction)),
                                    pill_variant: None,
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Amount".to_string(),
                                    value: Some(get_amount(transaction)),
                                    pill_variant: Some(PillVariant::Green),
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Fee".to_string(),
                                    value: Some(get_fee(transaction)),
                                    pill_variant: Some(PillVariant::Orange),
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "From".to_string(),
                                    value: Some(get_from(transaction)),
                                    pill_variant: None,
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "To".to_string(),
                                    value: Some(get_to(transaction)),
                                    pill_variant: None,
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Nonce".to_string(),
                                    value: Some(get_nonce(transaction)),
                                    pill_variant: Some(PillVariant::Grey),
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Memo".to_string(),
                                    value: Some(get_memo(transaction)),
                                    pill_variant: Some(PillVariant::Grey),
                                    copiable: false,
                                },
                                SpotlightEntry {
                                    label: "Kind".to_string(),
                                    value: Some(get_kind(transaction)),
                                    pill_variant: Some(PillVariant::Grey),
                                    copiable: false,
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
                            value: None,
                            pill_variant: None,
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Transaction Hash".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: true,
                        },
                        SpotlightEntry {
                            label: "Payment ID".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: true,
                        },
                        SpotlightEntry {
                            label: "Block Height".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Canonical".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Block State Hash".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: true,
                        },
                        SpotlightEntry {
                            label: "Amount".to_string(),
                            value: None,
                            pill_variant: Some(PillVariant::Green),
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Fee".to_string(),
                            value: None,
                            pill_variant: Some(PillVariant::Orange),
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "From".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: true,
                        },
                        SpotlightEntry {
                            label: "To".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: true,
                        },
                        SpotlightEntry {
                            label: "Nonce".to_string(),
                            value: None,
                            pill_variant: Some(PillVariant::Grey),
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Memo".to_string(),
                            value: None,
                            pill_variant: Some(PillVariant::Grey),
                            copiable: false,
                        },
                        SpotlightEntry {
                            label: "Kind".to_string(),
                            value: None,
                            pill_variant: None,
                            copiable: false,
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
