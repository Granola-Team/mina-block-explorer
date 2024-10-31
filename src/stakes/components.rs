use super::{functions::*, models::*};
use crate::{
    common::{components::*, constants::*, functions::*, models::*, table::*},
    stakes::graphql::staking_ledgers_query::StakeSortByInput,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn StakesPageContents(
    #[prop(into)] current_epoch: u64,
    #[prop(into)] slot_in_epoch: u64,
    #[prop(into)] epoch_num_accounts: Option<u64>,
    #[prop(into)] total_num_accounts: Option<u64>,
    selected_epoch: Option<u64>,
) -> impl IntoView {
    fn create_table_columns(total_stake_percent_sort: AnySort) -> Vec<TableColumn<AnySort>> {
        vec![
            TableColumn {
                column: "Key".to_string(),
                is_searchable: true,
                width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                ..Default::default()
            },
            TableColumn {
                column: "Username".to_string(),
                width: Some(String::from(TABLE_COL_USERNAME_WIDTH)),
                ..Default::default()
            },
            TableColumn {
                column: "Balance".to_string(),
                width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
                is_searchable: false,
                ..Default::default()
            },
            TableColumn {
                column: "Stake".to_string(),
                width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
                is_searchable: true,
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
                column: "Delegate".to_string(),
                is_searchable: true,
                width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                ..Default::default()
            },
            TableColumn {
                column: "Delegators".to_string(),
                width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
                alignment: Some(ColumnTextAlignment::Right),
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
    let (next_epoch_sig, _) = create_signal(next_epoch_opt);
    let (prev_epoch_sig, _) = create_signal(prev_epoch_opt);
    let (sort_dir, _) = create_query_signal::<String>("sort-dir");
    let (data_sig, set_data) = create_signal(None);
    let query_params_map = use_query_map();
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");

    let (ledger_hash, set_ledger_hash) = create_signal(None::<String>);

    let resource = create_resource(
        move || {
            (
                selected_epoch,
                query_params_map.get(),
                row_limit_sig.get(),
                sort_dir.get(),
            )
        },
        move |(epoch_opt, params_map, mut row_limit, sort_dir)| async move {
            let public_key = params_map.get("q-key").cloned();
            let delegate = params_map.get("q-delegate").cloned();
            let stake = params_map.get("q-stake").cloned();
            let mut sort_by = StakeSortByInput::STAKE_DESC;
            if let Some(s_dir) = sort_dir {
                match StakesSort::try_from(s_dir) {
                    Ok(StakesSort::StakeAsc) => {
                        sort_by = StakeSortByInput::STAKE_ASC;
                    }
                    Ok(StakesSort::StakeDesc) => sort_by = StakeSortByInput::STAKE_DESC,
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
        set_metadata.set(Some(TableMetadata {
            total_records: total_num_accounts,
            available_records: epoch_num_accounts,
            displayed_records: u64::try_from(data_sig.get().map(|d| d.len()).unwrap_or_default())
                .unwrap_or_default(),
        }));
    });

    {
        move || {
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
                                    text="Next"
                                    style_variant=ButtonStyleVariant::Primary
                                    epoch_target=next_epoch_sig.get().unwrap_or_default()
                                />
                            </div>
                        }
                    }

                    additional_info=view! {
                        <div class="h-8 min-w-64 text-sm text-slate-500 ledger-hash">
                            {move || {
                                ledger_hash
                                    .get()
                                    .map_or_else(
                                        || data_placeholder().into_view(),
                                        |lh| { convert_to_link(lh, "#".to_string()).into_view() },
                                    )
                            }}

                        </div>

                        {move || {
                            if next_epoch_sig
                                .get()
                                .map_or(false, |next_epoch| current_epoch == next_epoch - 1)
                            {
                                view! {
                                    <div class="text-sm text-slate-500 staking-ledger-percent-complete">
                                        {format!(
                                            "{:.2}% complete ({}/{} slots filled)",
                                            format_number(
                                                ((slot_in_epoch as f64 / EPOCH_SLOTS as f64) * 100.0)
                                                    .to_string(),
                                            ),
                                            format_number(slot_in_epoch.to_string()),
                                            format_number(EPOCH_SLOTS.to_string()),
                                        )}

                                    </div>
                                }
                                    .into_view()
                            } else {
                                ().into_view()
                            }
                        }}
                    }
                />
            }
        }
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
