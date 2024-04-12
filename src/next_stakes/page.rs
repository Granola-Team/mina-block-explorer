use super::functions::*;
use crate::{
    common::{components::*, constants::TABLE_RECORD_SIZE, functions::*, search::*, table::*},
    stakes::components::StakesNavButton,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    view! {
        <Title text="Next Staking Ledger | Search For Stakers"/>
        <SearchBar placeholder="Exact search for public key"/>
        <PageContainer>
            <NextStakesPageContents/>
        </PageContainer>
    }
}

#[component]
fn NextStakesPageContents() -> impl IntoView {
    let query_params_map = use_query_map();
    let resource = create_resource(
        move || query_params_map.get(),
        |params_map| async move {
            let public_key = params_map.get("query");
            load_data(TABLE_RECORD_SIZE, public_key.cloned()).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    view! {
        <ErrorBoundary fallback=move |_| ().into_view()>
            <Suspense fallback=move || {
                view! {
                    <TableSection
                        section_heading="Next Staking Ledger"
                        controls=move || ().into_view()
                    >
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }>
                {move || {
                    resource
                        .get()
                        .and_then(|res| res.ok())
                        .map(|data| {
                            let pag = build_pagination(
                                data.nextstakes.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                                None,
                                None,
                            );
                            let subset = get_subset(
                                &data.nextstakes,
                                records_per_page,
                                current_page.get() - 1,
                            );
                            view! {
                                <TableSection
                                    section_heading="Next Staking Ledger"
                                    controls=move || {
                                        view! {
                                            <StakesNavButton href="/stakes" text="Current Stakes"/>
                                        }
                                    }
                                >

                                    <Table data=subset pagination=pag/>
                                </TableSection>
                            }
                        })
                }}

            </Suspense>
        </ErrorBoundary>
    }
}
