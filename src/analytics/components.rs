use super::{functions::*, models::*};
use crate::{
    common::{components::*, constants::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::create_query_signal;
use leptos_use::storage::use_local_storage;
use std::collections::HashMap;

const DEFAULT_BLOCK_RANGE: u64 = 3000;
const INPUT_STYLES: &str =
    "mr-4 h-8 pl-4 text-sm box-border border-[1px] border-slate-300 rounded-md";

#[component]
pub fn AnalyticsFilters(
    #[prop(optional)] epoch: bool,
    #[prop(optional, default = false)] by_block: bool,
) -> impl IntoView {
    let (blockheight_lte, set_blockheight_lte) = create_query_signal::<u64>("q-blockheight-lte");
    let (blockheight_gte, set_blockheight_gte) = create_query_signal::<u64>("q-blockheight-gte");
    let (epoch_sig, set_epoch) = create_query_signal::<u64>("epoch");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    create_effect(move |_| {
        if by_block && blockheight_lte.get().is_none() {
            set_blockheight_lte.set(Some(summary_sig.get().blockchain_length));
        }
        if by_block && blockheight_gte.get().is_none() {
            set_blockheight_gte.set(Some(
                summary_sig
                    .get()
                    .blockchain_length
                    .saturating_sub(DEFAULT_BLOCK_RANGE),
            ))
        }
    });

    create_effect(move |_| {
        if epoch_sig.get_untracked().is_none() {
            set_epoch.set(Some(summary_sig.get().epoch));
        }
    });

    view! {
        <div class="w-full flex justify-start items-center p-2 pl-8 md:p-8 md:py-2">
            {move || match by_block {
                true => {
                    view! {
                        <div class="flex justify-start items-baseline mr-2 md:mr-4">
                            <label for="blockheight-gte font-semibold" class="mr-2">
                                "Start Block Height: "
                            </label>
                            <ControlledInput
                                id="blockheight-gte"
                                input_type="number"
                                name="blockheight-gte"
                                disabled_sig=Signal::from(|| false)
                                value_sig=blockheight_gte
                                setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                    set_blockheight_gte
                                        .set(
                                            opt_str
                                                .map(|v_str| v_str.parse::<u64>().ok().unwrap_or_default()),
                                        )
                                })
                                input_class=INPUT_STYLES.to_string()
                                number_props=HashMap::from([
                                    ("min".to_string(), "0".to_string()),
                                    ("step".to_string(), "1000".to_string()),
                                    (
                                        "max".to_string(),
                                        summary_sig.get().blockchain_length.to_string(),
                                    ),
                                ])
                            />
                            <label for="blockheight-lte font-semibold" class="mr-2">
                                "End Block Height: "
                            </label>
                            <ControlledInput
                                id="blockheight-lte"
                                input_type="number"
                                name="blockheight-lte"
                                disabled_sig=Signal::from(|| false)
                                value_sig=blockheight_lte
                                setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                    set_blockheight_lte
                                        .set(
                                            opt_str
                                                .map(|v_str| v_str.parse::<u64>().ok().unwrap_or_default()),
                                        )
                                })
                                input_class=INPUT_STYLES.to_string()
                                number_props=HashMap::from([
                                    (
                                        "min".to_string(),
                                        blockheight_gte.get().unwrap_or(0).to_string(),
                                    ),
                                    ("step".to_string(), "1000".to_string()),
                                    (
                                        "max".to_string(),
                                        summary_sig.get().blockchain_length.to_string(),
                                    ),
                                ])
                            />

                        </div>
                    }
                        .into_view()
                }
                false => ().into_view(),
            }}
            {move || match epoch {
                true => {
                    view! {
                        <div class="flex justify-start items-baseline mr-2 md:mr-4">
                            <label for="blockheight-lte" class="mr-2">
                                "Epoch:"
                            </label>
                            <ControlledInput
                                id="epoch"
                                input_type="number"
                                name="epoch"
                                disabled_sig=Signal::from(|| false)
                                value_sig=epoch_sig
                                setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                    set_epoch
                                        .set(
                                            opt_str
                                                .map(|v_str| v_str.parse::<u64>().ok().unwrap_or_default()),
                                        )
                                })

                                number_props=HashMap::from([
                                    ("step".to_string(), "1".to_string()),
                                    ("min".to_string(), "0".to_string()),
                                    ("max".to_string(), "1000".to_string()),
                                ])
                            />

                        </div>
                    }
                        .into_view()
                }
                false => ().into_view(),
            }}

        </div>
    }
}

#[component]
pub fn SnarkerLeaderboard() -> impl IntoView {
    let (epoch_sig, set_epoch) = create_query_signal::<u32>("epoch");
    if epoch_sig.get_untracked().is_none() {
        set_epoch.set(Some(0u32));
    }
    let resource = create_resource(
        move || epoch_sig.get(),
        move |epoch| async move {
            load_snarker_leaderboard_data(epoch, SnarkerLeaderboardSort::HighestFeeDesc).await
        },
    );
    let (data_sig, set_data) = create_signal(None);

    create_effect(move |_| {
        set_data.set(
            resource
                .get()
                .and_then(|res| res.ok())
                .map(|res| res.data.top_snarkers),
        );
    });

    {
        move || {
            let table_columns: Vec<TableColumn<AnySort>> = vec![
                TableColumn {
                    column: "Username".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "Public Key".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "Total Fees".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Min Fee".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Max Fee".to_string(),
                    // TODO: implement sort_direction generic
                    // sort_direction: Some(TableSortDirection::Desc),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Snarks Sold".to_string(),
                    ..Default::default()
                },
            ];
            view! {
                <TableSectionTemplate
                    table_columns
                    data_sig
                    is_loading=resource.loading()
                    section_heading="Snarker Leaderboard"
                    controls=move || {
                        epoch_sig.get();
                        view! {
                            <div class="flex justify-start items-baseline mr-2 md:mr-4">
                                <label for="blockheight-lte" class="mr-2">
                                    "Epoch:"
                                </label>
                                <ControlledInput
                                    id="epoch"
                                    input_type="number"
                                    name="epoch"
                                    disabled_sig=Signal::from(|| false)
                                    value_sig=epoch_sig
                                    setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                        set_epoch
                                            .set(
                                                opt_str
                                                    .map(|v_str| v_str.parse::<u32>().ok().unwrap_or_default()),
                                            )
                                    })

                                    number_props=HashMap::from([
                                        ("step".to_string(), "1".to_string()),
                                        ("min".to_string(), "0".to_string()),
                                        ("max".to_string(), "1000".to_string()),
                                    ])
                                />
                            </div>
                        }
                    }
                />
            }
        }
    }
}

#[component]
pub fn StakerLeaderboard() -> impl IntoView {
    let (epoch_sig, set_epoch) = create_query_signal::<u32>("epoch");
    if epoch_sig.get_untracked().is_none() {
        set_epoch.set(Some(0u32));
    }
    let resource = create_resource(
        move || epoch_sig.get(),
        move |epoch| async move {
            load_staker_leaderboard_data(
                epoch,
                StakerLeaderboardSort::NumCanonicalBlocksProducedDesc,
            )
            .await
        },
    );
    let (data_sig, set_data) = create_signal(None);

    create_effect(move |_| {
        set_data.set(
            resource
                .get()
                .and_then(|res| res.ok())
                .map(|res| res.data.top_stakers),
        );
    });

    {
        move || {
            let table_columns: Vec<TableColumn<AnySort>> = vec![
                TableColumn {
                    column: "Username".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "Public Key".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "Canonical Blocks Produced".to_string(),
                    // TODO: implement sort_direction generic
                    // sort_direction: Some(TableSortDirection::Desc),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Supercharged Blocks Produced".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Slots Produced".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Orphan Rate".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
            ];
            view! {
                <TableSectionTemplate
                    table_columns
                    data_sig
                    is_loading=resource.loading()
                    section_heading="Staker Leaderboard"
                    controls=move || {
                        epoch_sig.get();
                        view! {
                            <div class="flex justify-start items-baseline mr-2 md:mr-4">
                                <label for="blockheight-lte" class="mr-2">
                                    "Epoch:"
                                </label>
                                <ControlledInput
                                    id="epoch"
                                    input_type="number"
                                    name="epoch"
                                    disabled_sig=Signal::from(|| false)
                                    value_sig=epoch_sig
                                    setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                        set_epoch
                                            .set(
                                                opt_str
                                                    .map(|v_str| v_str.parse::<u32>().ok().unwrap_or_default()),
                                            )
                                    })

                                    number_props=HashMap::from([
                                        ("step".to_string(), "1".to_string()),
                                        ("min".to_string(), "0".to_string()),
                                        ("max".to_string(), "1000".to_string()),
                                    ])
                                />
                            </div>
                        }
                    }
                />
            }
        }
    }
}

#[component]
pub fn SnarkFees() -> impl IntoView {
    let (blockheight_lte_sig, _) = create_query_signal::<u64>("q-blockheight-lte");
    let (blockheight_gte_sig, _) = create_query_signal::<u64>("q-blockheight-gte");
    let resource = create_resource(
        move || (blockheight_lte_sig.get(), blockheight_gte_sig.get()),
        move |(blockheight_lte, blockheight_gte)| async move {
            load_snark_fees(blockheight_lte, blockheight_gte).await
        },
    );
    let (data_sig, set_data) = create_signal(None);

    create_effect(move |_| {
        set_data.set(Some(
            resource
                .get()
                .and_then(|res| res.ok())
                .map(|data| SnarkStatsContainer::from(data.data.blocks)),
        ));
    });

    {
        move || {
            let table_columns: Vec<TableColumn<AnySort>> = vec![
                TableColumn {
                    column: "Metric".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "All SNARKs".to_string(),
                    ..Default::default()
                },
                TableColumn {
                    column: "SNARKs with non-zero fees".to_string(),
                    ..Default::default()
                },
            ];
            view! {
                <TableSectionTemplate
                    table_columns
                    data_sig
                    is_loading=resource.loading()
                    section_heading="SNARK Fees Overview"
                    controls=|| ().into_view()
                />
            }
        }
    }
}
