use super::{functions::*, models::*};
use crate::{
    common::{components::*, constants::*, functions::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::{create_query_signal, use_location, use_navigate, ParamsMap};
use leptos_use::storage::use_local_storage;
use std::collections::HashMap;

const INPUT_STYLES: &str =
    "mr-4 h-8 pl-4 text-sm box-border border-[1px] border-slate-300 rounded-md";

#[component]
pub fn CacheBustScript(
    #[prop(into)] src: String,
    #[prop(optional, default = false)] defer: bool,
) -> impl IntoView {
    view! {
        <script src=format!("{}?v={}", src, get_unix_timestamp().to_string()) defer=defer></script>
    }
}

#[component]
pub fn AnalyticsFilters(#[prop(optional, default = false)] by_block: bool) -> impl IntoView {
    let input_blockheight_gte: NodeRef<html::Input> = create_node_ref();
    let input_blockheight_lte: NodeRef<html::Input> = create_node_ref();
    let (blockheight_gte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_GTE);
    let (blockheight_lte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_LTE);
    let (validation_message_sig, set_validation_message) = create_signal::<Option<&str>>(None);
    let navigate = use_navigate();
    let nav_clone = navigate.clone();
    let location = use_location();

    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let apply = move |_| {
        by_block.then(|| {
            let blockheight_gte_opt = Some(
                input_blockheight_gte
                    .get()
                    .expect("<input/> should be mounted")
                    .value(),
            )
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<u64>().ok());
            let blockheight_lte_opt = Some(
                input_blockheight_lte
                    .get()
                    .expect("<input/> should be mounted")
                    .value(),
            )
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<u64>().ok());
            match validate_block_height_range(blockheight_gte_opt, blockheight_lte_opt) {
                Err(err) => set_validation_message.set(Some(err)),
                Ok(_) => {
                    set_validation_message.set(None);
                    let mut q_params = ParamsMap::new();
                    if let Some(blockheight_gte) = blockheight_gte_opt {
                        q_params.insert(
                            QUERY_PARAM_BLOCKHEIGHT_GTE.to_string(),
                            blockheight_gte.to_string(),
                        );
                    }
                    if let Some(blockheight_lte) = blockheight_lte_opt {
                        q_params.insert(
                            QUERY_PARAM_BLOCKHEIGHT_LTE.to_string(),
                            blockheight_lte.to_string(),
                        );
                    }
                    nav_clone(
                        &format!("{}{}", location.pathname.get(), q_params.to_query_string()),
                        Default::default(),
                    )
                }
            }
        });
    };

    let blockchain_length_opt = Some(summary_sig.get().blockchain_length).filter(|&n| n != 0);

    view! {
        <div class="w-full flex justify-start items-center p-2 md:p-8 md:py-2">
            <div class="w-full md:w-fit grid grid-cols-2 gap-4 md:flex md:flex-row md:justify-start md:items-baseline md:mr-4">
                {by_block
                    .then(|| {
                        view! {
                            <label
                                for="blockheight-gte"
                                class="font-semibold whitespace-nowrap mr-2"
                            >
                                "Start Block Height: "
                            </label>
                            <input
                                id="blockheight-gte"
                                type="number"
                                name="blockheight-gte"
                                on:input=move |_| {
                                    set_validation_message.set(None);
                                }
                                class=INPUT_STYLES
                                min=0
                                step=50
                                max=summary_sig.get().blockchain_length.to_string()
                                value=blockheight_gte_sig
                                    .get()
                                    .or(blockchain_length_opt.map(|l| l - 1000))
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                                node_ref=input_blockheight_gte
                            />
                            <label
                                for="blockheight-lte"
                                class="font-semibold whitespace-nowrap mr-2"
                            >
                                "End Block Height: "
                            </label>
                            <input
                                id="blockheight-lte"
                                type="number"
                                name="blockheight-lte"
                                on:input=move |_| {
                                    set_validation_message.set(None);
                                }
                                class=INPUT_STYLES
                                min=0
                                step=50
                                max=summary_sig.get().blockchain_length.to_string()
                                value=blockheight_lte_sig
                                    .get()
                                    .or(blockchain_length_opt)
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                                node_ref=input_blockheight_lte
                            />
                        }
                    })} <Button text="Apply" on_click=apply class_str="col-span-2" />
                {move || {
                    validation_message_sig
                        .get()
                        .map(|message| {
                            view! {
                                <div id="input-validation" class="col-span-2 text-red-600">
                                    {message}
                                </div>
                            }
                        })
                }}
            </div>
        </div>
    }
}

#[component]
pub fn SnarkerLeaderboard() -> impl IntoView {
    let (epoch_sig, set_epoch) = create_query_signal::<u32>("epoch");
    let (sort_dir_sig, _) = create_query_signal::<String>("sort-dir");
    if epoch_sig.get_untracked().is_none() {
        set_epoch.set(Some(0u32));
    }
    let resource = create_resource(
        move || (epoch_sig.get(), sort_dir_sig.get()),
        move |(epoch, sort_dir)| async move {
            let highest_fee_sort = sort_dir
                .clone()
                .and_then(|s| SnarkerLeaderboardHighestFees::try_from(s).ok());
            let total_fees_sort =
                sort_dir.and_then(|s| SnarkerLeaderboardTotalFees::try_from(s).ok());
            load_snarker_leaderboard_data(epoch, total_fees_sort, highest_fee_sort).await
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
            let mut highest_fee_sort = SnarkerLeaderboardHighestFees::Nil;
            if let Some(hf_sort) = sort_dir_sig
                .get()
                .and_then(|s| SnarkerLeaderboardHighestFees::try_from(s).ok())
            {
                highest_fee_sort = hf_sort;
            }
            let mut total_fees_sort = SnarkerLeaderboardTotalFees::Nil;
            if let Some(tf_sort) = sort_dir_sig
                .get()
                .and_then(|s| SnarkerLeaderboardTotalFees::try_from(s).ok())
            {
                total_fees_sort = tf_sort;
            }
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
                    sort_direction: Some(AnySort::SnarkerLeaderboardTotalFees(total_fees_sort)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Min Fee".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
                },
                TableColumn {
                    column: "Max Fee".to_string(),
                    sort_direction: Some(AnySort::SnarkerLeaderboardHighestFee(highest_fee_sort)),
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
                    section_heading=(String::from("SNARKER Leaderboard"), ().into_view())
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
                    sort_direction: Some(AnySort::StakerLeaderboardCanonicalBlocks(
                        StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc,
                    )),
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
                    section_heading=(String::from("Staker Leaderboard"), ().into_view())
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
    let (blockheight_lte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_LTE);
    let (blockheight_gte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_GTE);
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
                    section_heading=(String::from("SNARK Fees Overview"), ().into_view())
                    controls=|| ().into_view()
                />
            }
        }
    }
}
