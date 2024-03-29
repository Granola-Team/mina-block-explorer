use super::functions::*;
use crate::common::{components::*, functions::*, models::*, search::*, table::*};
use leptos::*;
use leptos_meta::Title;
use leptos_router::{create_query_signal, use_query_map};

#[component]
pub fn SnarksPage() -> impl IntoView {
    let query_params_map = use_query_map();
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let resource = create_resource(
        move || (query_params_map.get(), canonical_qp.get()),
        |(value, canonical)| async move {
            let mut public_key = value.get("account");
            if public_key.is_none() {
                public_key = value.get("query");
            }
            load_data(50, public_key.cloned(), None, canonical).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <Title text="SNARKs | Search For SNARKs"/>
        <SearchBar placeholder="Exact search for prover".to_string()/>
        <PageContainer>
            <TableSection
                section_heading="SNARKs".to_owned()
                controls=move || {
                    view! {
                        <BooleanUrlParamSelectMenu
                            id="canonical-selection"
                            query_str_key="canonical"
                            labels=BooleanUrlParamSelectOptions {
                                true_case: String::from("Canonical"),
                                false_case: String::from("Non-Canonical"),
                            }
                        />
                    }
                }
            >

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
