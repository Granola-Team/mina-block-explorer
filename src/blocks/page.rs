use super::components::*;
use super::functions::*;
use crate::blocks::graphql::blocks_query::BlocksQueryBlocks;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::search::*;
use crate::common::spotlight::*;
use crate::common::table::Table;
use crate::common::table::TableSection;
use crate::icons::*;

use leptos::ErrorBoundary;
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

#[derive(Clone)]
enum BlockContent {
    Spotlight,
    UserCommands,
    FeeTransfers,
    ZKApps,
}

#[component]
pub fn BlockSpotlightTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::Spotlight/> }
}

#[component]
pub fn BlockUserCommandsTab() -> impl IntoView {
    view! { <BlockTabContainer content=BlockContent::UserCommands/> }
}

#[component]
pub fn BlockTabContainer(content: BlockContent) -> impl IntoView {
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

                    {resource
                        .get()
                        .and_then(|res| res.ok())
                        .and_then(|res| res.blocks.first().cloned().unwrap_or_default())
                        .map(|block| {
                            let content_clone = content.clone();
                            {
                                match content_clone {
                                    BlockContent::Spotlight => {
                                        view! { <BlockSpotlight block=block/> }
                                    }
                                    BlockContent::UserCommands => {
                                        view! { <BlockUserCommands block=block/> }
                                    }
                                    BlockContent::FeeTransfers => todo!(),
                                    BlockContent::ZKApps => todo!(),
                                }
                            }
                        })}

                </Suspense>
            </ErrorBoundary>
        </PageContainer>
    }
}

#[component]
pub fn BlockUserCommands(block: BlocksQueryBlocks) -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    view! {
        <TableSection section_heading="User Commands".to_string() controls=|| ().into_view()>

            {move || match get_user_commands(&block) {
                Some(user_commands) => {
                    let pag = build_pagination(
                        user_commands.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &user_commands,
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }
                None => view! { <NullView/> },
            }}

        </TableSection>
    }
}

#[component]
pub fn BlockSpotlight(block: BlocksQueryBlocks) -> impl IntoView {
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
            meta=Some(format!("{} ({})", date_time, print_time_since(&date_time)))
        >

            <BlockIcon width=40/>
        </SpotlightSection>
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
