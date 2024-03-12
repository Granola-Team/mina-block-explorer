use super::{functions::*, graphql::blocks_query::BlocksQueryBlocks, models::*};
use crate::{
    account_dialog::components::*,
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    fee_transfers::components::{BlockInternalCommandsTable, BlockSpotlightFeeTransferAnalytics},
    icons::*,
};
use charming::{
    component::{Legend, Title},
    series::*,
    Chart, WasmRenderer,
};
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use std::collections::HashMap;

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
                                    section_heading="Internal Commands".to_string()
                                    controls=|| ().into_view()
                                >
                                    <Table data=LoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::Analytics => {
                            view! {
                                <TableSection
                                    section_heading="Analytics".to_string()
                                    controls=|| ().into_view()
                                >
                                    <span></span>
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
                                    view! { <BlockInternalCommands block=block/> }
                                }
                                (Some(block), BlockContent::Analytics) => {
                                    view! { <BlockAnalytics block=block/> }
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
            <BlockSpotlightSnarkJobTable block=block/>
        </TableSection>
    }
}

#[component]
pub fn BlockInternalCommands(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        <TableSection section_heading="Internal Commands".to_string() controls=|| ().into_view()>
            <BlockInternalCommandsTable block_state_hash=Option::from(get_state_hash(&block))/>
        </TableSection>
    }
}

#[component]
pub fn BlockAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (block_sig, _) = create_signal(block);
    let user_command_amount_total = move || {
        if let Some(user_commands) = get_user_commands(&block_sig.get()) {
            user_commands
                .iter()
                .filter_map(|transaction_option| {
                    transaction_option
                        .as_ref()
                        .map(|transaction| transaction.amount.unwrap_or(0.0))
                })
                .sum()
        } else {
            0.0
        }
    };
    let winner_total = move || get_winner_total(&block_sig.get());

    view! {
        <TableSection section_heading="Analytics".to_string() controls=|| ().into_view()>
            <AnalyticsLayout>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total User Amounts Transferred".into())
                        value=wrap_in_pill(
                            decorate_with_currency_tag(
                                nanomina_to_mina(user_command_amount_total()),
                                "mina".to_string(),
                            ),
                            ColorVariant::Green,
                        )

                        variant=ColorVariant::Green
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total Internal Fees Transferred".into())
                        value=wrap_in_pill(
                            decorate_with_currency_tag(
                                get_transaction_fees(&block_sig.get()),
                                "mina".to_string(),
                            ),
                            ColorVariant::Orange,
                        )

                        variant=ColorVariant::Orange
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total SNARK Fees".into())
                        value=wrap_in_pill(
                            decorate_with_currency_tag(
                                get_snark_fees(&block_sig.get()),
                                "mina".to_string(),
                            ),
                            ColorVariant::Blue,
                        )

                        variant=ColorVariant::Blue
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Winner Total".into())
                        value=wrap_in_pill(
                            decorate_with_currency_tag(winner_total(), "mina".to_string()),
                            ColorVariant::Grey,
                        )

                        variant=ColorVariant::Grey
                    />
                </AnalyticsSmContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightFeeTransferAnalytics block_state_hash=Option::from(
                        get_state_hash(&block_sig.get()),
                    )/>
                </AnalyticsLgContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightUserCommandAnalytics block=block_sig.get()/>
                </AnalyticsLgContainer>
            </AnalyticsLayout>
        </TableSection>
    }
}

#[component]
pub fn BlockSpotlightUserCommandAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (data, set_data) = create_signal(HashMap::new());
    create_effect(move |_| {
        let mut pie_hashmap = HashMap::new();

        if let Some(transactions) = &block.transactions {
            if let Some(user_commands) = &transactions.user_commands {
                user_commands.into_iter().for_each(|row| {
                    if let Some(r) = row.clone() {
                        match (r.amount, r.to) {
                            (Some(amount), Some(mut recipient)) => {
                                recipient.truncate(12);
                                let recipient = recipient.to_string();
                                if !pie_hashmap.contains_key(&recipient) {
                                    pie_hashmap.insert(recipient, amount as i64);
                                } else {
                                    if let Some(val) = pie_hashmap.get_mut(&recipient) {
                                        *val += amount as i64;
                                    }
                                }
                            }
                            (_, _) => (),
                        }
                    }
                    // logging::log!("{}", "iterating...");
                });

                // logging::log!("{:?}", pie_hashmap);
                set_data.set(pie_hashmap);
            }
        }
    });

    let action = create_action(move |input: &HashMap<String, i64>| {
        let input = input.clone();
        async move {
            logging::log!("input: {:?}", input);
            let mut data = input
                .iter()
                .map(|(key, val)| (*val, key))
                .collect::<Vec<_>>();

            // Sort the vector in descending order
            data.sort_by(|a, b| b.cmp(a));

            // Split the vector into two parts
            let size = data.len();
            let (top_five, rest) = data.split_at_mut(5.min(size));

            // Aggregate the remaining entries
            let binding = String::from("Other");
            let aggregated = rest.iter().fold((0, &binding), |mut acc, tup| {
                acc.0 += tup.0;
                acc
            });

            // Append the aggregated tuple to the top six
            let mut result = top_five.to_vec();
            if !rest.is_empty() {
                result.push(aggregated);
            }

            let series = Pie::new()
                .radius(vec!["50", "100"])
                .center(vec!["50%", "50%"])
                .data(result);
            let chart = Chart::new()
                .title(Title::new().text("Top Payments"))
                .legend(Legend::new().top("bottom"))
                .series(series);
            let renderer = WasmRenderer::new(375, 375);

            TimeoutFuture::new(1_000).await;
            renderer.render("chart2", &chart).unwrap();
        }
    });

    create_effect(move |_| {
        if data.get().is_empty() {
            return;
        } else {
            action.dispatch(data.get());
        }
    });

    view! { <div id="chart2" class="p-4 md:p-8"></div> }
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
            any_el: Some(wrap_in_pill(
                decorate_with_currency_tag(get_coinbase(&block), "mina".to_string()),
                ColorVariant::Green,
            )),
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
            any_el: Some(wrap_in_pill(
                decorate_with_currency_tag(get_snark_fees(&block), "mina".to_string()),
                ColorVariant::Orange,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            any_el: Some(convert_to_pill(get_global_slot(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            any_el: Some(convert_to_pill(get_slot(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            any_el: Some(convert_to_pill(get_epoch(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            any_el: Some(wrap_in_pill(
                decorate_with_currency_tag(get_transaction_fees(&block), "mina".to_string()),
                ColorVariant::Orange,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            any_el: Some(convert_to_pill(
                get_block_height(&block),
                ColorVariant::Grey,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            any_el: Some(wrap_in_pill(
                decorate_with_currency_tag(get_total_currency(&block), "mina".to_string()),
                ColorVariant::Green,
            )),
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
        <AccountDialogSubsectionTable>
            {sub_entries
                .into_iter()
                .map(|se| view! { <AccountDialogSubsectionRow label=se.label value=se.value/> })
                .collect::<Vec<_>>()}
        </AccountDialogSubsectionTable>
    }
    .into_view()
}

#[component]
pub fn BlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let resource = create_resource(
        move || (query_params_map.get(), canonical_qp.get()),
        |(value, canonical)| async move {
            let public_key = value.get("account");
            let block_hash = value.get("query");
            load_data(50, public_key.cloned(), block_hash.cloned(), canonical).await
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
                                <BooleanUrlParamSelectMenu
                                    id="canonical-selection"
                                    query_str_key="canonical"
                                    labels=BooleanUrlParamSelectOptions {
                                        true_case: String::from("Canonical"),
                                        false_case: String::from("Non-Canonical"),
                                        none_case: String::from("All"),
                                    }
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
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");
    let resource = create_resource(
        move || (query_params_map.get(), canonical_qp.get()),
        |(value, canonical)| async move {
            let state_hash = value.get("query");
            load_data(50, None, state_hash.cloned(), canonical).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <ErrorBoundary fallback=move |_| {
            view! { <NullView/> }
        }>
            {move || match resource.get().and_then(|res| res.ok()) {
                None => {
                    view! {
                        <TableSection
                            section_heading="Blocks".to_string()
                            controls=move || view! { <NullView/> }
                        >
                            <Table data=LoadingPlaceholder {}/>
                        </TableSection>
                    }
                }
                Some(data) => {
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
                                    <BooleanUrlParamSelectMenu
                                        id="canonical-selection"
                                        query_str_key="canonical"
                                        labels=BooleanUrlParamSelectOptions {
                                            true_case: String::from("Canonical"),
                                            false_case: String::from("Non-Canonical"),
                                            none_case: String::from("All"),
                                        }
                                    />
                                }
                            }
                        >

                            <Table data=SummaryPageBlocksQueryBlocks(blocks_subset) pagination=pag/>
                        </TableSection>
                    }
                }
            }}

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

#[component]
pub fn BlockSpotlightSnarkJobTable(block: BlocksQueryBlocks) -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match block.snark_jobs.clone() {
            Some(snark_jobs) => {
                view! {
                    {match snark_jobs.len() {
                        0 => {
                            view! {
                                <EmptyTable message="No SNARK work related to this block"
                                    .to_string()/>
                            }
                        }
                        _ => {
                            let pag = build_pagination(
                                snark_jobs.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                            );
                            let subset = get_subset(
                                &snark_jobs,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! { <Table data=subset pagination=pag/> }
                        }
                    }}
                }
            }
            _ => view! { <NullView/> },
        }}
    }
}
