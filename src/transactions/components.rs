use super::{
    functions::*, models::DirectionalTransactionsQueryTransactions, table_trait::TransactionsTrait,
};
use crate::{
    account_dialog::components::*,
    common::{components::*, functions::*, models::*, table::*},
    icons::*,
    transactions::graphql::transactions_query::TransactionsQueryTransactions,
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn AccountDialogTransactionSection(
    transactions: Vec<Option<TransactionsQueryTransactions>>,
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
                                    status=get_status(&unwrap_opt_trans.get_block_datetime())
                                    date=unwrap_opt_trans.get_block_datetime()
                                    moments_ago=print_time_since(
                                        &unwrap_opt_trans.get_block_datetime(),
                                    )

                                    from=unwrap_opt_trans.get_from()
                                    to=unwrap_opt_trans.get_to()
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
    from: String,
    to: String,
    fee: String,
    amount: String,
    hash: String,
) -> impl IntoView {
    let entries = vec![
        ("From", from),
        ("To", to),
        ("Fee", fee),
        ("Amount", amount),
        ("Hash", hash),
    ];

    let grouped: Vec<[(&str, String); 2]> = entries
        .chunks(2)
        .map(|chunk| match chunk {
            [a, b] => [a.clone(), b.clone()], // For chunks of size 2
            /* For the last chunk of size 1, with a default/filler value */
            [a] => [a.clone(), ("", String::new())],
            _ => unreachable!(), // This case will never happen with chunks(2)
        })
        .collect();

    view! {
        <AccountDialogSectionEntryHeader date=date status=status moments_ago=moments_ago/>
        <AccountDialogSubsectionTable>
            {grouped
                .into_iter()
                .map(|e| {
                    view! {
                        {e
                            .into_iter()
                            .map(|(label, value)| {
                                view! {
                                    <AccountDialogSubsectionRow
                                        label=label.to_string()
                                        value=value
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}
                    }
                        .into_view()
                })
                .collect::<Vec<_>>()}
        </AccountDialogSubsectionTable>
        <AccountDialogEntryDivider/>
    }
}

#[component]
pub fn TransactionsSection(
    #[prop(default = None)] state_hash: Option<String>,
    #[prop(default = false)] with_link: bool,
) -> impl IntoView {
    let (state_hash_sig, _) = create_signal(state_hash);
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");

    let resource = create_resource(
        move || (state_hash_sig.get(), canonical_qp.get()),
        move |(state_hash, canonical)| async move {
            load_data(50, None, None, state_hash, canonical).await
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => {
                view! {
                    <TransactionSection
                        transactions=Some(data.transactions)
                        with_link
                        public_key=None
                    />
                }
            }
            _ => {
                view! {
                    <TableSection
                        section_heading="Transactions".to_owned()
                        controls=|| ().into_view()
                    >
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        }}
    }
}

#[component]
pub fn AccountTransactionsSection(
    public_key: Option<String>,
    #[prop(default = None)] state_hash: Option<String>,
) -> impl IntoView {
    let (pk, _set_public_key) = create_signal(public_key);
    let (state_hash_sig, _) = create_signal(state_hash);
    let (canonical_qp, _) = create_query_signal::<bool>("canonical");
    let (data, set_data) = create_signal(None);

    let transactions_from_resource = create_resource(
        move || (pk.get(), state_hash_sig.get(), canonical_qp.get()),
        move |(pk_value, state_hash, canonical)| async move {
            logging::log!("create_resource");
            load_data(50, pk_value, None, state_hash, canonical).await
        },
    );

    let transactions_to_resource = create_resource(
        move || (pk.get(), state_hash_sig.get(), canonical_qp.get()),
        move |(pk_value, state_hash, canonical)| async move {
            load_data(50, None, pk_value, state_hash, canonical).await
        },
    );

    create_effect(move |_| {
        if let (Some(Ok(data_from)), Some(Ok(data_to))) = (
            transactions_from_resource.get(),
            transactions_to_resource.get(),
        ) {
            let mut data = data_from
                .transactions
                .iter()
                .filter(|d| d.is_some())
                .chain(data_to.transactions.iter())
                .map(|d| {
                    let trx = d.clone().unwrap();
                    Some(DirectionalTransactionsQueryTransactions::from_original(
                        &trx,
                        pk.get().unwrap(),
                    ))
                })
                .collect::<Vec<_>>();
            data.sort_by(|a, b| {
                        match (&<std::option::Option<DirectionalTransactionsQueryTransactions> as Clone>::clone(a).unwrap().base_transaction.block.unwrap().date_time, &<std::option::Option<DirectionalTransactionsQueryTransactions> as Clone>::clone(b).unwrap().base_transaction.block.unwrap().date_time) {
                            (Some(date_time_a), Some(date_time_b)) => date_time_b.cmp(date_time_a),
                            (Some(_), None) => std::cmp::Ordering::Greater,
                            (None, Some(_)) => std::cmp::Ordering::Less,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    });
            set_data.set(Some(data));
        }
    });

    {
        move || {
            view! { <TransactionSection transactions=data.get() public_key=pk.get()/> }
        }
    }
}

#[component]
fn TransactionSection<T>(
    public_key: Option<String>,
    #[prop(default = false)] with_link: bool,
    transactions: Option<Vec<Option<T>>>,
) -> impl IntoView
where
    T: TransactionsTrait + Clone + 'static,
    Vec<Option<T>>: TableData,
{
    let (pk, _set_public_key) = create_signal(public_key);
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    let transactions_show_condition = transactions.clone();
    let transactions_inner = transactions.clone();
    view! {
        <Show
            when=move || transactions_show_condition.is_some()
            fallback=move || {
                view! {
                    <TableSection
                        section_heading="Transactions".to_owned()
                        controls=|| ().into_view()
                    >
                        <Table data=LoadingPlaceholder {}/>
                    </TableSection>
                }
            }
        >

            {
                let transactions_for_empty = transactions_inner.clone().unwrap();
                let transactions_inner = transactions_inner.clone().unwrap();
                view! {
                    <Show
                        when=move || { !transactions_for_empty.is_empty() }
                        fallback=move || view! { <EmptyTable message="No transactions found"/> }
                    >

                        {
                            let data = transactions_inner.clone();
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
                                        view! {
                                            <Table data=subset pagination=pag/>
                                            <Show
                                                when=move || pk.get().is_some() && with_link
                                                fallback=move || view! { <NullView/> }
                                            >

                                                {
                                                    let pk_i = pk.get().unwrap();
                                                    view! {
                                                        <TableLink
                                                            href=format!("/transactions?account={}", pk_i)
                                                            text="See all transactions".to_string()
                                                        >
                                                            <TransactionIcon/>
                                                        </TableLink>
                                                    }
                                                }

                                            </Show>
                                        }
                                    }}

                                </TableSection>
                            }
                        }

                    </Show>
                }
            }

        </Show>
    }
}
