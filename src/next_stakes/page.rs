use super::functions::*;
use crate::{
    common::{
        components::*,
        functions::convert_to_link,
        models::{MyError, TableMetadata},
        table::*,
    },
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
    let (metadata, set_metadata) = create_signal(Some(TableMetadata::default()));
    let query_params_map = use_query_map();

    let (ledger_hash, set_ledger_hash) = create_signal(None::<String>);

    let resource = create_resource(
        move || query_params_map.get(),
        move |params_map| async move {
            let public_key = params_map.get("q-key").cloned();
            let delegate = params_map.get("q-delegate").cloned();

            let response = load_data(public_key, delegate).await;
            match &response {
                Ok(data) => {
                    let ledger_hash = data
                        .nextstakes
                        .first()
                        .and_then(|x| x.as_ref())
                        .and_then(|x| x.ledger_hash.to_owned());
                    if ledger_hash.is_some() {
                        set_ledger_hash.set(ledger_hash);
                    }
                    response
                }
                _ => Err(MyError::ParseError(String::from(
                    "missing epoch information",
                ))),
            }
        },
    );

    let get_data = move || resource.get().and_then(|res| res.ok());

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
    ];
    let table_cols_length = table_columns.len();

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_metadata.set(Some(TableMetadata {
                total_records: "all".to_string(),
                displayed_records: data.nextstakes.len() as i64,
            }))
        }
    });

    view! {
        <ErrorBoundary fallback=move |_| ().into_view()>
            <TableSection
                metadata
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

                additional_info=view! {
                    {match ledger_hash.get() {
                        Some(data) => {
                            view! {
                                <div class="text-sm text-slate-500">
                                    {convert_to_link(data, "#".to_string())}
                                </div>
                            }
                                .into_view()
                        }
                        None => ().into_view(),
                    }}
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
                                get_data()
                                    .map(|data| {
                                        view! { <TableRows data=data.nextstakes/> }
                                    })
                            }}

                        </Suspense>
                    </Table>
                </TableContainer>
            </TableSection>
        </ErrorBoundary>
    }
}
