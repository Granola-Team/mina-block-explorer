use crate::{
    accounts::functions::*,
    common::{
        components::*,
        constants::{TABLE_ROW_LIMIT, *},
        models::*,
        table::*,
    },
    summary::models::BlockchainSummary,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Accounts | Search for accounts on Mina Blockchain"/>
        <PageContainer>
            <AccountsPageContents/>
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (data_sig, set_data) = create_signal(None);
    let (public_key_sig, _) = create_query_signal::<String>("q-public-key");
    let (username_sig, _) = create_query_signal::<String>("q-username");
    let (balance_sig, _) = create_query_signal::<i64>("q-balance");
    let resource = create_resource(
        move || (public_key_sig.get(), username_sig.get(), balance_sig.get()),
        |(public_key, username, balance)| async move {
            load_data(
                TABLE_ROW_LIMIT,
                public_key,
                username,
                balance.map(|b| b * 1_000_000_000i64),
            )
            .await
        },
    );
    let table_columns = vec![
        TableColumn {
            column: "Public Key".to_string(),
            is_searchable: true,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Username".to_string(),
            width: Some(String::from(TABLE_COL_USERNAME_WIDTH)),
            is_searchable: true,
            ..Default::default()
        },
        TableColumn {
            column: "Balance".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            sort_direction: Some(TableSortDirection::Desc),
            is_searchable: true,
            ..Default::default()
        },
        TableColumn {
            column: "Nonce".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Delegate".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Time Locked".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
    ];
    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.accounts))
        } else {
            set_data.set(Some(vec![]))
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_accounts).ok(),
                    displayed_records: u64::try_from(
                            data_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: None,
                })
            })

            section_heading="Accounts"
            is_loading=resource.loading()
            controls=|| ().into_view()
        />
    }
}
