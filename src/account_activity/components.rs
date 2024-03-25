use super::models::{
    AccountActivityQueryDirectionalTransactionTrait, AccountActivityQueryDirectionalTransactions,
};
use crate::{common::{components::*, functions::*, models::*, table::EmptyTable}, account_activity::functions::load_data};
use leptos::*;
use leptos_router::create_query_signal;
use crate::common::table::*;

#[component]
pub fn AccountDialogSectionContainer(
    title: String,
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
            0 => view! { <NullView/> }.into_view(),
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
    date: String,
    status: Status,
    moments_ago: String,
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
            title=String::from("Transactions")
            showing_message=format!("Showing latest {} transactions", inner_transactions.len())
        >
            <Show
                when=has_transactions
                fallback=move || {
                    view! {
                        <EmptyTable message="This public key has no transactions".to_string()/>
                    }
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
                                fallback=move || view! { <NullView/> }
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
    date: String,
    moments_ago: String,
    direction: String,
    counterparty: String,
    fee: String,
    amount: String,
    hash: String,
) -> impl IntoView {
    let (hash_sig, _) = create_signal(hash);
    let (direction_sig, _) = create_signal(direction);
    let (counterparty_sig, _) = create_signal(counterparty);
    view! {
        <AccountDialogSectionEntryHeader date=date status=status moments_ago=moments_ago/>
        <AccountDialogSubsectionTable>
            <AccountDialogSubsectionRow
                label="Hash"
                el=convert_to_link(hash_sig.get(), format!("/transactions/{}", hash_sig.get()))
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
                        wrap_in_pill(
                            decorate_with_currency_tag(amount, "MINA".to_string()),
                            ColorVariant::Green,
                        ),
                        convert_to_span(" / ".to_string()).attr("class", "whitespace-pre"),
                        wrap_in_pill(
                            decorate_with_currency_tag(fee, "MINA".to_string()),
                            ColorVariant::Orange,
                        ),
                    ],
                )
            />

        </AccountDialogSubsectionTable>
        <AccountDialogEntryDivider/>
    }
}

#[component]
pub fn AccountTransactionsSection(public_key: Option<String>) -> impl IntoView {
    let (pk, _set_public_key) = create_signal(public_key);
    let (transactions, set_transactions) = create_signal(None);
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    let account_activity_resource = create_resource(
        move || (pk.get(), canonical_qp.get()),
        |(id_opt, canonical)| async move {
            if let Some(id) = id_opt {
                load_data(
                    Some(id),
                    None,
                    None,
                    Some(100),
                    Some(canonical.unwrap_or(true)),
                )
                .await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    create_effect(move |_| {
        if let Some(res) = account_activity_resource.get().and_then(|res| res.ok()) {
            let mut transactions: Vec<_> = res
                .incoming_transactions
                .into_iter()
                .filter(|t| t.is_some())
                .map(|r| r.map(|t| t.into()))
                .chain(
                    res.outgoing_transactions
                        .into_iter()
                        .filter(|t| t.is_some())
                        .map(|r| r.map(|t| t.into())),
                )
                .collect();
            transactions.sort_by(|a, b| {
                match (
                        <std::option::Option<
                            AccountActivityQueryDirectionalTransactions,
                        > as Clone>::clone(a)
                            .unwrap()
                            .date_time,
                        <std::option::Option<
                            AccountActivityQueryDirectionalTransactions,
                        > as Clone>::clone(b)
                            .unwrap()
                            .date_time,
                    ) {
                        (Some(date_time_a), Some(date_time_b)) => {
                            date_time_b.cmp(&date_time_a)
                        }
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
            });
            set_transactions.set(Some(transactions));
        };
    });

    view! {
        <ErrorBoundary fallback=move |_| {
            view! { <NullView/> }
        }>

            {move || match transactions.get() {
                Some(data) => {
                    view! {
                        <TableSection
                            section_heading="Transactions".to_owned()
                            controls=move || {
                                view! {
                                    <BooleanUrlParamSelectMenu
                                        id="canonical-selection"
                                        query_str_key="canonical"
                                        labels=BooleanUrlParamSelectOptions {
                                            true_case: String::from("Canonical"),
                                            false_case: String::from("Non-Canonical"),
                                        }
                                    />
                                }
                            }
                        >

                            {move || {
                                let pag = build_pagination(
                                    data.len(),
                                    records_per_page,
                                    current_page.get(),
                                    set_current_page,
                                );
                                let subset = get_subset(
                                    &data,
                                    records_per_page,
                                    current_page.get() - 1,
                                );
                                view! { <Table data=subset pagination=pag/> }
                            }}

                        </TableSection>
                    }
                }
                None => {
                    view! {
                        <TableSection
                            section_heading="Transactions".to_string()
                            controls=|| ().into_view()
                        >
                            <Table data=LoadingPlaceholder {}/>
                        </TableSection>
                    }
                }
            }}

        </ErrorBoundary>
    }
}
