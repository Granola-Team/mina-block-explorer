use super::{functions::*, models::*};
use crate::{
    common::{components::*, constants::*, functions::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::{ParamsMap, create_query_signal, use_location, use_navigate};
use leptos_use::{UseTimeoutFnReturn, storage::use_local_storage, use_timeout_fn};
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
                                data-test="start-block-height-input"
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
                                data-test="end-block-height-input"
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
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let (sort_dir_sig, _) = create_query_signal::<String>("sort-dir");
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

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        |(epoch, e, set_epoch): (Option<u32>, u64, SignalSetter<Option<u32>>)| {
            if epoch.is_none() {
                set_epoch.clone().set(Some(e as u32));
            }
        },
        1000.0,
    );

    create_effect(move |_| {
        start((
            epoch_sig.get_untracked(),
            summary_sig
                .get()
                .chain
                .as_ref()
                .and_then(|c| c.get(BERKELEY_CHAIN_ID))
                .map(|c| c.latest_epoch)
                .unwrap_or_default(),
            set_epoch,
        ));
    });

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
    let (sort_dir_sig, _) = create_query_signal::<String>("sort-dir");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let resource = create_resource(
        move || (epoch_sig.get(), sort_dir_sig.get()),
        move |(epoch, sort_dir)| async move {
            load_staker_leaderboard_data(
                epoch,
                sort_dir
                    .and_then(|dir| StakerLeaderboardCanonicalBlocks::try_from(dir).ok())
                    .unwrap_or(
                        StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc,
                    ),
            )
            .await
        },
    );
    let (data_sig, set_data) = create_signal(None);

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        |(epoch, e, set_epoch): (Option<u32>, u64, SignalSetter<Option<u32>>)| {
            if epoch.is_none() {
                set_epoch.clone().set(Some(e as u32));
            }
        },
        1000.0,
    );

    create_effect(move |_| {
        start((
            epoch_sig.get_untracked(),
            summary_sig
                .get()
                .chain
                .as_ref()
                .and_then(|c| c.get(BERKELEY_CHAIN_ID))
                .map(|c| c.latest_epoch)
                .unwrap_or_default(),
            set_epoch,
        ));
    });

    create_effect(move |_| {
        let top_stakers_opt = resource
            .get()
            .and_then(|res| res.ok())
            .map(|res| res.data.top_stakers);

        let updated_stakers = top_stakers_opt.map(|top_stakers| {
            top_stakers
                .into_iter()
                .map(|ss| StakerStats {
                    username: ss.username,
                    public_key: ss.public_key,
                    num_blocks_produced: ss.num_blocks_produced,
                    num_canonical_blocks_produced: ss.num_canonical_blocks_produced,
                    num_supercharged_blocks_produced: ss.num_supercharged_blocks_produced,
                    num_slots_produced: ss.num_slots_produced,
                    delegation_totals: ss.delegation_totals,
                    epoch_num_canonical_blocks: resource
                        .get()
                        .and_then(|res| res.ok())
                        .and_then(|res| res.data.blocks.first().cloned())
                        .map(|block| block.epoch_num_canonical_blocks),
                    epoch_num_blocks: resource
                        .get()
                        .and_then(|res| res.ok())
                        .and_then(|res| res.data.blocks.first().cloned())
                        .map(|block| block.epoch_num_blocks),
                })
                .collect::<Vec<StakerStats>>()
        });

        set_data.set(updated_stakers);
    });

    {
        move || {
            let table_columns: Vec<TableColumn<AnySort>> = vec![
                TableColumn {
                    column: "Delegate".to_string(),
                    width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Canonical Blocks Produced".to_string(),
                    sort_direction: Some(AnySort::StakerLeaderboardCanonicalBlocks(sort_dir_sig
                        .get()
                        .and_then(|dir| StakerLeaderboardCanonicalBlocks::try_from(dir).ok())
                        .unwrap_or(StakerLeaderboardCanonicalBlocks::NumberOfCanonicalBlocksProducedDesc))),
                    alignment: Some(ColumnTextAlignment::Right),
                    is_sortable: true,
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Stake Percentage".to_string(),
                    alignment: Some(ColumnTextAlignment::Center),
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Slots Produced".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Orphan Rate".to_string(),
                    alignment: Some(ColumnTextAlignment::Center),
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Supercharged Blocks Produced".to_string(),
                    alignment: Some(ColumnTextAlignment::Center),
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
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
                    section_heading="SNARK Fees Overview"
                />
            }
        }
    }
}
