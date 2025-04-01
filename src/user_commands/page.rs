use super::{components::*, functions::*, table_trait::*};
use crate::{
    common::{components::*, constants::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn CommandsTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/commands/user".to_string(),
            text: "User Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/commands/pending".to_string(),
            text: "Pending Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/commands/internal".to_string(),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        // NavEntry {
        //     href: "/commands/zk-app".to_string(),
        //     text: "zkApp Commands".to_string(),
        //     icon: NavIcon::ZKApps,
        //     ..Default::default()
        // },
    ];

    view! { <TabbedPage tabs=tabs /> }
}

#[component]
pub fn UserCommandsPage() -> impl IntoView {
    view! {
        <Title text="Commands | Search For Commands" />
        <PageContainer>
            <TransactionsSection />
        </PageContainer>
    }
}

#[component]
pub fn PendingCommandsPage() -> impl IntoView {
    view! {
        <Title text="Commands | Search For Commands" />
        <PageContainer>
            <PendingTransactionsSection />
        </PageContainer>
    }
}

#[component]
pub fn CommandSpotlightPage() -> impl IntoView {
    let (metadata, _) = create_signal::<Option<TableMetadata>>(None);
    let memo_params_map = use_params_map();
    let (state_hash_sig, _) = create_query_signal::<String>("q-state-hash");
    let (txn_memo, set_txn_memo) = create_signal("No Memo".to_string());
    let resource = create_resource(
        move || (memo_params_map.get(), state_hash_sig.get()),
        |(value, state_hash)| async move {
            let txn_hash = value.get("id");
            load_data(
                Some(10),
                None,
                None,
                txn_hash.cloned(),
                None,
                None,
                state_hash,
                None,
                None,
                None,
                None,
            )
            .await
        },
    );
    let (other_txns, set_other_txns) = create_signal(None);

    create_effect(move |_| {
        if let Some(Ok(data)) = resource.get() {
            if let Some(Some(txn)) = data.transactions.first() {
                set_txn_memo.set(txn.get_memo());
            }
        }
    });

    let get_data = move || resource.get().and_then(|res| res.ok());

    create_effect(move |_| {
        let Some(data) = get_data() else {
            return;
        };
        let Some(spotlight_txn) = data.transactions.first().cloned().flatten() else {
            return;
        };

        let txns_in_other_blocks = data
            .other_transactions
            .into_iter()
            .filter(|txn_opt| {
                txn_opt.as_ref().is_some_and(|txn| {
                    txn.get_block_state_hash() != spotlight_txn.get_block_state_hash()
                })
            })
            .collect::<Vec<_>>();

        if !txns_in_other_blocks.is_empty() {
            set_other_txns.set(Some(txns_in_other_blocks));
        }
    });

    view! {
        <Title
            formatter=move |text| format!("Transaction Overview | {text}")
            text=move || txn_memo.get()
        />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    match data.transactions.first().cloned() {
                        Some(Some(transaction)) => {
                            let txn_clone_1 = transaction.clone();
                            let txn_clone_2 = transaction.clone();
                            let state_hash = transaction.get_hash();
                            let date_time = transaction.get_block_datetime();
                            let has_succeeded = transaction.get_failure_reason().is_none();
                            let status = if has_succeeded {
                                TXN_STATUS_APPLIED
                            } else {
                                TXN_STATUS_FAILED
                            };
                            let status_color = if has_succeeded {
                                ColorVariant::Green
                            } else {
                                ColorVariant::Orange
                            };
                            let mut spotlight_items = vec![
                                SpotlightEntry {
                                    label: "Status".to_string(),
                                    any_el: Some(convert_to_pill(status.to_string(), status_color)),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Date".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_block_datetime())),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Txn Hash".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_hash())),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Block Height".to_string(),
                                    any_el: Some(convert_to_span(transaction.get_block_height())),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Canonical".to_string(),
                                    any_el: Some(
                                        convert_to_pill(
                                            transaction.get_canonical().unwrap_or_default().to_string(),
                                            ColorVariant::Grey,
                                        ),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Block State Hash".to_string(),
                                    any_el: Some(
                                        convert_to_copy_link(
                                            transaction.get_block_state_hash(),
                                            format!("/blocks/{}", transaction.get_block_state_hash()),
                                        ),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Amount".to_string(),
                                    any_el: {
                                        let amount_el = decorate_with_mina_tag(
                                            transaction.get_amount(),
                                        );
                                        Some(
                                            if transaction.get_kind() == STAKE_DELEGATION_TYPE {
                                                convert_array_to_span(
                                                    vec![
                                                        amount_el,
                                                        convert_to_tooltip(
                                                            "Stake delegations have no transacted amount".to_string(),
                                                        ),
                                                    ],
                                                )
                                            } else {
                                                amount_el
                                            },
                                        )
                                    },
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Fee".to_string(),
                                    any_el: Some(decorate_with_mina_tag(transaction.get_fee())),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "From/Fee Payer".to_string(),
                                    any_el: Some(
                                        convert_to_copy_link(
                                            transaction.get_from(),
                                            format!("/addresses/accounts/{}", transaction.get_from()),
                                        ),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "To".to_string(),
                                    any_el: Some(
                                        convert_to_copy_link(
                                            transaction.get_to(),
                                            format!("/addresses/accounts/{}", transaction.get_to()),
                                        ),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "Nonce".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_nonce(), ColorVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Memo".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_memo(), ColorVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                                SpotlightEntry {
                                    label: "Kind".to_string(),
                                    any_el: Some(
                                        convert_to_pill(transaction.get_kind(), ColorVariant::Grey),
                                    ),
                                    ..Default::default()
                                },
                            ];
                            if transaction.zkapp.is_some() {
                                spotlight_items
                                    .push(SpotlightEntry {
                                        label: "Tokens".to_string(),
                                        any_el: Some(
                                            convert_to_pill(
                                                transaction
                                                    .zkapp
                                                    .as_ref()
                                                    .map(|zkapp| {
                                                        zkapp
                                                            .accounts_updated
                                                            .iter()
                                                            .map(|au| au.token.clone())
                                                            .collect::<std::collections::HashSet<_>>()
                                                            .len()
                                                            .to_string()
                                                    })
                                                    .unwrap_or("0".to_string()),
                                                ColorVariant::Grey,
                                            ),
                                        ),
                                        ..Default::default()
                                    })
                            }
                            let table_columns: Vec<TableColumn<AnySort>> = vec![
                                TableColumn {
                                    column: "Height".to_string(),
                                    html_input_type: "number".to_string(),
                                    ..Default::default()
                                },
                                TableColumn {
                                    column: "Block State Hash".to_string(),
                                    ..Default::default()
                                },
                            ];
                            view! {
                                <SpotlightSection
                                    header="Command Spotlight"
                                    spotlight_items=spotlight_items
                                    id=Some(state_hash)
                                    meta=Some(
                                        format!(
                                            "{} ({})",
                                            convert_to_local_timezone_formatted(&date_time),
                                            print_time_since(&date_time),
                                        ),
                                    )
                                >

                                    <TransactionIcon width=40 />
                                </SpotlightSection>
                                {if transaction.zkapp.is_some() {
                                    view! {
                                        <AccountsUpdatedSection zkapp=transaction.zkapp.clone() />
                                        <TableSection
                                            metadata=metadata.into()
                                            section_heading="Actions & Events"
                                        >
                                            <SpotlightTable>

                                                <ZkAppDetailTr>
                                                    <ZkAppDetailTh>"Actions:"</ZkAppDetailTh>
                                                    <ZkAppDetailTd>
                                                        <CopyToClipboard>
                                                            <CodeBlock>
                                                                {get_actions(&txn_clone_1)
                                                                    .ok()
                                                                    .unwrap_or("Unable to serialize actions".to_string())}
                                                            </CodeBlock>
                                                        </CopyToClipboard>
                                                    </ZkAppDetailTd>
                                                </ZkAppDetailTr>
                                                <ZkAppDetailTr>
                                                    <ZkAppDetailTh>"Events:"</ZkAppDetailTh>
                                                    <ZkAppDetailTd>
                                                        <CopyToClipboard>
                                                            <CodeBlock>
                                                                {get_events(&txn_clone_2)
                                                                    .ok()
                                                                    .unwrap_or("Unable to serialize events".to_string())}
                                                            </CodeBlock>
                                                        </CopyToClipboard>
                                                    </ZkAppDetailTd>
                                                </ZkAppDetailTr>
                                            </SpotlightTable>
                                        </TableSection>
                                    }
                                        .into_view()
                                } else {
                                    ().into_view()
                                }}

                                {other_txns
                                    .get()
                                    .map(|_| {
                                        view! {
                                            <TableSectionTemplate
                                                table_columns
                                                data_sig=other_txns
                                                section_heading="In Other Blocks"
                                                is_loading=resource.loading()
                                                half_width=true
                                            />
                                        }
                                    })}
                            }
                                .into_view()
                        }
                        _ => {
                            view! {
                                <NotFound message=Some("Transaction Not Found :(".to_string()) />
                            }
                        }
                    }
                }
                Some(Err(_)) => {
                    view! { <NotFound message=Some("Transaction Not Found :(".to_string()) /> }
                }
                None => {
                    let spotlight_items = vec![
                        SpotlightEntry {
                            label: "Status".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Date".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Txn Hash".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Block Height".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Canonical".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Block State Hash".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Amount".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Fee".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "From/Fee Payer".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "To".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Nonce".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Memo".to_string(),
                            ..Default::default()
                        },
                        SpotlightEntry {
                            label: "Kind".to_string(),
                            ..Default::default()
                        },
                    ];
                    view! {
                        <SpotlightSection
                            header="Command Spotlight"
                            spotlight_items=spotlight_items
                            id=None
                            meta=None
                        >
                            <TransactionIcon width=40 />
                        </SpotlightSection>
                    }
                        .into_view()
                }
            }}

        </PageContainer>
    }
}
