use crate::{
    accounts::functions::*,
    common::{components::*, constants::TABLE_ROW_LIMIT, models::*, table::*},
};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Accounts | Search For Mina Account"/>
        <PageContainer>
            <AccountsPageContents/>
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    let resource = create_resource(|| (), |_| async move { load_data(TABLE_ROW_LIMIT).await });
    let table_columns = vec![
        TableColumn {
            column: "Public Key".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Username".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Balance".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Nonce".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Delegate".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Time Locked".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();
    let get_data = move || resource.get().and_then(|res| res.ok());
    view! {
        <TableSection metadata section_heading="Accounts" controls=|| ().into_view()>
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
                                view! { <TableRows data=data.accounts/> }
                            })
                    }}

                </Suspense>
            </Table>
        </TableSection>
    }
}
