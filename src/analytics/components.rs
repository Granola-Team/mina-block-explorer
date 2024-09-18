use super::{functions::*, models::*};
use crate::common::{
    components::*,
    table::{TableColumn, TableSectionTemplate},
};
use leptos::*;
use leptos_router::create_query_signal;
use std::collections::HashMap;
// #[component]
// pub fn AnalayticsFilters() -> impl IntoView {}

#[component]
pub fn SnarkFees() -> impl IntoView {
    let default_block_limit = 1000;
    let (limit_sig, set_limit) = create_query_signal::<u64>("limit");
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
                    section_heading=format!(
                        "SNARK Fees in latest {} blocks",
                        limit_sig.get().unwrap_or(default_block_limit),
                    )

                    controls=move || {
                        view! {
                            <ControlledInput
                                id="block-selection"
                                input_type="number"
                                name="block-selection"
                                disabled_sig=resource.loading()
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
                                    ("max".to_string(), "5000".to_string()),
                                ])
                            />
                        }
                    }
                />
            }
        }
    }
}
