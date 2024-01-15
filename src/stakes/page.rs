use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use leptos::*;
use leptos_router::use_query_map;

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

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Stakes".to_string()>
                    <Table data=data.stakes/>
                </TableSection>
             },
            _ => view! { <NullView /> }
        }}
    }
}
