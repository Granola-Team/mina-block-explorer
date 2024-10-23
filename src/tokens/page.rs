use super::functions::*;
use crate::common::{components::*, constants::TABLE_ROW_LIMIT, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, _) = create_signal(Some(stub_token_data(TABLE_ROW_LIMIT)));
    let (loading_sig, _) = create_signal(false);

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Token Name".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Token ID".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Supply".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Token Owner".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Token Holders".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Transaction Count".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Locked".to_string(),
            ..Default::default()
        },
    ];

    view! {
        <Title text="Tokens | Search" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=loading_sig.into()
                section_heading=(String::from("Tokens"), ().into_view())
                controls=|| ().into_view()
            />
        </PageContainer>
    }
}
