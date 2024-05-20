use crate::{
    common::{components::*, constants::*, models::*, table::*},
    internal_commands::functions::load_data,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::create_query_signal;

#[component]
pub fn InternalCommandsTab() -> impl IntoView {
    let (metadata, set_metadata) = create_signal(Some(TableMetadata::default()));
    let (recipient, _) = create_query_signal::<String>("q-recipient");
    let (height_sig, _) = create_query_signal::<i64>("q-height");
    let (state_hash_sig, _) = create_query_signal::<String>("q-state-hash");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let resource = create_resource(
        move || {
            (
                recipient.get(),
                height_sig.get(),
                state_hash_sig.get(),
                canonical_sig.get(),
            )
        },
        |(opt_recipient, height, state_hash, canonical)| async move {
            load_data(opt_recipient, height, state_hash, canonical).await
        },
    );

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
            column: "Recipient".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Fee".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Type".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_metadata.set(Some(TableMetadata {
                total_records: "all".to_string(),
                displayed_records: data.feetransfers.len() as i64,
            }))
        }
    });
    view! {
        <Title text="Transactions | Internal Commands"/>
        <PageContainer>
            <TableSection
                metadata
                section_heading="Internal Commands"
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
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    TABLE_ROW_LIMIT.try_into().unwrap()
                                ]/>
                            }
                        }>
                            {move || {
                                get_data()
                                    .map(|data| {
                                        if data.feetransfers.is_empty() {
                                            return view! {
                                                <EmptyTable message="No internal commands found"/>
                                            };
                                        }
                                        view! { <TableRows data=data.feetransfers/> }
                                    })
                            }}

                        </Suspense>
                    </Table>
                </TableContainer>
            </TableSection>
        </PageContainer>
    }
}
