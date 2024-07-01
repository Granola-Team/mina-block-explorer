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
    let mut tabs = vec![
        NavEntry {
            href: "/commands/user".to_string(),
            text: "User Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/commands/internal".to_string(),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
    ];

    if BERKELEY_FEATURES_ENABLED == "true" {
        tabs.push(NavEntry {
            href: "/commands/zk-app".to_string(),
            text: "zkApp Commands".to_string(),
            icon: NavIcon::ZKApps,
            ..Default::default()
        });
    }

    view! { <TabbedPage tabs=tabs/> }
}

#[component]
pub fn UserCommandsPage() -> impl IntoView {
    view! {
        <Title text="Commands | Search For Commands"/>
        <PageContainer>
            <TransactionsSection/>
        </PageContainer>
    }
}

#[component]
pub fn CommandSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (state_hash_sig, _) = create_query_signal::<String>("q-state-hash");
    let (txn_memo, set_txn_memo) = create_signal("No Memo".to_string());
    let resource = create_resource(
        move || (memo_params_map.get(), state_hash_sig.get()),
        |(value, state_hash)| async move {
            let txn_hash = value.get("id");
            load_data(10, None, None, txn_hash.cloned(), None, state_hash, None).await
        },
    );

    create_effect(move |_| {
        if let Some(Ok(data)) = resource.get() {
            if let Some(Some(txn)) = data.transactions.first() {
                set_txn_memo.set(txn.get_memo());
            }
        }
    });

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            html_input_type: "number".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Hash".to_string(),
            ..Default::default()
        },
    ];

    view! {
        <Title
            formatter=move |text| format!("Transaction Overview | {text}")
            text=move || txn_memo.get()
        />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(data)) => {
                    match data.transactions.first() {
                        Some(Some(transaction)) => {
                            let state_hash = transaction.get_hash();
                            let date_time = transaction.get_block_datetime();
                            let spotlight_items = vec![
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
                                        convert_to_link(
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
                                    label: "From".to_string(),
                                    any_el: Some(
                                        convert_to_link(
                                            transaction.get_from(),
                                            format!("/addresses/accounts/{}", transaction.get_from()),
                                        ),
                                    ),
                                    copiable: true,
                                },
                                SpotlightEntry {
                                    label: "To".to_string(),
                                    any_el: Some(
                                        convert_to_link(
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
                            view! {
                                <SpotlightSection
                                    header="Command Spotlight"
                                    spotlight_items=spotlight_items
                                    id=Some(state_hash)
                                    meta=Some(
                                        format!("{} ({})", date_time, print_time_since(&date_time)),
                                    )
                                >

                                    <TransactionIcon width=40/>
                                </SpotlightSection>
                            }
                                .into_view()
                        }
                        _ => {
                            view! {
                                <NotFound message=Some("Transaction Not Found :(".to_string())/>
                            }
                        }
                    }
                }
                None => {
                    let spotlight_items = vec![
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
                            label: "From".to_string(),
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
                            <TransactionIcon width=40/>
                        </SpotlightSection>
                    }
                        .into_view()
                }
                _ => ().into_view(),
            }}
            <AppSection>
                <AppHeading heading="In Other Blocks"/>
                <div class="mx-auto @container w-full lg:w-1/2 flex justify-center">
                    <Table>
                        <ColGroup columns=table_columns.clone()/>
                        <TableHeader columns=table_columns/>

                        <tbody>
                            {move || {
                                resource
                                    .get()
                                    .and_then(|res| res.ok())
                                    .map(|data| {
                                        data.other_transactions
                                            .into_iter()
                                            .filter(|txn_opt| {
                                                txn_opt
                                                    .clone()
                                                    .map(|txn| {
                                                        txn.get_block_state_hash()
                                                            != state_hash_sig.get().unwrap_or_default()
                                                    })
                                                    .unwrap_or_default()
                                            })
                                            .map(|txn_opt| {
                                                txn_opt
                                                    .map(|transaction| {
                                                        let transaction_clone = transaction.clone();
                                                        view! {
                                                            <tr>
                                                                <TableCell column_opt=None>
                                                                    {convert_to_span(transaction.get_block_height())}
                                                                </TableCell>
                                                                <TableCell column_opt=None>
                                                                    {if !transaction_clone.get_memo().is_empty() {
                                                                        convert_array_to_span(
                                                                                vec![
                                                                                    convert_to_link(
                                                                                        transaction_clone.get_hash(),
                                                                                        format!(
                                                                                            "/commands/{}?q-state-hash={}",
                                                                                            transaction_clone.get_hash(),
                                                                                            transaction_clone.get_block_state_hash(),
                                                                                        ),
                                                                                    ),
                                                                                    convert_to_span(transaction_clone.get_memo())
                                                                                        .attr("class", "block text-xs font-light text-slate-400"),
                                                                                ],
                                                                            )
                                                                            .attr("class", "block")
                                                                    } else {
                                                                        convert_to_link(
                                                                            transaction_clone.get_hash(),
                                                                            format!(
                                                                                "/commands/{}?q-state-hash={}",
                                                                                transaction_clone.get_hash(),
                                                                                transaction_clone.get_block_state_hash(),
                                                                            ),
                                                                        )
                                                                    }}

                                                                </TableCell>
                                                            </tr>
                                                        }
                                                    })
                                            })
                                            .collect_view()
                                    })
                                    .collect_view()
                            }}

                        </tbody>
                    </Table>
                </div>
            </AppSection>
        </PageContainer>
    }
}
