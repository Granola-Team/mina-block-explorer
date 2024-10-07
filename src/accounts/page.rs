use crate::{
    accounts::{functions::*, models::AccountsSort},
    common::{components::*, constants::*, models::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;
use leptos_use::storage::use_local_storage;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Accounts | Search for accounts on Mina Blockchain" />
        <PageContainer>
            <AccountsPageContents />
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (data_sig, set_data) = create_signal(None);
    let (public_key_sig, _) = create_query_signal::<String>("q-public-key");
    let (username_sig, _) = create_query_signal::<String>("q-username");
    let (balance_sig, _) = create_query_signal::<i64>("q-balance");
    let (delegate_sig, _) = create_query_signal::<String>("q-delegate");
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");
    let resource = create_resource(
        move || {
            (
                public_key_sig.get(),
                username_sig.get(),
                balance_sig.get(),
                delegate_sig.get(),
                row_limit_sig.get(),
            )
        },
        |(public_key, username, balance, delegate, mut row_limit)| async move {
            load_data(
                Some(*row_limit.get_or_insert(25i64)),
                public_key,
                username,
                balance.map(|b| b * 1_000_000_000i64),
                delegate,
            )
            .await
        },
    );
    let table_columns: Vec<TableColumn<AnySort>> = vec![
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
            sort_direction: Some(AnySort::Accounts(AccountsSort::BalanceDesc)),
            is_searchable: true,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
        },
        TableColumn {
            column: "Nonce".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Delegate".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            is_searchable: true,
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
                    total_records: Some(summary_sig.get().total_num_accounts),
                    displayed_records: u64::try_from(
                            data_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: None,
                })
            })

            section_heading="Accounts"
            is_loading=resource.loading()
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <UrlParamSelectMenu
                            label="Rows"
                            id="row-limit"
                            query_str_key="row-limit"
                            labels=UrlParamSelectOptions {
                                is_boolean_option: false,
                                cases: vec![
                                    "25".to_string(),
                                    "50".to_string(),
                                    "100".to_string(),
                                    "250".to_string(),
                                ],
                            }
                        />
                    </div>
                }
            }
        />
    }
}
