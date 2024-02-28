use leptos::*;
use leptos_router::*;

use super::functions::*;
use super::graphql::blocks_query::BlocksQueryBlocks;
use super::models::*;
use crate::account_dialog::components::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::spotlight::*;
use crate::common::table::*;
use crate::fee_transfers::components::BlockSpotlightFeeTransfersTable;
use crate::icons::*;
use crate::snarks::components::BlockSpotlightSnarkJobTable;

#[component]
pub fn BlockTabContainer(content: BlockContent) -> impl IntoView {
    let option_block = use_context::<ReadSignal<Option<BlocksQueryBlocks>>>()
        .expect("there to be an optional block signal provided");

    let content_for_fallback = content.clone();

    view! {
        <PageContainer>
            <ErrorBoundary fallback=move |_| view! { <NullView/> }>
                <Suspense fallback=move || {
                    let content_clone = content_for_fallback.clone();
                    match content_clone {
                        BlockContent::Spotlight => view! { <BlockSpotlightPlaceholder/> },
                        BlockContent::UserCommands => {
                            view! {
                                <TableSection
                                    section_heading="User Commands".to_string()
                                    controls=|| ().into_view()
                                >
                                    <Table data=LoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::SNARKJobs => {
                            view! {
                                <TableSection
                                    section_heading="SNARK Jobs".to_string()
                                    controls=|| ().into_view()
                                >
                                    <Table data=LoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::FeeTransfers => {
                            view! {
                                <TableSection
                                    section_heading="Fee Transfers".to_string()
                                    controls=|| ().into_view()
                                >
                                    <Table data=LoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                    }
                }>

                    {
                        let content_clone = content.clone();
                        move || {
                            match (option_block.get(), content_clone.clone()) {
                                (Some(block), BlockContent::Spotlight) => {
                                    view! { <BlockSpotlight block=block/> }
                                }
                                (Some(block), BlockContent::UserCommands) => {
                                    view! { <BlockUserCommands block=block/> }
                                }
                                (Some(block), BlockContent::SNARKJobs) => {
                                    view! { <BlockSnarkJobs block=block/> }
                                }
                                (Some(block), BlockContent::FeeTransfers) => {
                                    view! { <BlockFeeTransfers block=block/> }
                                }
                                _ => view! { <NullView/> },
                            }
                        }
                    }

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
pub fn BlockSnarkJobs(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        <TableSection section_heading="SNARK Jobs".to_string() controls=|| ().into_view()>
            <BlockSpotlightSnarkJobTable block_state_hash=Option::from(get_state_hash(&block))/>
        </TableSection>
    }
}

#[component]
pub fn BlockFeeTransfers(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        <TableSection section_heading="Fee Transfers".to_string() controls=|| ().into_view()>
            <BlockSpotlightFeeTransfersTable block_state_hash=Option::from(get_state_hash(&block))/>
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
            any_el: Some(convert_to_span(state_hash)),
            copiable: true,
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            any_el: Some(convert_to_span(get_previous_state_hash(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Staged Ledger Hash".to_string(),
            any_el: Some(convert_to_span(get_staged_ledger_hash(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Snarked Ledger Hash".to_string(),
            any_el: Some(convert_to_span(get_snarked_ledger_hash(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Coinbase".to_string(),
            any_el: Some(wrap_in_pill(decorate_with_currency_tag(get_coinbase(&block),"mina".to_string()),PillVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            any_el: Some(convert_to_span(get_coinbase_receiver(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Winning Account".to_string(),
            any_el: Some(convert_to_span(get_winning_account(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            any_el: Some(wrap_in_pill(decorate_with_currency_tag(get_snark_fees(&block),"mina".to_string()),PillVariant::Orange)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            any_el: Some(convert_to_pill(get_global_slot(&block),PillVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            any_el: Some(convert_to_pill(get_slot(&block),PillVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            any_el: Some(convert_to_pill(get_epoch(&block),PillVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            any_el: Some(wrap_in_pill(decorate_with_currency_tag(get_transaction_fees(&block),"mina".to_string()),PillVariant::Orange)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            any_el: Some(convert_to_pill(get_block_height(&block),PillVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            any_el: Some(wrap_in_pill(decorate_with_currency_tag(get_total_currency(&block),"mina".to_string()),PillVariant::Green)),
            ..Default::default()
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
fn BlockSpotlightPlaceholder() -> impl IntoView {
    let spotlight_items = vec![
        SpotlightEntry {
            label: "State Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Staged Ledger Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Snarked Ledger Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Winning Account".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            ..Default::default()
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
pub fn AccountDialogBlocksSection(blocks: Vec<Option<BlocksQueryBlocks>>) -> impl IntoView {
    let blocks_inner = blocks.clone();
    let has_blocks = move || !blocks.clone().is_empty();

    view! {
        <AccountDialogSectionContainer
            title=String::from("Block Production")
            showing_message=format!("Showing latest {} blocks", blocks_inner.len())
        >
            <Show
                when=has_blocks
                fallback=move || {
                    view! {
                        <EmptyTable message="This public key has no block production".to_string()/>
                    }
                }
            >

                {blocks_inner
                    .iter()
                    .map(|opt_block| {
                        let check_block = opt_block.clone();
                        let block = opt_block.clone().unwrap();
                        view! {
                            <Show
                                when=move || check_block.is_some()
                                fallback=move || view! { <NullView/> }
                            >

                                {
                                    let moments_ago = print_time_since(&get_date_time(&block));
                                    let date_time = get_date_time(&block);
                                    let status = get_status(&date_time);
                                    view! {
                                        <AccountDialogSectionEntryHeader
                                            status=status
                                            date=date_time
                                            moments_ago=moments_ago
                                        />
                                        <AccountDialogBlockEntry block=block.clone()/>
                                        <AccountDialogEntryDivider/>
                                    }
                                        .into_view()
                                }

                            </Show>
                        }
                    })
                    .collect::<Vec<_>>()}
            </Show>
        </AccountDialogSectionContainer>
    }
}

struct SubEntry {
    label: String,
    value: String,
}

#[component]
fn AccountDialogBlockEntry(block: BlocksQueryBlocks) -> impl IntoView {
    let sub_entries = vec![
        SubEntry {
            label: String::from("Hash"),
            value: get_state_hash(&block),
        },
        SubEntry {
            label: String::from("Coinbase"),
            value: get_coinbase(&block),
        },
    ];
    view! {
        <div class="w-full flex justify-between">
            {sub_entries
                .into_iter()
                .map(|se| view! { <AccountDialogSectionSubEntry label=se.label value=se.value/> })
                .collect::<Vec<_>>()}
        </div>
    }
    .into_view()
}

#[component]
pub fn BlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let public_key = value.get("account");
            let block_hash = value.get("query");
            let include_non_canonical_qs = value.get("include_non_canonical");
            let canonical_query = canonical_qs_to_canonical_query_param(include_non_canonical_qs);
            load_data(
                50,
                public_key.cloned(),
                block_hash.cloned(),
                canonical_query,
            )
            .await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                let pag = build_pagination(
                    data.blocks.len(),
                    records_per_page,
                    current_page.get(),
                    set_current_page,
                );
                let blocks_subset = get_subset(
                    &data.blocks,
                    records_per_page,
                    current_page.get() - 1,
                );
                view! {
                    <TableSection
                        section_heading="Blocks".to_owned()
                        controls=move || {
                            view! {
                                <URLCheckbox
                                    label="Include Non-Canonical".to_string()
                                    url_param_key="include_non_canonical".to_string()
                                />
                            }
                        }
                    >

                        <Table data=blocks_subset pagination=pag/>
                    </TableSection>
                    <Outlet/>
                }
                    .into_view()
            }
            None => {
                view! {
                    <TableSection
                        section_heading="Blocks".to_owned()
                        controls=move || view! { <NullView/> }
                    >
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                    <Outlet/>
                }
                    .into_view()
            }
            _ => view! { <span></span> }.into_view(),
        }}
    }
}

#[component]
pub fn SummaryPageBlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();
    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let state_hash = value.get("query");
            let include_non_canonical_qs = value.get("include_non_canonical");
            let canonical_query = canonical_qs_to_canonical_query_param(include_non_canonical_qs);
            load_data(50, None, state_hash.cloned(), canonical_query).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <ErrorBoundary fallback=move |_| view! { <NullView/> }>
            <Suspense fallback=move || {
                view! {
                    <TableSection
                        section_heading="Blocks".to_string()
                        controls=move || view! { <NullView/> }
                    >
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }>
                {move || {
                    resource
                        .get()
                        .and_then(|res| res.ok())
                        .map(|data| {
                            let pag = build_pagination(
                                data.blocks.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let blocks_subset = get_subset(
                                &data.blocks,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! {
                                <TableSection
                                    section_heading="Blocks".to_owned()
                                    controls=move || {
                                        view! {
                                            <URLCheckbox
                                                label="Include Non-Canonical".to_string()
                                                url_param_key="include_non_canonical".to_string()
                                            />
                                        }
                                    }
                                >

                                    <Table
                                        data=SummaryPageBlocksQueryBlocks(blocks_subset)
                                        pagination=pag
                                    />
                                </TableSection>
                            }
                        })
                }}

            </Suspense>
        </ErrorBoundary>
        <Outlet/>
    }
}

#[component]
pub fn AccountOverviewBlocksTable(public_key: Option<String>) -> impl IntoView {
    let pk = public_key.clone();
    let resource = create_resource(
        || (),
        move |_| {
            let public_key_inner = public_key.clone();
            async move { load_data(50, public_key_inner, None, Some(true)).await }
        },
    );

    let (href, _set_href) = create_signal(
        pk.as_ref()
            .map(|pk| format!("/blocks?account={}", pk))
            .unwrap_or_else(|| "/blocks".to_string()),
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                view! {
                    {match data.blocks.len() {
                        0 => {
                            view! {
                                <EmptyTable message="This public key has no block production"
                                    .to_string()/>
                            }
                        }
                        _ => {
                            {
                                let pag = build_pagination(
                                    data.blocks.len(),
                                    records_per_page,
                                    current_page.get(),
                                    set_current_page,
                                );
                                let blocks_subset = get_subset(
                                    &data.blocks,
                                    records_per_page,
                                    current_page.get() - 1,
                                );
                                view! {
                                    <Table data=blocks_subset pagination=pag/>
                                    <TableLink
                                        href=href.get()
                                        text="See all block production".to_string()
                                    >
                                        <BlockIcon/>
                                    </TableLink>
                                }
                            }
                                .into_view()
                        }
                    }}
                }
            }
            _ => view! { <span></span> }.into_view(),
        }}
    }
}
