use super::{functions::*, graphql::blocks_query::BlocksQueryBlocks, models::*};
use crate::{
    blocks::graphql::blocks_query::{BlocksQueryBlocksTransactionsFeeTransfer, ResponseData},
    common::{components::*, constants::*, functions::*, models::*, spotlight::*, table::*},
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
use leptos_use::{use_interval, UseIntervalReturn};
use std::collections::HashMap;
#[component]
pub fn BlockTabContainer(content: BlockContent) -> impl IntoView {
    let option_block = use_context::<ReadSignal<Option<BlocksQueryBlocks>>>()
        .expect("there to be an optional block signal provided");

    let content_for_fallback = content.clone();

    view! {
        <PageContainer>
            <ErrorBoundary fallback=move |_| ().into_view()>
                <Suspense fallback=move || {
                    let content_clone = content_for_fallback.clone();
                    match content_clone {
                        BlockContent::Spotlight => view! { <BlockSpotlightPlaceholder/> },
                        BlockContent::UserCommands => {
                            view! {
                                <TableSection
                                    section_heading="User Commands"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::SNARKJobs => {
                            view! {
                                <TableSection
                                    section_heading="SNARK Jobs"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::FeeTransfers => {
                            view! {
                                <TableSection
                                    section_heading="Internal Commands"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::Analytics => {
                            view! {
                                <TableSection
                                    section_heading="Analytics"
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
                                _ => ().into_view(),
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
    let (current_page, set_current_page) = create_signal(1);
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");

    view! {
        <TableSection section_heading="User Commands" controls=|| ().into_view()>

            {move || match get_user_commands(&block) {
                Some(user_commands) => {
                    let pag = build_pagination(
                        user_commands.len(),
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
                        &user_commands,
                        pag.records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <DeprecatedTable data=subset pagination=pag/> }
                }
                None => ().into_view(),
            }}

        </TableSection>
    }
}

#[component]
pub fn BlockSnarkJobs(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        <TableSection section_heading="SNARK Jobs" controls=|| ().into_view()>
            <BlockSpotlightSnarkJobTable block=block/>
        </TableSection>
    }
}

#[component]
pub fn BlockInternalCommands(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        <TableSection section_heading="Internal Commands" controls=|| ().into_view()>
            <BlockInternalCommandsTable block/>
        </TableSection>
    }
}

#[component]
pub fn BlockInternalCommandsTable(block: BlocksQueryBlocks) -> impl IntoView {
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match (
            block.transactions.clone().and_then(|txn| txn.fee_transfer),
            block.transactions.clone().and_then(|txn| txn.coinbase),
            block
                .transactions
                .clone()
                .and_then(|txn| txn.coinbase_receiver_account.and_then(|ra| ra.public_key)),
        ) {
            (Some(mut feetransfers), Some(coinbase), Some(coinbase_receiver)) => {
                feetransfers
                    .push(
                        Some(BlocksQueryBlocksTransactionsFeeTransfer {
                            fee: Some(coinbase),
                            type_: Some("Coinbase".to_string()),
                            recipient: Some(coinbase_receiver),
                        }),
                    );
                let pag = build_pagination(
                    feetransfers.len(),
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
                    &feetransfers,
                    pag.records_per_page,
                    current_page.get() - 1,
                );
                view! { <DeprecatedTable data=subset pagination=pag/> }
            }
            (_, _, _) => {
                view! { <EmptyTable message="No internal commands for this block"/> }
            }
        }}
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
                        .map(|f| f.round() as u64)
                })
                .sum()
        } else {
            0
        }
    };

    view! {
        <TableSection section_heading="Analytics" controls=|| ().into_view()>
            <AnalyticsLayout>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total User Amounts Transferred".into())
                        value=decorate_with_currency_tag(
                            nanomina_to_mina(user_command_amount_total()),
                            "mina".to_string(),
                        )

                        variant=ColorVariant::Transparent
                    />

                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total Internal Fees Transferred".into())
                        value=decorate_with_currency_tag(
                            get_transaction_fees(&block_sig.get()),
                            "mina".to_string(),
                        )

                        variant=ColorVariant::Transparent
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
                    <span></span>
                </AnalyticsSmContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightFeeTransferAnalytics block=block_sig.get()/>
                </AnalyticsLgContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightUserCommandAnalytics block=block_sig.get()/>
                </AnalyticsLgContainer>
            </AnalyticsLayout>
        </TableSection>
    }
}

#[component]
pub fn BlockSpotlightFeeTransferAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (block_sig, _) = create_signal(block);
    let (data, set_data) = create_signal(HashMap::new());

    create_effect(move |_| {
        if let Some(transactions) = block_sig.get().transactions.as_ref() {
            if let Some(fee_transfer) = transactions.fee_transfer.as_ref() {
                let pie_hashmap = fee_transfer
                    .iter()
                    .filter_map(|row| {
                        let r = row.as_ref()?;
                        let (Some(fee), Some(recipient)) = (r.fee.as_ref(), r.recipient.as_ref())
                        else {
                            return None;
                        };
                        let parsed_fee = str::parse::<i32>(fee).unwrap_or(0);
                        let sixth_to_last = recipient.len() - 6;
                        let recip = [
                            recipient[..6].to_string(),
                            recipient[sixth_to_last..].to_string(),
                        ];
                        Some((recip.join("..."), parsed_fee))
                    })
                    .fold(HashMap::new(), |mut acc, (recipient, fee)| {
                        *acc.entry(recipient).or_insert(0) += fee;
                        acc
                    });
                set_data.set(pie_hashmap);
            }
        }
    });

    create_effect(move |_| {
        if !data.get().is_empty() {
            setup_and_render_chart(&data.get(), "chart", "Top Internal Transfers");
        }
    });

    view! { <div id="chart" class="p-4 md:p-8"></div> }
}

#[component]
pub fn BlockSpotlightUserCommandAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (data, set_data) = create_signal(HashMap::new());
    create_effect(move |_| {
        if let Some(transactions) = block.transactions.as_ref() {
            if let Some(user_commands) = transactions.user_commands.as_ref() {
                let pie_hashmap = user_commands
                    .iter()
                    .filter_map(|row| {
                        let r = row.as_ref()?;
                        let (Some(amount), Some(recipient)) = (r.amount, r.to.as_ref()) else {
                            return None;
                        };
                        let sixth_to_last = recipient.len() - 6;
                        let recip = [
                            recipient[..6].to_string(),
                            recipient[sixth_to_last..].to_string(),
                        ];
                        Some((recip.join("..."), amount as i64))
                    })
                    .fold(HashMap::new(), |mut acc, (recipient, amount)| {
                        *acc.entry(recipient).or_insert(0) += amount;
                        acc
                    });
                set_data.set(pie_hashmap);
            }
        }
    });

    create_effect(move |_| {
        if !data.get().is_empty() {
            setup_and_render_chart(&data.get(), "chart2", "Top Payments");
        }
    });

    view! { <div id="chart2" class="p-4 md:p-8"></div> }
}

fn setup_and_render_chart<T>(data: &HashMap<String, T>, chart_id: &str, chart_title: &str)
where
    T: Into<i64> + Copy + 'static,
{
    let d = data.clone();
    let ch_id = chart_id.to_string();
    let ch_tl = chart_title.to_string();

    let action = create_action(move |_: &()| {
        let d_cloned = d.clone();
        let ch_id_cloned = ch_id.clone();
        let ch_tl_cloned = ch_tl.clone();

        async move { render_pie_chart(&d_cloned, &ch_id_cloned, &ch_tl_cloned).await }
    });

    action.dispatch(());
}

// Asynchronous function to render the chart
async fn render_pie_chart<T>(data: &HashMap<String, T>, chart_id: &str, chart_title: &str)
where
    T: Into<i64> + Copy,
{
    let mut sorted_data = data
        .iter()
        .map(|(key, &val)| (Into::<i64>::into(val), key))
        .collect::<Vec<_>>();
    sorted_data.sort_by(|a, b| b.0.cmp(&a.0));

    let size = sorted_data.len();
    let (top_items, rest) = sorted_data.split_at_mut(5.min(size));

    let binding = String::from("Other");
    let aggregated = rest.iter().fold((0, &binding), |mut acc, tup| {
        acc.0 += tup.0;
        acc
    });

    let mut result = top_items.to_vec();
    if !rest.is_empty() {
        result.push(aggregated);
    }

    let series = Pie::new()
        .radius(vec!["50", "100"])
        .center(vec!["50%", "50%"])
        .data(result);
    let chart = Chart::new()
        .title(Title::new().text(chart_title))
        .legend(Legend::new().top("bottom"))
        .series(series);
    let renderer = WasmRenderer::new(375, 375);

    TimeoutFuture::new(1_000).await;
    renderer.render(chart_id, &chart).unwrap();
}

#[component]
pub fn BlockSpotlight(block: BlocksQueryBlocks) -> impl IntoView {
    let state_hash = get_state_hash(&block);
    let date_time = get_date_time(&block);
    let spotlight_items = vec![
        SpotlightEntry {
            label: "State Hash".to_string(),
            any_el: Some(convert_to_link(
                state_hash.clone(),
                format!("/blocks/{}", state_hash),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            any_el: Some({
                let prev_state_hash = get_previous_state_hash(&block);
                convert_to_link(
                    prev_state_hash.clone(),
                    format!("/blocks/{}", prev_state_hash),
                )
            }),
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
            any_el: Some(decorate_with_currency_tag(
                get_coinbase(&block),
                "mina".to_string(),
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            any_el: Some({
                let coinbase_receiver = get_coinbase_receiver(&block);
                convert_to_link(
                    coinbase_receiver.clone(),
                    format!("/addresses/accounts/{}", coinbase_receiver),
                )
            }),
            copiable: true,
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            any_el: Some(decorate_with_currency_tag(
                get_snark_fees(&block),
                "mina".to_string(),
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
            any_el: Some(decorate_with_currency_tag(
                get_transaction_fees(&block),
                "mina".to_string(),
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
            any_el: Some(decorate_with_currency_tag(
                get_total_currency(&block),
                "mina".to_string(),
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
            header="Block Spotlight"
            spotlight_items=spotlight_items
            id=None
            meta=None
        >
            <BlockIcon width=40/>
        </SpotlightSection>
    }
}

#[component]
pub fn BlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<i64>("q-height");
    let (slot_sig, _) = create_query_signal::<i64>("q-slot");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                block_height_sig.get(),
                slot_sig.get(),
                canonical_sig.get(),
            )
        },
        |(_, q_map, block_height, slot, canonical)| async move {
            load_data(
                TABLE_RECORD_SIZE,
                q_map.get("q-block-producer").cloned(),
                q_map.get("q-state-hash").cloned(),
                block_height,
                slot,
                if canonical.is_some() {
                    canonical
                } else {
                    Some(true)
                },
            )
            .await
        },
    );

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Slot".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Block Producer".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Coinbase".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "User Commands".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "SNARKs".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let build_blocks_pag = |data: &ResponseData,
                            current_page: ReadSignal<usize>,
                            set_current_page: WriteSignal<usize>,
                            page_dim: ReadSignal<PageDimensions>| {
        build_pagination(
            data.blocks.len(),
            TABLE_DEFAULT_PAGE_SIZE,
            current_page.get(),
            set_current_page,
            page_dim.get().height.map(|f| f as usize),
            Some(Box::new(|container_height: usize| {
                (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                    / ESTIMATED_ROW_HEIGHT
            })),
        )
    };

    view! {
        <TableSection
            section_heading="Blocks"
            controls=move || {
                view! {
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        >

            <TableContainer>
                <Table>
                    <TableHeader columns=table_columns/>
                    <Suspense fallback=move || {
                        view! {
                            <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                        }
                    }>
                        {move || {
                            resource
                                .get()
                                .and_then(|res| res.ok())
                                .map(|data| {
                                    let pag = build_blocks_pag(
                                        &data,
                                        current_page,
                                        set_current_page,
                                        page_dim,
                                    );
                                    let blocks_subset = get_subset(
                                        &data.blocks,
                                        pag.records_per_page,
                                        current_page.get() - 1,
                                    );
                                    view! { <TableRows data=blocks_subset/> }
                                })
                        }}

                    </Suspense>
                </Table>
                <Suspense fallback=|| {
                    ().into_view()
                }>
                    {move || {
                        resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|data| {
                                let pag = build_blocks_pag(
                                    &data,
                                    current_page,
                                    set_current_page,
                                    page_dim,
                                );
                                view! { <Pagination pagination=pag/> }
                            })
                            .collect_view()
                    }}

                </Suspense>
            </TableContainer>
        </TableSection>
        <Outlet/>
    }
}

const ESTIMATED_NON_TABLE_SPACE_IN_SUMMARY: usize = 290;

#[component]
pub fn SummaryPageBlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();
    let (block_height_sig, _) = create_query_signal::<i64>("q-height");
    let (slot_sig, _) = create_query_signal::<i64>("q-slot");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                block_height_sig.get(),
                slot_sig.get(),
                canonical_sig.get(),
            )
        },
        |(_, q_map, block_height, slot, canonical)| async move {
            load_data(
                TABLE_RECORD_SIZE,
                q_map.get("q-block-producer").cloned(),
                q_map.get("q-state-hash").cloned(),
                block_height,
                slot,
                if canonical.is_some() {
                    canonical
                } else {
                    Some(true)
                },
            )
            .await
        },
    );

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Slot".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Block Producer".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Coinbase".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "User Commands".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "SNARKs".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let get_data_and_pagination = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.blocks.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|f| f as usize),
                Some(Box::new(|container_height: usize| {
                    (container_height - ESTIMATED_NON_TABLE_SPACE_IN_SUMMARY) / ESTIMATED_ROW_HEIGHT
                })),
            );
            (data, pag)
        })
    };

    view! {
        <ErrorBoundary fallback=move |_| { ().into_view() }>

            <TableSection
                section_heading="Blocks"
                controls=move || {
                    view! {
                        <UrlParamSelectMenu
                            id="canonical-selection"
                            query_str_key="canonical"
                            labels=UrlParamSelectOptions {
                                is_boolean_option: true,
                                cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                            }
                        />
                    }
                }
            >

                <TableContainer>
                    <Table>
                        <TableHeader columns=table_columns/>
                        <Suspense fallback=move || {
                            view! {
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    10
                                ]/>
                            }
                        }>
                            {move || {
                                get_data_and_pagination()
                                    .map(|(data, pag)| {
                                        view! {
                                            <TableRows data=data
                                                .blocks[pag.start_index() - 1..pag.end_index()]
                                                .to_vec()/>
                                        }
                                    })
                            }}

                        </Suspense>
                    </Table>
                    <Suspense fallback=|| {
                        ().into_view()
                    }>
                        {move || {
                            get_data_and_pagination()
                                .map(|(_, pag)| {
                                    view! { <Pagination pagination=pag/> }
                                })
                        }}

                    </Suspense>
                </TableContainer>
            </TableSection>
        </ErrorBoundary>
        <Outlet/>
    }
}

#[component]
pub fn BlockSpotlightSnarkJobTable(block: BlocksQueryBlocks) -> impl IntoView {
    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match block.snark_jobs.clone() {
            Some(snark_jobs) => {
                view! {
                    {match snark_jobs.len() {
                        0 => {
                            view! { <EmptyTable message="No SNARK work related to this block"/> }
                        }
                        _ => {
                            let pag = build_pagination(
                                snark_jobs.len(),
                                TABLE_DEFAULT_PAGE_SIZE,
                                current_page.get(),
                                set_current_page,
                                page_dim.get().height.map(|h| h as usize),
                                Some(
                                    Box::new(|container_height: usize| {
                                        (container_height
                                            - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                                            / ESTIMATED_ROW_HEIGHT
                                    }),
                                ),
                            );
                            let subset = get_subset(
                                &snark_jobs,
                                pag.records_per_page,
                                current_page.get() - 1,
                            );
                            view! { <DeprecatedTable data=subset pagination=pag/> }
                        }
                    }}
                }
            }
            _ => ().into_view(),
        }}
    }
}
