use super::functions::*;
use crate::common::components::*;
use leptos::*;


#[component]
pub fn SnarksPage() -> impl IntoView {

    

    let resource = create_resource(|| (), |_| async move { load_data(10, None).await });

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="SNARKs".to_owned()>
                    <Table data=data.snarks/>
                </TableSection>
            },
            _ => view! { <span /> }.into_view()
        }}
    }
}
