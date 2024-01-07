use super::functions::*;
use super::components::*;
use leptos::*;
use leptos_router::*;
use crate::common::functions::print_time_since;
use crate::common::spotlight::{Spotlight, SpotlightEntry};
use crate::icons::*;

#[component]
pub fn LatestBlocksPage() -> impl IntoView {
    view! { <BlocksSection /> }
}

#[component]
pub fn BlockSpotlight() -> impl IntoView {
    let memo_params_map = use_params_map();
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(10, None, state_hash.cloned()).await
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                match data.blocks.get(0) {
                    Some(Some(block)) => {
                        let state_hash = get_state_hash(block);
                        let date_time = get_date_time(block);
                        let summary_items = vec![
                            SpotlightEntry { label: "State Hash".to_string(), value: state_hash, pill_variant: None},
                        ];
                        view!{
                            <section class="@container md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section p-0 md:p-4 mb-2">
                                <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Block Spotlight"</h1>
                                <Spotlight summary_items=summary_items id=get_state_hash(block) meta=format!("{} ({})", date_time, print_time_since(&date_time)) >
                                    <BlockIcon width=40/>
                                </Spotlight>
                            </section>
                        }.into_view()
                    },
                    _ => view! {}.into_view()
                }
                
            }
            _ => view! {}.into_view()
        }}
        
    }
}