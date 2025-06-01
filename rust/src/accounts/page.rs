use super::{graphql::accounts_query, models::TokenData};
use crate::{
    accounts::{components::NextAccountsPage, functions::*, models::AccountsSort},
    common::{components::*, constants::*, models::*, table::*},
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_local_storage;

#[component]
pub fn AccountsPage() -> impl IntoView {
    view! {
        <Title text="Mina Addresses | Search for accounts on Mina Blockchain" />
        <PageContainer>
            <AccountsPageContents />
        </PageContainer>
    }
}

#[component]
fn AccountsPageContents() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let memo_params_map = use_params_map();
    let (data_sig, set_data) = create_signal(None);
    let (account_sig, _) = create_query_signal::<String>(QUERY_PARAM_ACCOUNT);
    let (balance_sig, _) = create_query_signal::<f64>(QUERY_PARAM_BALANCE);
    let (delegate_sig, _) = create_query_signal::<String>(QUERY_PARAM_DELEGATE);
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");
    let (sort_dir_sig, _) = create_query_signal::<String>("sort-dir");
    let (q_type_sig, _) = create_query_signal::<String>(QUERY_PARAM_TYPE);
    let (token_sig, set_token) = create_signal::<Option<TokenData>>(None);

    let get_token_id = move || memo_params_map.get().get("token_id").cloned();

    let public_key_memo = Memo::new(move |_| {
        account_sig
            .get()
            .and_then(|account| match PublicKey::new(account) {
                Ok(public_key) => Some(public_key),
                Err(_) => None,
            })
    });
    let username_memo = Memo::new(move |_| {
        account_sig
            .get()
            .and_then(|account| match PublicKey::new(account) {
                Ok(_) => None,
                Err(_) => Some(
                    account_sig
                        .get()
                        .expect("Expected to find username in q-account"),
                ),
            })
    });

    let resource = create_resource(
        move || {
            (
                public_key_memo.get(),
                username_memo.get(),
                balance_sig.get(),
                delegate_sig.get(),
                row_limit_sig.get(),
                sort_dir_sig.get(),
                q_type_sig.get(),
                get_token_id(),
            )
        },
        |(public_key, username, balance, delegate, mut row_limit, sort_dir, q_type, q_token)| async move {
            let s_dir = if let Some(s) = sort_dir.and_then(|s| AccountsSort::try_from(s).ok()) {
                s
            } else {
                AccountsSort::BalanceDesc
            };
            let sort_by = match s_dir {
                AccountsSort::BalanceDesc => accounts_query::AccountSortByInput::BALANCE_DESC,
                AccountsSort::BalanceAsc => accounts_query::AccountSortByInput::BALANCE_ASC,
            };
            let is_zk_app = q_type.is_some_and(|p| p == TYPE_SEARCH_OPTION_ZKAPP);

            load_data(
                Some(*row_limit.get_or_insert(25i64)),
                public_key,
                username,
                balance,
                delegate,
                Some(sort_by),
                Some(is_zk_app),
                q_token,
            )
            .await
        },
    );
    let token_resource = create_resource(get_token_id, |token| async move {
        load_token_symbol(token).await
    });

    let get_data = move || resource.get().and_then(|res| res.ok());
    let get_token = move || token_resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        if let Some(data) = get_data() {
            set_data.set(Some(data.accounts))
        } else {
            set_data.set(Some(vec![]))
        }
    });

    create_effect(move |_| {
        if let Some(token) =
            get_token().and_then(|token_resp| token_resp.data.tokens.first().cloned())
        {
            set_token.set(Some(token));
        }
    });

    {
        move || {
            let s_dir = if let Some(s) = sort_dir_sig
                .get()
                .and_then(|s| AccountsSort::try_from(s).ok())
            {
                s
            } else {
                AccountsSort::BalanceDesc
            };
            let table_columns: Vec<TableColumn<AnySort>> = vec![
                TableColumn {
                    column: "Type".to_string(),
                    search_type: ColumnSearchType::Select,
                    search_options: Some(vec![
                        "".to_string(),
                        TYPE_SEARCH_OPTION_ZKAPP.to_string(),
                    ]),
                    width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Account".to_string(),
                    search_type: ColumnSearchType::Text,
                    width: Some(String::from(TABLE_COL_HASH_WIDTH)),
                    ..Default::default()
                },
                TableColumn {
                    column: "Balance".to_string(),
                    width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
                    sort_direction: Some(AnySort::Accounts(s_dir)),
                    search_type: ColumnSearchType::Text,
                    html_input_type: "number".to_string(),
                    alignment: Some(ColumnTextAlignment::Right),
                    ..Default::default()
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
                    search_type: ColumnSearchType::Text,
                    ..Default::default()
                },
                TableColumn {
                    column: "Time Locked".to_string(),
                    width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
                    ..Default::default()
                },
            ];
            view! {
                <TableSectionTemplate
                    table_columns
                    data_sig
                    metadata=Signal::derive(move || {
                        Some(
                            TableMetadataBuilder::new()
                                .displayed_records_value(
                                    data_sig.get().map(|a| a.len() as u64).unwrap_or_default(),
                                    None,
                                )
                                .available_records(
                                    move || {
                                        get_token_id().expect("expect token to be a URL param")
                                            != MINA_TOKEN_ADDRESS && q_type_sig.get().is_none()
                                    },
                                    token_sig.get().map(|t| t.num_holders).unwrap_or_default(),
                                    Some(
                                        format!(
                                            "Total {} accounts",
                                            token_sig.get().map(|t| t.symbol).unwrap_or_default(),
                                        ),
                                    ),
                                )
                                .available_records(
                                    move || {
                                        get_token_id().expect("expect token to be a URL param")
                                            == MINA_TOKEN_ADDRESS && q_type_sig.get().is_none()
                                    },
                                    summary_sig.get().total_num_mina_accounts,
                                    Some("Total MINA accounts".to_string()),
                                )
                                .available_records(
                                    move || {
                                        get_token_id().expect("expect token to be a URL param")
                                            == MINA_TOKEN_ADDRESS
                                            && q_type_sig
                                                .get()
                                                .as_ref()
                                                .map(|t| t == TYPE_SEARCH_OPTION_ZKAPP)
                                                .unwrap_or(false)
                                    },
                                    summary_sig.get().total_num_mina_zkapp_accounts,
                                    Some("All MINA zkApp accounts".to_string()),
                                )
                                .total_records_value(
                                    summary_sig.get().total_num_accounts,
                                    Some("All accounts for all tokens".to_string()),
                                )
                                .build(),
                        )
                    })
                    section_heading=token_sig
                        .get()
                        .map(|t| format!("{} Token Accounts", t.symbol))
                        .unwrap_or("Accounts".to_string())
                    is_loading=resource.loading()
                    footer=move || {
                        view! {
                            <NextAccountsPage
                                data=data_sig.get().unwrap_or(vec![])
                                row_limit=row_limit_sig.get().map(|rl| rl as u64)
                            />
                        }
                    }
                    controls=move || {
                        view! {
                            <div class="hidden md:flex justify-center items-center">
                                <RowLimit />
                            </div>
                        }
                    }
                />
            }
        }
    }
}
