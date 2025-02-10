use super::functions::*;
use crate::{
    common::{components::*, table::*},
    tokens::models::TokenDataSortBy,
};
use leptos::*;
use leptos_meta::*;
// use leptos_router::create_query_signal;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    // let (row_limit_sig, _) = create_query_signal::<u64>("row-limit");

    let resource = create_resource(
        || (),
        |_| async move {
            load_data(
                // row_limit_sig.get(),
                Some(100),
                None,
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
                // controls=move || {
                // view! {
                // <div class="hidden md:flex justify-center items-center">
                // <RowLimit />
                // </div>
                // }
                // }
                section_heading="Tokens"
            />

        </PageContainer>
    }
}
