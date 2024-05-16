use super::{
    graphql::account_activity_query::{AccountActivityQueryBlocks, AccountActivityQuerySnarks},
    models::{
        AccountActivityQueryDirectionalTransactionTrait,
        AccountActivityQueryDirectionalTransactions,
    },
};
use crate::{
    account_activity::table_traits::BlockTrait,
    common::{components::*, functions::*, models::*, table::*},
    icons::*,
};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn AccountDialogSectionContainer(
    #[prop(into)] title: String,
    showing_message: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-2 p-5 h-fit">
            <div class="flex justify-between items-baseline w-full mb-4">
                <h2 class="text-xl">{title}</h2>
                <span class="text-table-row-text-color text-xs flex items-center">
                    {showing_message}
                </span>
            </div>
            {children()}
        </section>
    }.into_view()
}

#[component]
pub fn AccountDialogSubsectionTable(children: Children) -> impl IntoView {
    view! { <table class="font-mono w-full">{children()}</table> }
}

#[component]
pub fn AccountDialogSubsectionRow(
    #[prop(into)] label: String,
    el: HtmlElement<html::AnyElement>,
) -> impl IntoView {
    view! {
        {match label.len() {
            0 => ().into_view(),
            _ => {
                view! {
                    <tr class="my-2 flex whitespace-nowrap">
                        <th class="text-xs text-slate-400 w-1/4 flex justify-start font-normal">
                            {label} :
                        </th>
                        <td class="text-xs overflow-hidden text-ellipsis w-[60%] flex justify-start">
                            {el}
                        </td>
                    </tr>
                }
                    .into_view()
            }
        }}
    }
}

pub struct StatusImg<'a> {
    src: &'a str,
    alt: &'a str,
}

#[component]
pub fn AccountDialogSectionEntryHeader(
    #[prop(into)] date: String,
    status: Status,
    #[prop(into)] moments_ago: String,
) -> impl IntoView {
    let img_attr = match status {
        Status::Pending => StatusImg {
            src: "/assets/img/timelapse.svg",
            alt: "Pending",
        },
        Status::Complete => StatusImg {
            src: "/assets/img/success.svg",
            alt: "Complete",
        },
        Status::Unknown => StatusImg {
            src: "",
            alt: "Unknown",
        },
    };
    view! {
        <div class="font-mono flex justify-between w-full">
            <div class="flex items-center">
                <img src=img_attr.src alt=img_attr.alt class="mr-2"/>
                {move || match status {
                    Status::Complete => {
                        view! { <span class="text-sm">{date.clone()}</span> }.into_view()
                    }
                    Status::Pending => view! { <span class="text-sm">"Pending"</span> }.into_view(),
                    Status::Unknown => view! { <span class="text-sm">"Unknown"</span> }.into_view(),
                }}

            </div>
            <div class="text-xs text-slate-400">{moments_ago}</div>
        </div>
    }
}

#[component]
pub fn AccountDialogEntryDivider() -> impl IntoView {
    view! { <div class="border-b border-slate-100 my-2 h-1 w-full"></div> }
}

#[component]
pub fn AccountDialogTransactionSection(
    transactions: Vec<Option<AccountActivityQueryDirectionalTransactions>>,
) -> impl IntoView {
    let inner_transactions = transactions.clone();
    let has_transactions = move || !transactions.clone().is_empty();
    view! {
        <AccountDialogSectionContainer
            title=String::from("User Commands")
            showing_message=format!("{} most recent transactions", inner_transactions.len())
        >
            <Show
                when=has_transactions
                fallback=move || {
                    view! { <EmptyTable message="This public key has no transactions"/> }
                }
            >

                {inner_transactions
                    .iter()
                    .map(|opt_transaction| {
                        let check_opt_trans = opt_transaction.clone();
                        let unwrap_opt_trans = opt_transaction.clone().unwrap();
                        view! {
                            <Show
                                when=move || check_opt_trans.is_some()
                                fallback=move || ().into_view()
                            >
                                <TransactionEntry
                                    status=get_status(&unwrap_opt_trans.get_date_time())
                                    date=unwrap_opt_trans.get_date_time()
                                    moments_ago=print_time_since(&unwrap_opt_trans.get_date_time())

                                    counterparty=unwrap_opt_trans.get_counterparty()
                                    direction=unwrap_opt_trans.get_direction()
                                    fee=unwrap_opt_trans.get_fee()
                                    amount=unwrap_opt_trans.get_amount()
                                    hash=unwrap_opt_trans.get_hash()
                                />
                            </Show>
                        }
                    })
                    .collect::<Vec<_>>()}

            </Show>
        </AccountDialogSectionContainer>
    }
}

#[component]
fn TransactionEntry(
    status: Status,
    #[prop(into)] date: String,
    #[prop(into)] moments_ago: String,
    #[prop(into)] direction: String,
    #[prop(into)] counterparty: String,
    #[prop(into)] fee: String,
    #[prop(into)] amount: String,
    #[prop(into)] hash: String,
) -> impl IntoView {
    let (hash_sig, _) = create_signal(hash);
    let (direction_sig, _) = create_signal(direction);
    let (counterparty_sig, _) = create_signal(counterparty);
    view! {
        <AccountDialogSectionEntryHeader date=date status=status moments_ago=moments_ago/>
        <AccountDialogSubsectionTable>
            <AccountDialogSubsectionRow
                label="Hash"
                el=convert_to_link(hash_sig.get(), format!("/commands/{}", hash_sig.get()))
            />
            <AccountDialogSubsectionRow
                label="Direction"
                el=convert_to_pill(
                    direction_sig.get(),
                    if direction_sig.get() == "OUT" {
                        ColorVariant::Blue
                    } else {
                        ColorVariant::DarkBlue
                    },
                )
            />

            <AccountDialogSubsectionRow
                label="Counterparty"
                el=convert_to_link(
                    counterparty_sig.get(),
                    format!("/addresses/accounts/{}", counterparty_sig.get()),
                )
            />

            <AccountDialogSubsectionRow
                label="Amount/Fee"
                el=convert_array_to_span(
                    vec![
                        decorate_with_currency_tag(amount, "MINA".to_string()),
                        convert_to_span(" / ".to_string()).attr("class", "whitespace-pre"),
                        decorate_with_currency_tag(fee, "MINA".to_string()),
                    ],
                )
            />

        </AccountDialogSubsectionTable>
        <AccountDialogEntryDivider/>
    }
}

#[component]
pub fn AccountTransactionsSection(
    transactions_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDirectionalTransactions>>>>,
) -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Txn Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Nonce".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Type".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Direction".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Counterparty".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Amount/Fee".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    view! {
        <TableSection
            section_heading="User Commands"
            controls=move || {
                view! {
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
        >

            <TableContainer>
                <Table>
                    <TableHeader columns=table_columns/>
                    {move || match transactions_sig.get() {
                        None => {
                            view! {
                                <TableRows data=vec![
                                    vec![LoadingPlaceholder; table_cols_length];
                                    10
                                ]/>
                            }
                        }
                        Some(transactions) => {
                            let pag = build_pagination(
                                transactions.len(),
                                records_per_page,
                                current_page.get(),
                                set_current_page,
                                None,
                                None,
                            );
                            view! {
                                <TableRows data=transactions[pag
                                        .start_index()..std::cmp::min(
                                        pag.end_index() + 1,
                                        pag.total_records,
                                    )]
                                    .to_vec()/>
                            }
                        }
                    }}

                </Table>
            </TableContainer>
            {move || {
                let length = transactions_sig.get().map(|s| s.len()).unwrap_or(0);
                let pag = build_pagination(
                    length,
                    records_per_page,
                    current_page.get(),
                    set_current_page,
                    None,
                    None,
                );
                view! { <Pagination pagination=pag/> }
            }}

        </TableSection>
    }
}

#[component]
pub fn AccountOverviewSnarkJobTable(
    snarks_sig: ReadSignal<Option<Vec<Option<AccountActivityQuerySnarks>>>>,
) -> impl IntoView {
    let memo_params_map = use_params_map();
    let (href, _set_href) = create_signal(
        memo_params_map
            .get()
            .get("id")
            .as_ref()
            .map(|pk| format!("/snarks?account={}", pk))
            .unwrap_or_else(|| "/snarks".to_string()),
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Prover".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Fee".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    view! {
        <TableContainer>
            <Table>
                <TableHeader columns=table_columns/>
                {move || match snarks_sig.get() {
                    None => {
                        view! {
                            <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                        }
                    }
                    Some(snarks) => {
                        let pag = build_pagination(
                            snarks.len(),
                            records_per_page,
                            current_page.get(),
                            set_current_page,
                            None,
                            None,
                        );
                        view! {
                            <TableRows data=snarks[pag
                                    .start_index()..std::cmp::min(
                                    pag.end_index() + 1,
                                    pag.total_records,
                                )]
                                .to_vec()/>
                        }
                    }
                }}

            </Table>
        </TableContainer>
        {move || {
            let length = snarks_sig.get().map(|s| s.len()).unwrap_or(0);
            let pag = build_pagination(
                length,
                records_per_page,
                current_page.get(),
                set_current_page,
                None,
                None,
            );
            view! { <Pagination pagination=pag/> }
        }}

        <TableLink href=href.get() text="See all snark jobs">
            <CheckCircleIcon/>
        </TableLink>
    }
}

#[component]
pub fn AccountOverviewBlocksTable(
    blocks_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryBlocks>>>>,
) -> impl IntoView {
    let memo_params_map = use_params_map();
    let (href, _set_href) = create_signal(
        memo_params_map
            .get()
            .get("id")
            .as_ref()
            .map(|pk| format!("/blocks?account={}", pk))
            .unwrap_or_else(|| "/blocks".to_string()),
    );

    let records_per_page = 5;
    let (current_page, set_current_page) = create_signal(1);

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Slot".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Block Producer".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Coinbase".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "User Commands".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Snarks".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            is_searchable: false,
        },
    ];
    let table_cols_length = table_columns.len();

    view! {
        <TableContainer>
            <Table>
                <TableHeader columns=table_columns/>
                {move || match blocks_sig.get() {
                    None => {
                        view! {
                            <TableRows data=vec![vec![LoadingPlaceholder; table_cols_length]; 10]/>
                        }
                    }
                    Some(blocks) => {
                        let pag = build_pagination(
                            blocks.len(),
                            records_per_page,
                            current_page.get(),
                            set_current_page,
                            None,
                            None,
                        );
                        let blocks_subset = blocks[pag
                                .start_index()..std::cmp::min(
                                pag.end_index() + 1,
                                pag.total_records,
                            )]
                            .to_vec();
                        view! { <TableRows data=blocks_subset/> }
                    }
                }}

            </Table>
            {move || {
                let length = blocks_sig.get().map(|s| s.len()).unwrap_or(0);
                let pag = build_pagination(
                    length,
                    records_per_page,
                    current_page.get(),
                    set_current_page,
                    None,
                    None,
                );
                view! { <Pagination pagination=pag/> }
            }}

            <TableLink href=href.get() text="See all block production">
                <BlockIcon/>
            </TableLink>
        </TableContainer>
    }
}

#[component]
pub fn AccountDialogBlocksSection(
    blocks: Vec<Option<AccountActivityQueryBlocks>>,
) -> impl IntoView {
    let blocks_inner = blocks.clone();
    let has_blocks = move || !blocks.clone().is_empty();

    view! {
        <AccountDialogSectionContainer
            title=String::from("Block Production")
            showing_message=format!("{} most recent blocks", blocks_inner.len())
        >
            <Show
                when=has_blocks
                fallback=move || {
                    view! { <EmptyTable message="This public key has no block production"/> }
                }
            >

                {blocks_inner
                    .iter()
                    .map(|opt_block| {
                        let check_block = opt_block.clone();
                        let block = opt_block.clone().unwrap();
                        view! {
                            <Show
                                when=move || check_block.is_some()
                                fallback=move || ().into_view()
                            >

                                {
                                    let moments_ago = print_time_since(&block.get_date_time());
                                    let date_time = block.get_date_time();
                                    let status = get_status(&date_time);
                                    view! {
                                        <AccountDialogSectionEntryHeader
                                            status=status
                                            date=date_time
                                            moments_ago=moments_ago
                                        />
                                        <AccountDialogBlockEntry block=block.clone()/>
                                        <AccountDialogEntryDivider/>
                                    }
                                        .into_view()
                                }

                            </Show>
                        }
                    })
                    .collect::<Vec<_>>()}
            </Show>
        </AccountDialogSectionContainer>
    }
}

struct SubEntry {
    label: String,
    value: String,
}

#[component]
fn AccountDialogBlockEntry(block: AccountActivityQueryBlocks) -> impl IntoView {
    let sub_entries = vec![
        SubEntry {
            label: String::from("Hash"),
            value: block.get_state_hash(),
        },
        SubEntry {
            label: String::from("Coinbase"),
            value: block.get_coinbase(),
        },
    ];
    view! {
        <AccountDialogSubsectionTable>
            {sub_entries
                .into_iter()
                .map(|se| {
                    view! {
                        <AccountDialogSubsectionRow
                            label=se.label
                            el=convert_to_ellipsis(se.value)
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </AccountDialogSubsectionTable>
    }
    .into_view()
}
