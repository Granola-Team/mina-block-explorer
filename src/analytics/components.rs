use super::{functions::*, models::*};
use crate::common::{
    components::*,
    table::{ColumnTextAlignment, TableColumn, TableSectionTemplate, TableSortDirection},
};
use leptos::*;
use leptos_router::create_query_signal;
use std::collections::HashMap;

#[component]
pub fn AnalayticsFilters(
    #[prop(optional)] epoch: bool,
    #[prop(optional, default = true)] block_limit: bool,
) -> impl IntoView {
    let (limit_sig, set_limit) = create_query_signal::<u64>("limit");
    let (epoch_sig, set_epoch) = create_query_signal::<u64>("epoch");

    view! {
        <div class="w-full flex justify-start items-center p-2 pl-8 md:p-8 md:py-2">
            {move || match block_limit {
                true => {
                    if limit_sig.get_untracked().is_none() {
                        set_limit.set(Some(1000u64));
                    }
                    view! {
                        <div class="flex justify-start items-baseline mr-2 md:mr-4">
                            <label for="block-limit" class="mr-2">
                                "Block limit:"
                            </label>
                            <ControlledInput
                                id="block-limit"
                                input_type="number"
                                name="block-limit"
                                disabled_sig=Signal::from(|| false)
                                value=limit_sig.get().map(|s| s.to_string()).unwrap_or_default()
                                setter_sig=SignalSetter::map(move |opt_str: Option<String>| {
                                    set_limit
                                        .set(
                                            opt_str
                                                .map(|v_str| v_str.parse::<u64>().ok().unwrap_or_default()),
                                        )
                                })

                                number_props=HashMap::from([
                                    ("step".to_string(), "1000".to_string()),
                                    ("min".to_string(), "1000".to_string()),
                                    ("max".to_string(), "10000".to_string()),
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
                    if epoch_sig.get_untracked().is_none() {
                        set_epoch.set(Some(0u64));
                    }
                    view! {
                        <div class="flex justify-start items-baseline mr-2 md:mr-4">
                            <label for="block-limit" class="mr-2">
                                "Epoch:"
                            </label>
                            <ControlledInput
                                id="epoch"
                                input_type="number"
                                name="epoch"
                                disabled_sig=Signal::from(|| false)
                                value=epoch_sig.get().map(|s| s.to_string()).unwrap_or_default()
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
            let table_columns = vec![
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
                    sort_direction: Some(TableSortDirection::Desc),
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
                                <label for="block-limit" class="mr-2">
                                    "Epoch:"
                                </label>
                                <ControlledInput
                                    id="epoch"
                                    input_type="number"
                                    name="epoch"
                                    disabled_sig=Signal::from(|| false)
                                    value=epoch_sig
                                        .get()
                                        .map(|s| s.to_string())
                                        .unwrap_or("0".to_string())
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
            let table_columns = vec![
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
                    sort_direction: Some(TableSortDirection::Desc),
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
                                <label for="block-limit" class="mr-2">
                                    "Epoch:"
                                </label>
                                <ControlledInput
                                    id="epoch"
                                    input_type="number"
                                    name="epoch"
                                    disabled_sig=Signal::from(|| false)
                                    value=epoch_sig
                                        .get()
                                        .map(|s| s.to_string())
                                        .unwrap_or("0".to_string())
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
    let (limit_sig, _) = create_query_signal::<u64>("limit");
    let resource = create_resource(
        move || limit_sig.get(),
        move |limit| async move { load_snark_fees(limit).await },
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
            let table_columns = vec![
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
