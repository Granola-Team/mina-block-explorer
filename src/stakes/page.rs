use super::functions::*;
use crate::common::components::*;
use leptos::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let resource = create_resource(
        || (),
        |_| async move { load_data(10, Some(69), None).await },
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
