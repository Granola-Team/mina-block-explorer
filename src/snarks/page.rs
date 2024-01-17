use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn SnarksPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let mut public_key = value.get("account");
            if public_key.is_none() {
                public_key = value.get("query");
            }
            load_data(10, public_key.cloned()).await
        },
    );

    view! {
        <SearchBar placeholder="Exact search for prover".to_string() />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => view! {
                    <TableSection section_heading="SNARKs".to_owned()>
                        <Table data=data.snarks/>
                    </TableSection>
                },
                _ => view! { <span /> }.into_view()
            }}
        </PageContainer>
    }
}
