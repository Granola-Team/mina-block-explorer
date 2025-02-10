use super::functions::*;
use crate::{
    common::{components::*, models::UrlParamSelectOptions, table::*},
    tokens::models::TokenDataSortBy,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    let (row_limit_sig, _) = create_query_signal::<u64>("row-limit");
    let (name_sig, _) = create_query_signal::<String>("q-name");

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
                data_sig
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
            />

        </PageContainer>
    }
}
