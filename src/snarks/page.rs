use super::functions::*;
use crate::common::components::*;
use crate::common::search::*;
use crate::common::table::*;
use crate::common::functions::*;
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
            load_data(50, public_key.cloned(), None).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <SearchBar placeholder="Exact search for prover".to_string()/>
        <PageContainer>
            <TableSection section_heading="SNARKs".to_owned() controls=|| ().into_view()>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    let pag = build_pagination(data.snarks.len(), records_per_page, current_page.get(), set_current_page);
                    let subset = get_subset(&data.snarks, records_per_page, current_page.get()-1);
                    view! {
                        <Table data=subset pagination=pag/>
                    }
                },
                None => view! { <Table data=LoadingPlaceholder{} />},
                _ => view! { <span /> }.into_view()
            }}
            </TableSection>
        </PageContainer>
    }
}
