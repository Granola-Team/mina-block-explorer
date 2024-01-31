use leptos::*;

use super::functions::*;
use crate::account_dialog::components::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::table::*;
use crate::icons::*;

use crate::account_dialog::graphql::account_activity_query::AccountActivityQueryTransactions;


#[component]
pub fn AccountDialogTransactionSection(transactions: Vec<Option<AccountActivityQueryTransactions>>) -> impl IntoView {

    view! {
        <AccountDialogSectionContainer title=String::from("Transactions") showing_message={format!("Showing latest {} transactions", transactions.len())}>
            {transactions.into_iter()
                .map(|opt_transaction| {
                    match opt_transaction {
                        Some(transaction) => view! {
                            <TransactionEntry status=get_status(&get_block_datetime_for_account_activity(&transaction))
                                date=get_block_datetime_for_account_activity(&transaction)
                                moments_ago=print_time_since(&get_block_datetime_for_account_activity(&transaction))
                                from=get_from_for_account_activity(&transaction)
                                to=get_to_for_account_activity(&transaction)
                                fee=get_fee_for_account_activity(&transaction)
                                amount=get_amount_for_account_activity(&transaction)
                                hash=get_hash_for_account_activity(&transaction) />
                        },
                        None => view! { <span /> }.into_view()
                    }
                })
                .collect::<Vec<_>>()}
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
