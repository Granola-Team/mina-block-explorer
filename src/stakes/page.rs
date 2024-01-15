use leptos::*;
use super::functions::*;
use crate::common::components::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        {match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Stakes".to_string()>
                    <Table data=data/>
                </TableSection>
             },
            _ => view! { <NullView /> }
        }}
    }
}
