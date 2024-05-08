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

            {move || match resource.get() {
                Some(Ok(data)) => {
                    let pag = build_pagination(
                        data.snarks.len(),
                        TABLE_DEFAULT_PAGE_SIZE,
                        current_page.get(),
                        set_current_page,
                        page_dim.get().height.map(|h| h as usize),
                        Some(
                            Box::new(|container_height: usize| {
                                (container_height - DEFAULT_ESTIMATED_NON_TABLE_SPACE_IN_SECTIONS)
                                    / ESTIMATED_ROW_HEIGHT
                            }),
                        ),
                    );
                    let subset = get_subset(
                        &data.snarks,
                        pag.records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <DeprecatedTable data=subset pagination=pag/> }
                }
                None => view! { <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/> },
                _ => view! { <span></span> }.into_view(),
            }}

        </TableSection>
    }
}
