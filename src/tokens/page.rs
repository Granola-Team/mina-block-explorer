use super::functions::*;
use crate::common::{components::*, constants::TABLE_ROW_LIMIT, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn TokensPage() -> impl IntoView {
    let (data_sig, _) = create_signal(Some(stub_token_data(TABLE_ROW_LIMIT)));
    let (loading_sig, _) = create_signal(false);

    let table_columns = vec![
        TableColumn {
            column: "Token Name".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Token ID".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Supply".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Token Owner".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Token Holders".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Transaction Count".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Locked".to_string(),
            is_searchable: false,
        },
    ];

    view! {
        <Title text="Tokens | Search For Tokens"/>
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=loading_sig.into()
                section_heading="Tokens"
                controls=|| ().into_view()
            />
        </PageContainer>
    }
}
