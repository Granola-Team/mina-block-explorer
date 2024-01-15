use super::functions::*;
use crate::common::components::*;
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn SnarksPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let public_key = value.get("account");
            load_data(10, public_key.cloned()).await
        },
    );

    view! {
        <MainContainer>
            {move || match resource.get() {
                Some(Ok(data)) => view! {
                    <TableSection section_heading="SNARKs".to_owned()>
                        <Table data=data.snarks/>
                    </TableSection>
                },
                _ => view! { <span /> }.into_view()
            }}
        </MainContainer>
    }
}
