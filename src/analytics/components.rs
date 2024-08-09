use super::{functions::*, models::*};
use crate::common::{
    constants::DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL,
    table::{TableColumn, TableSectionTemplate},
};
use leptos::*;
use leptos_router::create_query_signal;
use leptos_use::{use_debounce_fn_with_options, DebounceOptions};

#[component]
pub fn SnarkFees() -> impl IntoView {
    let default_block_limit = 1000;
    let input_element: NodeRef<html::Input> = create_node_ref();
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

    create_effect(move |_| {
        if limit_sig.get().is_none() {
            set_limit.set(Some(default_block_limit));
        }
    });

    let (handle_limit_sig, _) = create_signal(use_debounce_fn_with_options(
        move || {
            let v = input_element
                .get()
                .expect("<input/> should be mounted")
                .value();
            set_limit.set(v.parse::<u64>().ok());
        },
        DEFAULT_USER_INPUT_DEBOUNCE_INTERNVAL,
        DebounceOptions::default(),
    ));

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
                            <input
                                id="block-selection"
                                type="number"
                                on:keypress=move |ev| {
                                    ev.prevent_default();
                                }

                                on:input=move |_| {
                                    let handle_limit = handle_limit_sig.get_untracked();
                                    handle_limit();
                                }

                                disabled=resource.loading()
                                name="block-selection"
                                step=1000
                                value=limit_sig.get()
                                max=5000
                                min=1000
                                class="block h-8 text-base text-sm font-normal font-mono p-2 text-right border rounded-sm border-slate-400 focus:border-granola-orange"
                                node_ref=input_element
                            />
                        }
                    }
                />
            }
        }
    }
}
