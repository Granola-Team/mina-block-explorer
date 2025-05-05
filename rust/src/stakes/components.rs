use super::{functions::*, models::*};
use crate::{
    common::{components::*, constants::*, functions::*, models::*, table::*},
    stakes::graphql::staking_ledgers_query::{StakesSortByInput, StakingLedgersQueryStakes},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn StakesPageContents(
    #[prop(into)] current_epoch: u64,
    #[prop(into)] slot_in_epoch: u64,
    #[prop(into)] epoch_num_accounts: Option<u64>,
    #[prop(into)] total_num_accounts: Option<u64>,
    #[prop(into)] genesis_state_hash: Option<String>,
    selected_epoch: Option<u64>,
    #[prop(into, optional)] chain_id: String,
) -> impl IntoView {
    fn create_table_columns(total_stake_percent_sort: AnySort) -> Vec<TableColumn<AnySort>> {
        vec![
            TableColumn {
                column: "Account".to_string(),
                search_type: ColumnSearchType::Text,
                width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                ..Default::default()
            },
            TableColumn {
                column: "Balance".to_string(),
                width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
                ..Default::default()
            },
            TableColumn {
                column: "Stake".to_string(),
                width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
                search_type: ColumnSearchType::Text,
                html_input_type: "number".to_string(),
                ..Default::default()
            },
            TableColumn {
                column: "Total Stake %".to_string(),
                sort_direction: Some(total_stake_percent_sort),
                is_sortable: true,
                width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                alignment: Some(ColumnTextAlignment::Right),
                ..Default::default()
            },
            TableColumn {
                column: "Block Win %".to_string(),
                width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                alignment: Some(ColumnTextAlignment::Right),
                ..Default::default()
            },
            TableColumn {
                column: "Delegators".to_string(),
                width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                alignment: Some(ColumnTextAlignment::Right),
                ..Default::default()
            },
            TableColumn {
                column: "Delegate".to_string(),
                search_type: ColumnSearchType::Text,
                width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                ..Default::default()
            },
        ]
    }

    let (metadata_sig, set_metadata) = create_signal(None);
    let header_epoch = selected_epoch.unwrap_or(current_epoch);
    let next_epoch = header_epoch + 1;
    let prev_epoch = header_epoch.saturating_sub(1); // prevents underflow

    let next_epoch_opt = if next_epoch > current_epoch + 1 {
        None
    } else {
        Some(next_epoch)
    };
    let prev_epoch_opt = Some(prev_epoch);

    let section_heading = format!("Staking Ledger - Epoch {}", header_epoch);
    let (section_heading_sig, _) = create_signal(section_heading);
    let (next_epoch_sig, _) = create_signal(Some(next_epoch));
    let (prev_epoch_sig, _) = create_signal(prev_epoch_opt);
    let (sort_dir, _) = create_query_signal::<String>("sort-dir");
    let (data_sig, set_data) = create_signal(None);
    let query_params_map = use_query_map();
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");
    let (genesis_state_hash_sig, _) = create_signal(genesis_state_hash);

    let (ledger_hash, set_ledger_hash) = create_signal(None::<String>);

    let resource = create_resource(
        move || {
            (
                selected_epoch,
                query_params_map.get(),
                row_limit_sig.get(),
                sort_dir.get(),
                genesis_state_hash_sig.get(),
            )
        },
        move |(epoch_opt, params_map, mut row_limit, sort_dir, genesis_state_hash)| async move {
            let public_key = params_map.get("q-account").cloned();
            let delegate = params_map.get("q-delegate").cloned();
            let stake = params_map.get("q-stake").cloned();
            let mut sort_by = StakesSortByInput::STAKE_DESC;
            if let Some(s_dir) = sort_dir {
                match StakesSort::try_from(s_dir) {
                    Ok(StakesSort::StakeAsc) => {
                        sort_by = StakesSortByInput::STAKE_ASC;
                    }
                    Ok(StakesSort::StakeDesc) => sort_by = StakesSortByInput::STAKE_DESC,
                    Err(_) => (),
                };
            }

            load_data(
                Some(*row_limit.get_or_insert(25i64)),
                Some(epoch_opt.unwrap_or(current_epoch)),
                public_key,
                delegate,
                stake,
                sort_by,
                genesis_state_hash,
            )
            .await
        },
    );

    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        get_data().map(|data| {
            set_data.set(Some(data.stakes.clone()));
            let ledger_hash = data
                .stakes
                .first()
                .and_then(|x| x.as_ref())
                .and_then(|x| x.ledger_hash.to_owned());

            set_ledger_hash.set(ledger_hash);
        })
    });

    create_effect(move |_| {
        let mut available_records = None;
        let qp_map = query_params_map.get();
        let public_key = qp_map.get("q-account").cloned();
        let delegate = qp_map.get("q-delegate").cloned();
        let stake = qp_map.get("q-stake").cloned();
        if stake.is_none() && delegate.is_none() && public_key.is_none() {
            available_records = epoch_num_accounts;
        }
        set_metadata.set(Some(TableMetadata {
            total_records: total_num_accounts,
            available_records,
            displayed_records: u64::try_from(data_sig.get().map(|d| d.len()).unwrap_or_default())
                .unwrap_or_default(),
        }));
    });

    {
        move || {
            let chain_id_clone = chain_id.to_string();
            let s_dir = sort_dir
                .get()
                .and_then(|s| StakesSort::try_from(s).ok())
                .unwrap_or(StakesSort::StakeDesc);
            let table_columns = create_table_columns(AnySort::Stakes(s_dir));
            view! {
                <TableSectionTemplate
                    table_columns
                    data_sig
                    metadata=metadata_sig.into()
                    section_heading=section_heading_sig.get()
                    is_loading=resource.loading()
                    controls=move || {
                        view! {
                            <div class="hidden md:flex justify-center items-center space-x-4">
                                <RowLimit />
                                <UrlParamSelectMenu
                                    id="berkeley_selection"
                                    query_str_key="is-berkeley"
                                    labels=UrlParamSelectOptions {
                                        is_boolean_option: true,
                                        cases: vec![
                                            format!("{} (Berkeley)", BERKELEY_CHAIN_ID),
                                            format!("{}", MAINNET_CHAIN_ID),
                                        ],
                                    }
                                />
                                <EpochButton
                                    disabled=prev_epoch_sig
                                        .get()
                                        .map(|prev_epoch| prev_epoch == 0)
                                        .unwrap_or_default()
                                    text="Previous"
                                    style_variant=ButtonStyleVariant::Secondary
                                    epoch_target=prev_epoch_sig.get().unwrap_or_default()
                                />
                                <EpochButton
                                    disabled=next_epoch_opt.is_none()
                                        || next_epoch_opt
                                            .is_some_and(|ne| {
                                                ne == LAST_EPOCH_OF_MAINNET_CHAIN
                                                    && chain_id_clone == MAINNET_CHAIN_ID
                                            })
                                    text="Next"
                                    style_variant=ButtonStyleVariant::Primary
                                    epoch_target=next_epoch_sig.get().unwrap_or_default()
                                />
                            </div>
                        }
                    }
                    footer=move || {
                        view! {
                            <NextStakePage
                                data=data_sig.get().unwrap_or(vec![])
                                row_limit=row_limit_sig.get()
                            />
                        }
                    }
                    additional_info=move || {
                        view! {
                            <div class="h-8 min-w-64 text-sm text-slate-500 ledger-hash">
                                {move || {
                                    ledger_hash
                                        .get()
                                        .map_or_else(
                                            || data_placeholder().into_view(),
                                            |lh| {
                                                convert_to_copy_link(lh, "#".to_string()).into_view()
                                            },
                                        )
                                }}

                            </div>

                            {move || {
                                let resolved_slot_in_epoch = match next_epoch_opt {
                                    Some(next_epoch) => {
                                        if current_epoch == next_epoch - 1 {
                                            slot_in_epoch
                                        } else {
                                            EPOCH_SLOTS as u64
                                        }
                                    }
                                    None => 0_u64,
                                };
                                view! {
                                    <div class="text-sm text-slate-500 staking-ledger-percent-complete">
                                        {format!(
                                            "{:.0}% complete ({}/{} slots filled)",
                                            (resolved_slot_in_epoch as f64 / EPOCH_SLOTS as f64)
                                                * 100.0,
                                            format_number(resolved_slot_in_epoch.to_string()),
                                            format_number(EPOCH_SLOTS.to_string()),
                                        )}

                                    </div>
                                }
                                    .into_view()
                            }}
                        }
                    }
                />
            }
        }
    }
}

#[component]
pub fn NextStakePage(
    data: Vec<Option<StakingLedgersQueryStakes>>,
    row_limit: Option<i64>,
) -> impl IntoView {
    let (_, set_stake) = create_query_signal_with_options::<String>(
        QUERY_PARAM_STAKE,
        NavigateOptions {
            scroll: false,
            ..Default::default()
        },
    );
    let mut last_stake = None;
    if let Some(Some(last_row)) = data.last() {
        last_stake = last_row
            .delegation_totals
            .as_ref()
            .and_then(|delegation_totals| delegation_totals.total_delegated_nanomina)
            .map(|stake| nanomina_to_mina(stake as u64))
            .as_ref()
            .and_then(|stake| normalize_number_format(stake).ok())
    }
    let last_stake_clone = last_stake.clone();
    view! {
        <div class="w-full flex justify-center items-center p-4">
            <Button
                style_variant=ButtonStyleVariant::Tertiary
                text="Load Next"
                on_click=move |_| { set_stake.set(last_stake.clone()) }
                class_str="ml-2"
                disabled=data.len() as i64 != row_limit.unwrap_or(TABLE_ROW_LIMIT as i64)
                    || *"0.0" == last_stake_clone.unwrap_or("0.0".to_string())
            />
        </div>
    }
}

#[component]
pub fn EpochButton(
    #[prop(into)] text: String,
    #[prop(optional)] epoch_target: u64,
    #[prop(default = false)] disabled: bool,
    style_variant: ButtonStyleVariant,
    #[prop(default=String::new(), into)] href: String,
) -> impl IntoView {
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold ml-2 flex justify-center items-center border border-granola-orange border-[1px]";
    let mut button_variant_styles = format!(
        "{} {}",
        button_base_styles,
        get_button_style_variation(&style_variant)
    );
    button_variant_styles = match disabled {
        true => format!(
            "{} {}",
            button_variant_styles,
            "bg-slate-100 text-slate-400 border-slate-100 hover:cursor-not-allowed"
        ),
        false => button_variant_styles,
    };

    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    if href.clone().is_empty() {
        let handle_click = move |_| {
            if disabled {
                return;
            }
            let pathname = location.pathname.get();
            let mut pm = query_params_map.get();
            pm.insert("epoch".to_string(), epoch_target.to_string());

            logging::log!("{}", pm.to_query_string());
            logging::log!("{}", pathname);

            navigate(
                &format!("{}{}", pathname, pm.to_query_string()),
                Default::default(),
            );
        };
        view! { <Button on_click=handle_click text=text style_variant disabled /> }.into_view()
    } else {
        view! {
            <a href=href class=button_variant_styles + " ml-2">
                {text}
            </a>
        }
        .into_view()
    }
}
