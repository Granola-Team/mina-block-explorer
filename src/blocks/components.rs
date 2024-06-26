use super::{functions::*, graphql::blocks_query::BlocksQueryBlocks, models::*};
use crate::{
    blocks::graphql::{blocks_query, blocks_query::BlocksQueryBlocksTransactionsFeeTransfer},
    common::{components::*, constants::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
    summary::models::BlockchainSummary,
};
use charming::{
    component::{Legend, Title},
    series::*,
    Chart, WasmRenderer,
};
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use leptos_use::{
    storage::use_local_storage, use_document_visibility, use_interval, utils::JsonCodec,
    UseIntervalReturn,
};
use std::collections::HashMap;
use web_sys::VisibilityState;

#[component]
pub fn UniqueBlocksProducersSummaryItem() -> impl IntoView {
    let (blocks_sig, _, _) =
        use_local_storage::<blocks_query::ResponseData, JsonCodec>(BLOCKS_STORAGE_KEY);
    let (unique_producers_sig, _set_up) = create_signal(Some("...".to_string()));

    move || {
        view! {
            <SummaryItem
                label=format!("Unique Producers of last {} Blocks", blocks_sig.get().blocks.len())
                value=unique_producers_sig.get()
                id="uniqueBlockProducers"
            />
        }
    }
}

#[component]
pub fn BlockTabContainer(content: BlockContent) -> impl IntoView {
    let option_block = use_context::<ReadSignal<Option<BlocksQueryBlocks>>>()
        .expect("there to be an optional block signal provided");
    view! {
        <PageContainer>
            {move || match (option_block.get(), content.clone()) {
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
            }}

        </PageContainer>
    }
}

#[component]
pub fn BlockUserCommands(block: BlocksQueryBlocks) -> impl IntoView {
    let (data_sig, _) = create_signal(get_user_commands(&block));
    let (loading_sig, _) = create_signal(false);

    let table_columns = vec![
        TableColumn {
            column: "Hash".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Status".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "From".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "To".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Amount".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            is_loading=loading_sig.into()
            section_heading="User Commands"
            controls=|| ().into_view()
        />
    }
}

#[component]
pub fn BlockSnarkJobs(block: BlocksQueryBlocks) -> impl IntoView {
    let (data_sig, _) = create_signal(block.snark_jobs);
    let (loading_sig, _) = create_signal(false);

    let table_columns = vec![
        TableColumn {
            column: "State Hash".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Age".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Prover".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            is_loading=loading_sig.into()
            section_heading="SNARK Jobs"
            controls=|| ().into_view()
        />
    }
}

#[component]
pub fn BlockInternalCommands(block: BlocksQueryBlocks) -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    let (loading_sig, _) = create_signal(false);

    create_effect(move |_| {
        let (fee_transfers, coinbase, coinbase_receiver) = {
            let transactions = &block.transactions;

            (
                transactions
                    .as_ref()
                    .and_then(|txn| txn.fee_transfer.clone()),
                transactions.as_ref().and_then(|txn| txn.coinbase.clone()),
                transactions.as_ref().and_then(|txn| {
                    txn.coinbase_receiver_account
                        .as_ref()
                        .and_then(|ra| ra.public_key.clone())
                }),
            )
        };

        if let Some(mut fee_transfers) = fee_transfers {
            fee_transfers.push(Some(BlocksQueryBlocksTransactionsFeeTransfer {
                fee: coinbase,
                type_: Some("Coinbase".to_string()),
                recipient: coinbase_receiver,
            }));

            set_data.set(Some(fee_transfers.clone()));
        } else {
            let new_fee_transfers = vec![Some(BlocksQueryBlocksTransactionsFeeTransfer {
                fee: coinbase,
                type_: Some("Coinbase".to_string()),
                recipient: coinbase_receiver,
            })];
            set_data.set(Some(new_fee_transfers));
        }
    });

    let table_columns = vec![
        TableColumn {
            column: "Recipient".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            is_loading=loading_sig.into()
            section_heading="Internal Commands"
            controls=|| ().into_view()
        />
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
    let (metadata, _) = create_signal(Some(TableMetadata {
        displayed_records: user_command_amount_total(),
        available_records: None,
        total_records: None,
    }));

    view! {
        <TableSection
            metadata=metadata.into()
            section_heading="Analytics"
            controls=|| ().into_view()
        >
            <AnalyticsLayout>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total User Amounts Transferred".into())
                        value=decorate_with_mina_tag(nanomina_to_mina(user_command_amount_total()))

                        variant=ColorVariant::Transparent
                    />

                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total Internal Fees Transferred".into())
                        value=decorate_with_mina_tag(get_transaction_fees(&block_sig.get()))

                        variant=ColorVariant::Transparent
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total SNARK Fees".into())
                        value=wrap_in_pill(
                            decorate_with_mina_tag(get_snark_fees(&block_sig.get())),
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
            any_el: Some(decorate_with_mina_tag(get_coinbase(&block))),
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
            any_el: Some(decorate_with_mina_tag(get_snark_fees(&block))),
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
            any_el: Some(decorate_with_mina_tag(get_transaction_fees(&block))),
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
            label: "Total MINA".to_string(),
            any_el: Some(decorate_with_mina_tag(get_total_currency(&block))),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Canonical".to_string(),
            any_el: Some(wrap_in_pill(
                convert_to_span(get_canonical(&block).unwrap_or_default().to_string()),
                ColorVariant::Grey,
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
            label: "Total MINA".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Canonical".to_string(),
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
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let visibility = use_document_visibility();
    let query_params_map = use_query_map();
    let (data_sig, set_data_sig) = create_signal(None);
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
        move |(_, q_map, block_height, slot, canonical)| async move {
            if visibility.get() == VisibilityState::Visible {
                load_data(
                    TABLE_ROW_LIMIT,
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
            } else {
                logging::log!("Document not visible. Data polling skipped for blocks query.");
                Ok(blocks_query::ResponseData {
                    blocks: data_sig.get().unwrap_or_default(),
                })
            }
        },
    );

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            html_input_type: "number".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Slot".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Age".to_string(),
            width: Some(String::from("125px")),
            ..Default::default()
        },
        TableColumn {
            column: "Block Producer".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Coinbase".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "User Commands".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "SNARKs".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
    ];

    create_effect(move |_| {
        if let Some(data) = resource.get().and_then(|res| res.ok()) {
            set_data_sig.set(Some(data.blocks));
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            section_heading="Blocks"
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    displayed_records: u64::try_from(
                            data_sig.get().map(|d| d.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: canonical_sig
                        .get()
                        .map(|c| {
                            if c {
                                summary_sig.get().blockchain_length
                            } else {
                                summary_sig.get().total_num_blocks
                                    - summary_sig.get().blockchain_length
                            }
                        })
                        .or(Some(summary_sig.get().blockchain_length)),
                    total_records: Some(summary_sig.get().total_num_blocks),
                })
            })

            is_loading=resource.loading()
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
        />

        <Outlet/>
    }
}
