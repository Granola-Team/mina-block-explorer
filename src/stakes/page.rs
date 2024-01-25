use super::components::*;
use super::functions::*;
use super::models::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::summary::functions::load_data as load_summary_data;
use leptos::*;
use leptos_router::*;
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
                    let (previous_epoch, next_epoch, curr_epoch, section_heading) = match (current_epoch(), query_string_epoch()) {
                        (Some(curr_epoch), Some(qs_epoch)) => {
                            let i_curr_epoch = curr_epoch as i64;
                            match qs_epoch.parse::<i64>() {
                                Ok(i_qs_epoch) => {
                                    let i_next_epoch = min(i_curr_epoch, i_qs_epoch+1);
                                    let i_prev_epoch = max(0, i_qs_epoch-1);
                                    let header = if i_next_epoch == i_qs_epoch { "Current Staking Ledger".to_string() } else { format!("Epoch {} Staking Ledger",i_qs_epoch)};
                                    (i_prev_epoch,i_next_epoch,i_curr_epoch,header)
                                },
                                _ => (0,0,0, "".to_string())
                            }

                        },
                        (Some(curr_epoch), None) => {
                            ((curr_epoch-1) as i64, (curr_epoch) as i64, (curr_epoch) as i64, "Current Staking Ledger".to_string())
                        },
                        _ => (0,0,0, "".to_string())
                    };
                    view! {
                        <TableSection section_heading=section_heading controls=move || view! {
                            <EpochButton text="Previous".to_string()
                                style_variant=EpochStyleVariant::Secondary
                                epoch_target=previous_epoch/>
                            { if next_epoch == curr_epoch {
                                view! {
                                    <StakesNavButton href="/next-stakes".to_string() text="Next Stakes".to_string() />
                                }
                            } else {
                                view! {
                                    <EpochButton text="Next".to_string()
                                        style_variant=EpochStyleVariant::Primary
                                        epoch_target=next_epoch/>
                                }
                            }}
                        }>
                            <Table data=data.stakes/>
                        </TableSection>
                    }
                },
                None => view! {
                    <TableSection section_heading=String::new() controls=move || view! { <NullView /> }>
                        <Table data=LoadingPlaceholder{}/>
                    </TableSection>
                },
                _ => view! { <NullView /> }
            }}
        </PageContainer>
    }
}
