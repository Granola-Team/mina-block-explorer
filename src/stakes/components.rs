use super::{functions::*, models::*};
use crate::common::{constants::*, functions::convert_to_link, table::*};
use leptos::*;
use leptos_router::*;

#[component]
pub fn StakesPageContents(
    #[prop(into)] current_epoch: i64,
    #[prop(into)] slot_in_epoch: i64,
) -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let (data_sig, set_data) = create_signal(None);
    let query_params_map = use_query_map();

    let (ledger_hash, set_ledger_hash) = create_signal(None::<String>);

    let resource = create_resource(
        move || (epoch_sig.get(), query_params_map.get()),
        move |(epoch_opt, params_map)| async move {
            let public_key = params_map.get("q-key").cloned();
            let delegate = params_map.get("q-delegate").cloned();
            load_data(
                Some(epoch_opt.unwrap_or(current_epoch)),
                public_key,
                delegate,
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

    let get_heading_and_epochs = create_memo(move |_| {
        let mut section_heading = "Staking Ledger - Epoch ".to_string();
        let mut next_epoch = current_epoch + 1;
        let mut prev_epoch = current_epoch - 1;
        let header_epoch = if let Some(qs_epoch) = epoch_sig.get() {
            if qs_epoch != current_epoch {
                next_epoch = qs_epoch + 1;
                next_epoch = next_epoch.clamp(0, current_epoch + 1);
                prev_epoch = qs_epoch - 1;
                qs_epoch
            } else {
                current_epoch
            }
        } else {
            current_epoch
        };
        section_heading += format!("{}", header_epoch).as_str();
        (section_heading, current_epoch, next_epoch, prev_epoch)
    });

    let table_columns = vec![
        TableColumn {
            column: "Key".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Username".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Stake".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Total Stake %".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Delegators".to_string(),
            is_searchable: false,
        },
    ];

    {
        let table_columns_clone = table_columns.clone();
        move || {
            let table_columns_clone = table_columns_clone.clone();
            let (section_heading, current_epoch, next_epoch, prev_epoch) =
                get_heading_and_epochs.get();
            view! {
                <TableSectionTemplate
                    table_columns=table_columns_clone
                    data_sig
                    section_heading
                    is_loading=resource.loading()
                    controls=move || {
                        view! {
                            <EpochButton
                                disabled=prev_epoch < 0
                                text="Previous"
                                style_variant=EpochStyleVariant::Secondary
                                epoch_target=prev_epoch
                            />
                            <EpochButton
                                text="Next"
                                style_variant=EpochStyleVariant::Primary
                                epoch_target=next_epoch
                            />
                        }
                    }

                    additional_info=view! {
                        {match ledger_hash.get() {
                            Some(data) => {
                                view! {
                                    <div class="text-sm text-slate-500">
                                        {convert_to_link(data, "#".to_string())}
                                    </div>
                                }
                                    .into_view()
                            }
                            None => ().into_view(),
                        }}

                        {if next_epoch - 1 == current_epoch {
                            view! {
                                <div class="text-sm text-dark-blue staking-ledger-percent-complete">
                                    {format!(
                                        "{:.2}% complete ({}/{} slots filled)",
                                        ({ slot_in_epoch } as f64 / { EPOCH_SLOTS } as f64) * 100.0,
                                        { slot_in_epoch },
                                        { EPOCH_SLOTS },
                                    )}

                                </div>
                            }
                                .into_view()
                        } else {
                            ().into_view()
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
    #[prop(optional)] epoch_target: i64,
    #[prop(default = false)] disabled: bool,
    style_variant: EpochStyleVariant,
    #[prop(default=String::new(), into)] href: String,
) -> impl IntoView {
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold ml-2 flex justify-center items-center border border-granola-orange border-[1px]";
    let mut button_variant_styles = match style_variant {
        EpochStyleVariant::Primary => {
            format!("{} {}", button_base_styles, "text-white bg-granola-orange")
        }
        EpochStyleVariant::Secondary => {
            format!("{} {}", button_base_styles, "text-granola-orange bg-white")
        }
    };
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
        view! {
            <button on:click=handle_click class=button_variant_styles>
                {text}
            </button>
        }
        .into_view()
    } else {
        view! {
            <a href=href class=button_variant_styles>
                {text}
            </a>
        }
        .into_view()
    }
}
