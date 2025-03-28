use super::functions::*;
use crate::{
    common::{
        components::*,
        constants::{
            QUERY_PARAM_TOKEN_SYMBOL, TABLE_COL_HASH_WIDTH, TABLE_COL_LARGE_BALANCE,
            TABLE_COL_NUMERIC_WIDTH, TABLE_COL_USERNAME_WIDTH,
        },
        models::{TableMetadata, UrlParamSelectOptions},
        table::*,
    },
    tokens::graphql::tokens_query,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    let (total_count_sig, set_total_count) = create_signal(None::<i64>);
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");
    let (name_sig, _) = create_query_signal::<String>(QUERY_PARAM_TOKEN_SYMBOL);

    let resource = create_resource(
        move || name_sig.get(),
        move |name_opt| async move {
            load_data(
                row_limit_sig.get().or(Some(50)),
                name_opt,
                Some(tokens_query::TokensSortByInput::SUPPLY_DESC),
            )
            .await
        },
    );
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.tokens.clone()));
            if let Some(first_token) = data.tokens.first().cloned().flatten() {
                set_total_count.set(Some(first_token.total_num_tokens));
            }
        };
    });

    create_effect(move |_| {
        if let Some(res) = get_data() {
            if let Some(first_token) = res.tokens.first().cloned().flatten() {
                set_total_count.set(Some(first_token.total_num_tokens));
            }
        }
    });

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Name".to_string(),
            html_input_type: "text".to_string(),
            is_searchable: true,
            is_sortable: true,
            width: Some(String::from(TABLE_COL_USERNAME_WIDTH)),
            alignment: Some(ColumnTextAlignment::Left),
            ..Default::default()
        },
        TableColumn {
            column: "Supply".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            alignment: Some(ColumnTextAlignment::Left),
            ..Default::default()
        },
        TableColumn {
            column: "ID".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            alignment: Some(ColumnTextAlignment::Left),
            ..Default::default()
        },
        TableColumn {
            column: "Owner".to_string(),
            is_sortable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            alignment: Some(ColumnTextAlignment::Left),
            ..Default::default()
        },
        TableColumn {
            column: "Holders".to_string(),
            is_sortable: true,
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Transactions".to_string(),
            is_sortable: true,
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "% Unlocked".to_string(),
            is_sortable: true,
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
    ];

    view! {
        <Title text="Tokens | Search For Tokens" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=resource.loading()
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
                    data_sig
                        .get()
                        .map(|data| TableMetadata {
                            displayed_records: u64::try_from(data.len()).unwrap_or_default(),
                            available_records: u64::try_from(data.len()).ok(),
                            total_records: total_count_sig
                                .get()
                                .map(|count| u64::try_from(count).unwrap_or_default()),
                        })
                })
            />

        </PageContainer>
    }
}
