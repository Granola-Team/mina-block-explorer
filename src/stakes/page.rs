use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::summary::functions::load_data as load_summary_data;
use leptos::*;
use leptos_router::{use_location, use_navigate, use_query_map, NavigateOptions, State};
use std::cmp::{max, min};

#[component]
pub fn StakesPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let query_string_epoch = move || query_params_map.with(|params| params.get("epoch").cloned());

    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

    let current_epoch = move || match summary_resource.get() {
        Some(Ok(data)) => Some(data.epoch),
        _ => None,
    };

    let resource = create_resource(
        move || (query_params_map.get(), current_epoch()),
        move |(params_map, c_epoch)| async move {
            let q_str_epoch = params_map.get("epoch");
            let public_key = params_map.get("query").cloned();
            let resolved_epoch = match (c_epoch, q_str_epoch) {
                (Some(epoch), None) => Some(epoch as i64),
                _ => q_str_epoch.and_then(|s| s.parse::<i64>().ok()),
            };
            load_data(10, resolved_epoch, public_key).await
        },
    );

    view! {
        <SearchBar placeholder="Exact search for public key".to_string()/>
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    let (previous_epoch, next_epoch, section_heading) = match (current_epoch(), query_string_epoch()) {
                        (Some(curr_epoch), Some(qs_epoch)) => {
                            let i_curr_epoch = curr_epoch as i64;
                            match qs_epoch.parse::<i64>() {
                                Ok(i_qs_epoch) => {
                                    let i_next_epoch = min(i_curr_epoch, i_qs_epoch+1);
                                    let i_prev_epoch = max(0, i_qs_epoch-1);
                                    let header = if i_next_epoch == i_qs_epoch { "Current Staking Ledger".to_string() } else { format!("Epoch {} Staking Ledger",i_qs_epoch)};
                                    (i_prev_epoch,i_next_epoch,header)
                                },
                                _ => (0,0, "".to_string())
                            }

                        },
                        (Some(curr_epoch), None) => {
                            ((curr_epoch-1) as i64, (curr_epoch+1) as i64, "Current Staking Ledger".to_string())
                        },
                        _ => (0,0, "".to_string())
                    };
                    view! {
                        <TableSection section_heading=section_heading controls=move || view! {
                            <EpochButton text="Previous".to_string()
                                style_variant=EpochStyleVariant::Secondary
                                epoch_target=previous_epoch/>
                            <EpochButton text="Next".to_string()
                                style_variant=EpochStyleVariant::Primary
                                epoch_target=next_epoch/>
                        }>
                            <Table data=data.stakes/>
                        </TableSection>
                    }
                },
                _ => view! { <NullView /> }
            }}
        </PageContainer>
    }
}

enum EpochStyleVariant {
    Primary,
    Secondary,
}

#[component]
fn EpochButton(
    text: String,
    epoch_target: i64,
    #[prop(default = false)] disabled: bool,
    style_variant: EpochStyleVariant,
) -> impl IntoView {
    let button_base_styles = "text-sm rounded-md p-2 h-9 font-semibold mx-2 flex justify-center items-center border border-granola-orange border-[1px]";
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
            button_variant_styles, "bg-slate-400 border-slate-400 hover:cursor-not-allowed"
        ),
        false => button_variant_styles,
    };

    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let handle_click = move |_event| {
        let pathname = location.pathname.get();
        let mut pm = query_params_map.get();
        pm.insert("epoch".to_string(), epoch_target.to_string());

        logging::log!("{}", pm.to_query_string());
        logging::log!("{}", pathname);

        navigate(
            &format!("{}{}", pathname, pm.to_query_string()),
            NavigateOptions {
                resolve: true,
                replace: false,
                scroll: false,
                state: State(None),
            },
        );
    };

    view! {
        <button on:click=handle_click class=button_variant_styles>
            {text}
        </button>
    }
}
