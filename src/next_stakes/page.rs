use super::functions::*;
use crate::{
    common::{components::*, constants::*, functions::*, models::*, table::*},
    stakes::{components::EpochButton, models::EpochStyleVariant},
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    view! {
        <Title text="Next Staking Ledger | Search For Stakers"/>
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
            let public_key = params_map.get("q-key").cloned();
            let delegate = params_map.get("q-delegate").cloned();
            load_data(TABLE_RECORD_SIZE, public_key, delegate).await
        },
    );

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let get_data_and_pagination = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.nextstakes.len(),
                TABLE_DEFAULT_PAGE_SIZE,
                current_page.get(),
                set_current_page,
                page_dim.get().height.map(|h| h as usize),
                Some(Box::new(|container_height: usize| {
                    (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                        / ESTIMATED_ROW_HEIGHT
                })),
            );
            (data, pag)
        })
    };

    let table_columns = vec![
        TableColumn {
            column: "Key".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Stake".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Delegators".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Ledger Hash".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    view! {
        <ErrorBoundary fallback=move |_| ().into_view()>
            <TableSection
                section_heading="Next Staking Ledger"
                controls=move || {
                    view! {
                        <EpochButton
                            href="/staking-ledgers"
                            text="Previous"
                            style_variant=EpochStyleVariant::Secondary
                        />
                        <EpochButton
                            text="Next"
                            disabled=true
                            style_variant=EpochStyleVariant::Primary
                        />
                    }
                }
            >

                <TableContainer>
                    <Table>
                        <TableHeader columns=table_columns/>
                        <Suspense fallback=move || {
                            view! {
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    10
                                ]/>
                            }
                        }>
                            {move || {
                                get_data_and_pagination()
                                    .map(|(data, pag)| {
                                        view! {
                                            <TableRows data=data
                                                .nextstakes[pag
                                                    .start_index()..std::cmp::min(
                                                    pag.end_index() + 1,
                                                    pag.total_records,
                                                )]
                                                .to_vec()/>
                                        }
                                    })
                            }}

                        </Suspense>
                    </Table>
                </TableContainer>
                {move || {
                    get_data_and_pagination()
                        .map(|(_, pag)| {
                            view! { <Pagination pagination=pag/> }
                        })
                }}

            </TableSection>
        </ErrorBoundary>
    }
}
