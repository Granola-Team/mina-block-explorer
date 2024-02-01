use leptos::*;

use super::functions::*;
use crate::account_dialog::components::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::table::*;
use crate::icons::*;
use crate::transactions::graphql::transactions_query::TransactionsQueryTransactions;

#[component]
pub fn AccountDialogTransactionSection(
    transactions: Vec<Option<TransactionsQueryTransactions>>,
) -> impl IntoView {
    view! {
        <AccountDialogSectionContainer title=String::from("Transactions") showing_message={format!("Showing latest {} transactions", transactions.len())}>

            {transactions.into_iter()
                .map(|opt_transaction| {
                    let check_opt_trans = opt_transaction.clone();
                    let unwrap_opt_trans = opt_transaction.unwrap();
                    view! {
                        <Show when={move || check_opt_trans.is_some() }
                            fallback=move || view! { <NullView /> }
                        >
                            <TransactionEntry status=get_status(&get_block_datetime(&unwrap_opt_trans))
                                date=get_block_datetime(&unwrap_opt_trans)
                                moments_ago=print_time_since(&get_block_datetime(&unwrap_opt_trans))
                                from=get_from(&unwrap_opt_trans)
                                to=get_to(&unwrap_opt_trans)
                                fee=get_fee(&unwrap_opt_trans)
                                amount=get_amount(&unwrap_opt_trans)
                                hash=get_hash(&unwrap_opt_trans) />
                        </Show>
                    }
                })
                .collect::<Vec<_>>()
            }
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
            [a, b] => [a.clone(), b.clone()],        // For chunks of size 2
            [a] => [a.clone(), ("", String::new())], // For the last chunk of size 1, with a default/filler value
            _ => unreachable!(),                     // This case will never happen with chunks(2)
        })
        .collect();

    view! {
        <AccountDialogSectionEntryHeader date=date status=status moments_ago=moments_ago />
        {grouped.into_iter()
            .map(|e| view! {
                <div class="w-full flex justify-between">
                    {e.into_iter()
                        .map(|(label, value)| view! {
                            <AccountDialogSectionSubEntry label=label.to_string() value=value />
                        })
                        .collect::<Vec<_>>()}
                </div>
            }.into_view())
        .collect::<Vec<_>>()}

        <AccountDialogEntryDivider />

    }
}

#[component]
pub fn TransactionsSection(
    public_key: Option<String>,
    #[prop(default = None)] payment_id: Option<String>,
    #[prop(default = false)] with_link: bool,
) -> impl IntoView {
    let (pk, _set_public_key) = create_signal(public_key);
    let (pid, _set_pid) = create_signal(payment_id);

    let resource = create_resource(
        move || (pk.get(), pid.get()),
        move |(pk_value, pid_value)| async move { load_data(50, pk_value, None, pid_value).await },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Transactions".to_owned() controls=|| ().into_view()>
                    {move || match data.transactions.len() {
                        0 => view! { <EmptyTable message="This public key has no transactions".to_string() /> },
                        _ => {
                            let pag = build_pagination(data.transactions.len(), records_per_page, current_page.get(), set_current_page);
                            let subset = get_subset(&data.transactions, records_per_page, current_page.get()-1);
                            view! {
                                <Table data=subset pagination=pag/>
                                // <NullView />
                                {match with_link {
                                    false => view! { <NullView /> },
                                    true => {
                                        let pk_inner = pk.get();
                                        let link = pk_inner.map_or_else(
                                            || "/transactions".to_string(),
                                            |mpk| {
                                                if mpk.is_empty() {
                                                    "/transactions".to_string()
                                                } else {
                                                    format!("/transactions?account={}", mpk)
                                                }
                                            },
                                        );
                                        view! {
                                            <TableLink href=link text="See all transactions".to_string() >
                                                <TransactionIcon />
                                            </TableLink>
                                        }
                                    }.into_view()
                                }}
                            }
                        }.into_view()
                    }}
                </TableSection>
             },
            None => view! {
                <TableSection section_heading="Transactions".to_owned() controls=|| ().into_view()>
                    <Table data=LoadingPlaceholder{}/>
                </TableSection>
            },
            _ => view! { <span /> }.into_view()
        }}
    }
}
