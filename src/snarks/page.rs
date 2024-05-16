use super::functions::*;
use crate::common::{
    components::*,
    constants::{TABLE_RECORD_SIZE, *},
    functions::*,
    models::*,
    table::*,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::{create_query_signal, use_query_map};
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
pub fn SnarksPage() -> impl IntoView {
    view! {
        <Title text="SNARKs | Search For SNARKs"/>
        <PageContainer>
            <SnarksPageContents/>
        </PageContainer>
    }
}

#[component]
fn SnarksPageContents() -> impl IntoView {
    let query_params_map = use_query_map();
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");
    let (block_height_sig, _) = create_query_signal::<i64>("q-height");
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                canonical_qp.get(),
                block_height_sig.get(),
            )
        },
        |(_, value, canonical, block_height)| async move {
            let prover = value.get("q-prover");
            let block_state_hash = value.get("q-state-hash");
            load_data(
                TABLE_RECORD_SIZE,
                prover.cloned(),
                block_state_hash.cloned(),
                block_height,
                canonical,
            )
            .await
        },
    );

    let page_dim = use_context::<ReadSignal<PageDimensions>>()
        .expect("there to be a `PageDimensions` signal provided");
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Prover".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Fee".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    let get_pagination_and_data = move || {
        resource.get().and_then(|res| res.ok()).map(|data| {
            let pag = build_pagination(
                data.snarks.len(),
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

    view! {
        <TableSection
            section_heading="SNARKs"
            controls=move || {
                view! {
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        >

            <TableContainer>
                <Table>
                    <TableHeader columns=table_columns/>
                    <Suspense fallback=move || {
                        view! {
                            <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                        }
                    }>
                        {move || {
                            get_pagination_and_data()
                                .map(|(data, pag)| {
                                    let subset = data
                                        .snarks[pag
                                            .start_index()..std::cmp::min(
                                            pag.end_index() + 1,
                                            pag.total_records,
                                        )]
                                        .to_vec();
                                    view! { <TableRows data=subset/> }
                                })
                        }}

                    </Suspense>
                </Table>
            </TableContainer>
            {move || {
                {
                    move || {
                        get_pagination_and_data()
                            .map(|(_, pag)| {
                                view! { <Pagination pagination=pag/> }
                            })
                    }
                }
            }}

        </TableSection>
    }
}
