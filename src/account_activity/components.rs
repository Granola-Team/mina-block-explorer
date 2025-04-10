use super::{
    graphql::account_activity_query::{
        AccountActivityQueryAccountsZkapp, AccountActivityQueryBlocks, AccountActivityQuerySnarks,
    },
    models::*,
};
use crate::{
    account_activity::{
        functions::*,
        graphql::account_activity_query::{
            AccountActivityQueryAccounts, AccountActivityQueryFeetransfers,
            AccountActivityQueryTokenHolders,
        },
        models::AccountActivityQueryDelegatorExt,
    },
    common::{components::*, constants::*, models::*, spotlight::SpotlightTable, table::*},
    icons::*,
    summary::models::BlockchainSummary,
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::use_params_map;
use leptos_use::storage::use_local_storage;

#[component]
pub fn ZkAppDetailsSection(zkapp: Option<AccountActivityQueryAccountsZkapp>) -> impl IntoView {
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);

    if let Some(zkapp) = zkapp {
        let zkapp_clone = zkapp.clone();
        view! {
            <TableSection metadata=metadata.into() section_heading="ZKApp Details".to_string()>
                <SpotlightTable>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"App State:"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CopyToClipboard>
                                <CodeBlock>
                                    {get_app_state(&zkapp)
                                        .ok()
                                        .unwrap_or("Unable to serialize app state".to_string())}
                                </CodeBlock>
                            </CopyToClipboard>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"Action State:"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CopyToClipboard>
                                <CodeBlock>
                                    {get_action_state(&zkapp_clone)
                                        .ok()
                                        .unwrap_or("Unable to serialize action state".to_string())}
                                </CodeBlock>
                            </CopyToClipboard>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                </SpotlightTable>
            </TableSection>
        }
    } else {
        ().into_view()
    }
}

#[component]
pub fn AccountTransactionsSection(
    transactions_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDirectionalTransactions>>>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("Expected an account to be provided in the context");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let memo_params_map = use_params_map();
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Txn Hash".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Nonce".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            search_type: ColumnSearchType::Select,
            search_options: Some(vec!["".to_string(), TYPE_SEARCH_OPTION_ZKAPP.to_string()]),
            ..Default::default()
        },
        TableColumn {
            column: "Direction".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Counterparty".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Amount/Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=transactions_sig
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_user_commands).ok(),
                    displayed_records: u64::try_from(
                            transactions_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: account
                        .get()
                        .and_then(|a| {
                            a.pk_total_num_user_commands.and_then(|t| u64::try_from(t).ok())
                        }),
                })
            })

            section_heading="User Commands"
            is_loading
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        />
        {move || {
            transactions_sig
                .get()
                .filter(|d| !d.is_empty())
                .map(|_| {
                    view! {
                        <TableLink
                            href=format!(
                                "/commands/user?{}={}",
                                QUERY_PARAM_TO,
                                memo_params_map.get().get("id").cloned().unwrap_or_default(),
                            )
                            text="See all user commands"
                        >
                            <CheckCircleIcon />
                        </TableLink>
                    }
                })
        }}
    }
}

#[component]
pub fn AccountInternalCommandsSection(
    txn_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryFeetransfers>>>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("Expected an account to be provided in the context");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let memo_params_map = use_params_map();
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Type".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=txn_sig
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_internal_commands)
                        .ok(),
                    displayed_records: u64::try_from(
                            txn_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: account
                        .get()
                        .and_then(|a| {
                            a.pk_total_num_internal_commands.and_then(|t| u64::try_from(t).ok())
                        }),
                })
            })

            section_heading="Internal Commands"
            is_loading
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        />
        {move || {
            txn_sig
                .get()
                .filter(|d| !d.is_empty())
                .map(|_| {
                    view! {
                        <TableLink
                            href=format!(
                                "/commands/internal?{}={}",
                                QUERY_PARAM_RECEIPIENT,
                                memo_params_map.get().get("id").cloned().unwrap_or_default(),
                            )
                            text="See all internal commands"
                        >
                            <CheckCircleIcon />
                        </TableLink>
                    }
                })
        }}
    }
}

#[component]
pub fn AccountDelegationsSection(
    delegations_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>>,
    delegator_count: Option<usize>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let memo_params_map = use_params_map();
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Public Key".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Username".to_string(),
            width: Some(String::from(TABLE_COL_USERNAME_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Delegated Balance".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
            sort_direction: Some(AnySort::Delegator(Delegators::BalanceDesc)),
            ..Default::default()
        },
        TableColumn {
            column: "% of Delegation".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=delegations_sig
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: delegator_count.and_then(|n| n.try_into().ok()),
                    displayed_records: u64::try_from(
                            delegations_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: None,
                })
            })

            section_heading="Delegations"
            is_loading
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                }
            }
        />
        {move || {
            delegations_sig
                .get()
                .filter(|d| !d.is_empty())
                .map(|_| {
                    view! {
                        <TableLink
                            href=format!(
                                "/staking-ledgers?{}={}",
                                QUERY_PARAM_DELEGATE,
                                memo_params_map.get().get("id").cloned().unwrap_or_default(),
                            )
                            text="See all delegators"
                        >
                            <CheckCircleIcon />
                        </TableLink>
                    }
                })
        }}
    }
}

#[component]
pub fn AccountOverviewSnarkJobTable(
    snarks_sig: ReadSignal<Option<Vec<Option<AccountActivityQuerySnarks>>>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("Expected an account to be provided in the context");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let memo_params_map = use_params_map();
    let (href, _set_href) = create_signal(
        memo_params_map
            .get()
            .get("id")
            .as_ref()
            .map(|pk| format!("/snarks?q-prover={}", pk))
            .unwrap_or_else(|| "/snarks".to_string()),
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Prover".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=snarks_sig
            section_heading="SNARK Jobs"
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: u64::try_from(summary_sig.get().total_num_snarks).ok(),
                    displayed_records: u64::try_from(
                            snarks_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: account
                        .get()
                        .and_then(|a| { a.pk_total_num_snarks.and_then(|t| u64::try_from(t).ok()) }),
                })
            })

            is_loading
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                }
            }
        />
        {move || {
            snarks_sig
                .get()
                .filter(|d| !d.is_empty())
                .map(|_| {
                    view! {
                        <TableLink href=href.get() text="See all snark jobs">
                            <CheckCircleIcon />
                        </TableLink>
                    }
                })
        }}
    }
}

#[component]
pub fn AccountOverviewBlocksTable(
    blocks_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryBlocks>>>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("Expected an account to be provided in the context");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let memo_params_map = use_params_map();
    let (href, _set_href) = create_signal(
        memo_params_map
            .get()
            .get("id")
            .as_ref()
            .map(|pk| format!("/blocks?q-block-producer={}", pk))
            .unwrap_or_else(|| "/blocks".to_string()),
    );

    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Height".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "State Hash".to_string(),
            search_type: ColumnSearchType::Text,
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Slot".to_string(),
            search_type: ColumnSearchType::Text,
            html_input_type: "number".to_string(),
            alignment: Some(ColumnTextAlignment::Right),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Date".to_string(),
            width: Some(String::from(TABLE_COL_DATE_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Block Producer".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Coinbase".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "User Commands".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Snarks".to_string(),
            width: Some(String::from(TABLE_COL_NUMERIC_WIDTH)),
            alignment: Some(ColumnTextAlignment::Right),
            ..Default::default()
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=blocks_sig
            metadata=Signal::derive(move || {
                Some(TableMetadata {
                    total_records: Some(summary_sig.get().total_num_blocks),
                    displayed_records: u64::try_from(
                            blocks_sig.get().map(|a| a.len()).unwrap_or_default(),
                        )
                        .unwrap_or_default(),
                    available_records: account
                        .get()
                        .and_then(|a| { a.pk_total_num_blocks.and_then(|t| u64::try_from(t).ok()) }),
                })
            })

            section_heading="Block Production"
            is_loading
            controls=move || {
                view! {
                    <div class="hidden md:flex justify-center items-center">
                        <RowLimit />
                    </div>
                }
            }
        />
        {move || {
            blocks_sig
                .get()
                .filter(|d| !d.is_empty())
                .map(|_| {
                    view! {
                        <TableLink href=href.get() text="See all block production">
                            <CheckCircleIcon />
                        </TableLink>
                    }
                })
        }}
    }
}

#[component]
pub fn AccountOverviewTokensTable(
    tokens_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryTokenHolders>>>>,
    is_loading: Signal<bool>,
) -> impl IntoView {
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Token Symbol".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Token ID".to_string(),
            width: Some(String::from(TABLE_COL_HASH_WIDTH)),
            ..Default::default()
        },
        TableColumn {
            column: "Balance".to_string(),
            width: Some(String::from(TABLE_COL_LARGE_BALANCE)),
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
            column: "More".to_string(),
            width: Some(String::from(TABLE_COL_SHORT_WIDTH)),
            ..Default::default()
        },
    ];

    view! {
        <TableSectionTemplate
            table_columns
            data_sig=tokens_sig
            metadata=Signal::derive(move || { None })

            section_heading="Tokens"
            is_loading
            controls=|| ().into_view()
        />
    }
}
