use super::functions::*;
use crate::{
    common::{components::*, constants::*, functions::*, models::*, search::*, table::*},
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

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
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
                                TABLE_DEFAULT_PAGE_SIZE,
                                current_page.get(),
                                set_current_page,
                                page_dim.get().height.map(|h| h as usize),
                                Some(
                                    Box::new(|container_height: usize| {
                                        (container_height
                                            - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                                            / ESTIMATED_ROW_HEIGHT
                                    }),
                                ),
                            );
                            let subset = get_subset(
                                &data.nextstakes,
                                pag.records_per_page,
                                current_page.get() - 1,
                            );
                            view! {
                                <TableSection
                                    section_heading="Next Staking Ledger"
                                    controls=move || {
                                        view! {
                                            <StakesNavButton
                                                href="/staking-ledgers"
                                                text="Current Stakes"
                                            />
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
