use super::functions::*;
use crate::{
    common::{
        components::*,
        constants::TABLE_COL_USERNAME_WIDTH,
        models::{TableMetadata, UrlParamSelectOptions},
        table::*,
    },
    tokens::models::{TokenData, TokenDataSortBy},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    let (total_count_sig, set_total_count) = create_signal(None::<i64>);
    let (row_limit_sig, _) = create_query_signal::<u64>("row-limit");
    let (name_sig, _) = create_query_signal::<String>("q-name");

    // Get total unfiltered count on page load
    create_effect(move |_| {
        spawn_local(async move {
            if let Ok((_, count)) = load_data(1, None, None, None, None, false).await {
                set_total_count.set(Some(count));
            }
        });
    });

    let resource = create_resource(
        move || name_sig.get(),
        move |name_opt| async move {
            load_data(
                row_limit_sig.get().unwrap_or(50),
                name_opt,
                None,
                None,
                Some(TokenDataSortBy::Transactions),
                false,
            )
            .await
        },
    );
    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_data.set(Some(data)))
    });
    let (loading_sig, _) = create_signal(false);

    // Create a signal for just the data part
    let data_only = create_signal(None);
    create_effect(move |_| {
        if let Some((data, _)) = data_sig.get() {
            data_only.1.set(Some(data));
        }
    });

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Name".to_string(),
            html_input_type: "text".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_USERNAME_WIDTH)),
            alignment: Some(ColumnTextAlignment::Left),
            ..Default::default()
        },
        TableColumn {
            column: "ID".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Supply".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Owner".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Holders".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Transactions".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Locked".to_string(),
            ..Default::default()
        },
    ];

    view! {
        <Title text="Tokens | Search For Tokens" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig=data_only.0
                is_loading=loading_sig.into()
                controls=move || {
                    view! {
                        // Avoiding RowLimit component so we can set default
                        <div class="hidden md:flex justify-center items-center">

                            <UrlParamSelectMenu
                                label="Rows"
                                id="row-limit"
                                query_str_key="row-limit"
                                labels=UrlParamSelectOptions {
                                    is_boolean_option: false,
                                    cases: vec![
                                        "50".to_string(),
                                        "100".to_string(),
                                        "250".to_string(),
                                        "500".to_string(),
                                        "1000".to_string(),
                                    ],
                                }
                            />
                        </div>
                    }
                }
                section_heading="Tokens"
                metadata=Signal::derive(move || {
                                    data_sig.get().map(|(data, _)| TableMetadata {
                                        displayed_records: u64::try_from(data.len()).unwrap_or_default(),
                                        available_records: u64::try_from(data.len()).ok(),
                                        total_records: total_count_sig.get()
                                            .map(|count| u64::try_from(count).unwrap_or_default()),
                                    })
                                })
            />

        </PageContainer>
    }
}
