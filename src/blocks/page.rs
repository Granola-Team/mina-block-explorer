use super::components::*;
use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::search::*;
use crate::common::spotlight::*;
use crate::common::table::*;
use crate::icons::*;

use leptos::*;
use leptos_router::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! {
        <SearchBar placeholder="Exact search for block hash".to_string()/>
        <PageContainer>
            <BlocksSection/>
        </PageContainer>
    }
}

#[component]
fn BlockSpotlightPlaceholder() -> impl IntoView {
    let spotlight_items = vec![
        SpotlightEntry {
            label: "State Hash".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "Staged Ledger Hash".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "Snarked Ledger Hash".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "Coinbase".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Green),
            copiable: false,
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "Winning Account".to_string(),
            value: None,
            pill_variant: None,
            copiable: true,
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Orange),
            copiable: false,
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Grey),
            copiable: false,
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Grey),
            copiable: false,
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Grey),
            copiable: false,
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Orange),
            copiable: false,
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Grey),
            copiable: false,
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            value: None,
            pill_variant: Some(PillVariant::Green),
            copiable: false,
        },
    ];
    view! {
        <SpotlightSection
            header="Block Spotlight".to_string()
            spotlight_items=spotlight_items
            id=None
            meta=None
        >
            <BlockIcon width=40/>
        </SpotlightSection>
    }
}

#[component]
pub fn BlockSpotlight() -> impl IntoView {
    let memo_params_map = use_params_map();
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(50, None, state_hash.cloned(), None).await
        },
    );
    view! {
        <PageContainer>
            <ErrorBoundary fallback=move |_| view! { <NullView/> }>
                <Suspense fallback=move || {
                    view! { <BlockSpotlightPlaceholder/> }
                }>
                    {move || {
                        resource
                            .get()
                            .and_then(|res| res.ok())
                            .and_then(|res| res.blocks.first().cloned().unwrap_or_default())
                            .map(|block| {
                                let state_hash = get_state_hash(&block);
                                let date_time = get_date_time(&block);
                                let spotlight_items = vec![
                                    SpotlightEntry {
                                        label: "State Hash".to_string(),
                                        value: Some(state_hash),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "Previous State Hash".to_string(),
                                        value: Some(get_previous_state_hash(&block)),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "Staged Ledger Hash".to_string(),
                                        value: Some(get_staged_ledger_hash(&block)),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "Snarked Ledger Hash".to_string(),
                                        value: Some(get_snarked_ledger_hash(&block)),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "Coinbase".to_string(),
                                        value: Some(get_coinbase(&block)),
                                        pill_variant: Some(PillVariant::Green),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Coinbase Receiver".to_string(),
                                        value: Some(get_coinbase_receiver(&block)),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "Winning Account".to_string(),
                                        value: Some(get_winning_account(&block)),
                                        pill_variant: None,
                                        copiable: true,
                                    },
                                    SpotlightEntry {
                                        label: "SNARK Fees".to_string(),
                                        value: Some(get_snark_fees(&block)),
                                        pill_variant: Some(PillVariant::Orange),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Global Slot".to_string(),
                                        value: Some(get_global_slot(&block)),
                                        pill_variant: Some(PillVariant::Grey),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Slot".to_string(),
                                        value: Some(get_slot(&block)),
                                        pill_variant: Some(PillVariant::Grey),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Epoch".to_string(),
                                        value: Some(get_epoch(&block)),
                                        pill_variant: Some(PillVariant::Grey),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Transaction Fees".to_string(),
                                        value: Some(get_transaction_fees(&block)),
                                        pill_variant: Some(PillVariant::Orange),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Blockchain Length".to_string(),
                                        value: Some(get_block_height(&block)),
                                        pill_variant: Some(PillVariant::Grey),
                                        copiable: false,
                                    },
                                    SpotlightEntry {
                                        label: "Total Currency".to_string(),
                                        value: Some(get_total_currency(&block)),
                                        pill_variant: Some(PillVariant::Green),
                                        copiable: false,
                                    },
                                ];
                                view! {
                                    <SpotlightSection
                                        header="Block Spotlight".to_string()
                                        spotlight_items=spotlight_items
                                        id=Some(get_state_hash(&block))
                                        meta=Some(
                                            format!("{} ({})", date_time, print_time_since(&date_time)),
                                        )
                                    >

                                        <BlockIcon width=40/>
                                    </SpotlightSection>
                                }
                            })
                    }}

                </Suspense>
            </ErrorBoundary>
        </PageContainer>
    }
}

#[component]
pub fn BlockTabbedPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());
    let tabs = move || {
        vec![
            NavEntry {
                href: format!("/blocks/{}/spotlight", id()),
                text: "Block Spotlight".to_string(),
                icon: NavIcon::Blocks,
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/user-commands", id()),
                text: "User Commands".to_string(),
                icon: NavIcon::Transactions,
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/snark-jobs", id()),
                text: "SNARK Jobs".to_string(),
                icon: NavIcon::SNARKs,
                sub_entries: None,
                disabled: false,
            },
            NavEntry {
                href: format!("/blocks/{}/fee-transfers", id()),
                text: "Fee Transfers".to_string(),
                icon: NavIcon::FeeTransfers,
                sub_entries: None,
                disabled: false,
            },
        ]
    };
    view! { <TabbedPage tabs=tabs()/> }
}
