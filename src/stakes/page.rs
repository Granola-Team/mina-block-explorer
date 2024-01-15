use super::functions::*;
use crate::common::components::*;
use leptos::*;
use leptos_router::use_query_map;
use crate::summary::functions::load_data as load_summary_data;

#[component]
pub fn StakesPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let epoch = move || {
        query_params_map.with(|params| params.get("epoch").cloned())
    };

    let summary_resource = create_resource(||(), |_| async move {
        load_summary_data().await
    });

    let current_epoch = move || match summary_resource.get() {
        Some(Ok(data)) => Some(data.epoch),
        _ => None
    };

    let resource = create_resource(
        move || query_params_map.get(),
        move |value| async move {
            let q_str_epoch = value.get("epoch").clone();
            let resolved_epoch = match (current_epoch(), q_str_epoch) {
                (Some(epoch), None) => Some(epoch as i64),
                _ => q_str_epoch.and_then(|s| s.parse::<i64>().ok())
            }; 
            load_data(10, resolved_epoch, None).await
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                let section_heading = match summary_resource.get() {
                    Some(Ok(r_data)) => {
                        let e_str = r_data.epoch.to_string();
                        let e_qry = epoch().unwrap_or_default();
                        if e_str == e_qry {
                            "Current Staking Ledger".to_string()
                        } else {
                            format!("Epoch {} Staking Ledger",e_str)
                        }
                    },
                    _ => "Staking Ledger".to_string()
                };
                view! {
                    <TableSection section_heading=section_heading>
                        <Table data=data.stakes/>
                    </TableSection>
                }
            },
            _ => view! { <NullView /> }
        }}
    }
}
