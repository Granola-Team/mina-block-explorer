use super::{
    functions::*,
    graphql::top_stakers_query::TopStakersSortByInput,
    models::{DelegationTotals, StakerStats},
};
use crate::{
    common::{components::*, constants::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::create_query_signal;
use leptos_use::{UseTimeoutFnReturn, storage::use_local_storage, use_timeout_fn};
use std::collections::HashMap;

#[component]
pub fn StakerLeaderboard() -> impl IntoView {
    let (epoch_sig, set_epoch) = create_query_signal::<u32>("epoch");
    let (sort_dir_sig, _) = create_query_signal::<String>("sort-dir");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let resource = create_resource(
        move || (epoch_sig.get(), sort_dir_sig.get()),
        move |(epoch, sort_dir)| async move {
            load_data(
                epoch,
                sort_dir
                    .and_then(|dir| TopStakersSortByInput::try_from(dir).ok())
                    .unwrap_or(TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC),
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
                .and_then(|c| c.get(MAINNET_2_CHAIN_ID))
                .map(|c| c.latest_epoch)
                .unwrap_or_default(),
            set_epoch,
        ));
    });

    create_effect(move |_| {
        let top_stakers_opt = resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|res| res.top_stakers);
        let block_opt = resource
            .get()
            .and_then(|res| res.ok())
            .and_then(|res| res.blocks.first().cloned())
            .flatten();

        let updated_stakers = top_stakers_opt.map(|top_stakers| {
            top_stakers
                .into_iter()
                .map(|ts_opt| {
                    let ss = ts_opt.expect("Expected top stakers to be some");
                    StakerStats {
                        username: ss.username,
                        public_key: ss.public_key,
                        num_blocks_produced: ss.num_blocks_produced as u32,
                        num_canonical_blocks_produced: ss.num_canonical_blocks_produced as u32,
                        num_supercharged_blocks_produced: ss.num_supercharged_blocks_produced
                            as u32,
                        num_slots_produced: ss.num_slots_produced as u32,
                        delegation_totals: DelegationTotals {
                            total_stake_percentage: ss
                                .delegation_totals
                                .expect("Expected delegation totals to be some")
                                .total_stake_percentage,
                        },
                        epoch_num_slots_produced: block_opt
                            .as_ref()
                            .map(|bo| bo.epoch_num_slots_produced as u32),
                        epoch_num_canonical_blocks: block_opt
                            .as_ref()
                            .map(|bo| bo.epoch_num_canonical_blocks as u32),
                        epoch_num_blocks: block_opt.as_ref().map(|bo| bo.epoch_num_blocks as u32),
                    }
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
                    sort_direction: Some(AnySort::TopStakersSortByInput(
                        sort_dir_sig
                            .get()
                            .and_then(|dir| TopStakersSortByInput::try_from(dir).ok())
                            .unwrap_or(TopStakersSortByInput::NUM_CANONICAL_BLOCKS_PRODUCED_DESC),
                    )),
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
                    alignment: Some(ColumnTextAlignment::Center),
                    width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                    tooltip: Some("of total slots produced".to_string()),
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
