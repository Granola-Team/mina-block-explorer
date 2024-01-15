use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use leptos::*;
use leptos_router::use_query_map;
use crate::summary::functions::load_data as load_summary_data;

#[component]
pub fn StakesPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let epoch = map_string_to_i64(value.get("epoch").cloned());
            load_data(10, epoch, None).await
        },
    );

    let summary_resource = create_resource(||(), |_| async move {
        load_summary_data().await
    });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                let section_heading = match summary_resource.get() {
                    Some(Ok(r_data)) => format!("Stakes (Epoch {})", r_data.epoch),
                    _ => "Stakes".to_string()
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
