use super::functions::*;
use crate::common::{components::*, functions::*, search::*, table::*};
use leptos::*;
use leptos_router::*;

#[component]
pub fn SnarksPage() -> impl IntoView {
    let (public_key, _) = create_query_signal::<String>("account");
    let (query, _) = create_query_signal::<String>("query");
    let (canonical, _) = create_query_signal::<bool>("canonical");
    let (block_state_hash, _) = create_query_signal::<String>("block");

    let resource = create_resource(
        move || {
            (
                public_key.get(),
                query.get(),
                canonical.get(),
                block_state_hash.get(),
            )
        },
        |(public_key, query, canonical, block_state_hash)| async move {
            let mut pk = public_key.clone();
            if public_key.is_none() {
                pk = query;
            }
            load_data(50, pk, block_state_hash, canonical).await
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
                        let pag = build_pagination(
                            data.snarks.len(),
                            records_per_page,
                            current_page.get(),
                            set_current_page,
                        );
                        let subset = get_subset(
                            &data.snarks,
                            records_per_page,
                            current_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }
                    None => view! { <Table data=LoadingPlaceholder {}/> },
                    _ => view! { <span></span> }.into_view(),
                }}

            </TableSection>
        </PageContainer>
    }
}
