use crate::{
    accounts::functions::*,
    common::{components::*, constants::TABLE_ROW_LIMIT, table::*},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Accounts | Search For Mina Accounts"/>
        <PageContainer>
            <AccountsPageContents/>
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let (data_sig, set_data) = create_signal(None);
    let (public_key_sig, _) = create_query_signal::<String>("q-public-key");
    let resource = create_resource(
        move || public_key_sig.get(),
        |public_key| async move { load_data(TABLE_ROW_LIMIT, public_key).await },
    );
    let table_columns = vec![
        TableColumn {
            column: "Public Key".to_string(),
            is_searchable: true,
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
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.accounts))
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            section_heading="Accounts"
            is_loading=resource.loading()
            controls=|| ().into_view()
        />
    }
}
