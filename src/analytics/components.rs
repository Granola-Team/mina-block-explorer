use super::{functions::*, models::*};
use crate::common::table::{TableColumn, TableSectionTemplate};
use leptos::*;
use leptos_router::create_query_signal;

#[component]
pub fn SnarkFees() -> impl IntoView {
    let (limit_sig, set_limit) = create_query_signal::<u64>("limit");
    let resource = create_resource(
        move || limit_sig.get(),
        move |limit| async move { load_snark_fees(limit).await },
    );
    let (data_sig, set_data) = create_signal(None);

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

    create_effect(move |_| {
        set_data.set(Some(
            resource
                .get()
                .and_then(|res| res.ok())
                .map(|data| SnarkStatsContainer::from(data.data.blocks)),
        ));
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            is_loading=resource.loading()
            section_heading="SNARK Fees in latest blocks"
            controls=move || {
                view! {
                    <input
                        id="block-selection"
                        type="number"
                        on:input=move |ev| {
                            set_limit.set(event_target_value(&ev).parse::<u64>().ok())
                        }

                        disabled=resource.loading()
                        name="block-selection"
                        step=200
                        value=limit_sig.get()
                        max=5000
                        min=200
                        class="block h-8 text-base text-sm font-normal font-mono p-2 text-right border rounded-sm border-slate-400 focus:border-granola-orange"
                    />
                    <label
                        for="block-selection"
                        class="flex items-center h-8 text-base text-sm font-normal font-mono p-2 whitespace-nowrap"
                    >
                        "latest blocks"
                    </label>
                }
            }
        />
    }
}
