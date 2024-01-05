use leptos::*;

use super::functions::*;
use crate::accounts::components::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;

#[component]
pub fn AccountDialogTransactionSection(limit: i32, account_id: String) -> impl IntoView {
    let resource = create_resource(
        || (),
        move |_| {
            let account_id_clone = account_id.clone();
            async move { load_data(limit, Some(account_id_clone)).await }
        },
    );

    view! {
        {move || match resource.get() {
            Some(Ok(res)) => view! {
                <AccountDialogSectionContainer title=String::from("Transactions") showing_message={format!("Showing latest {} transactions", res.transactions.len())}>
                    {res.transactions.into_iter()
                        .map(|opt_transaction| {
                            match opt_transaction {
                                Some(transaction) => view! {
                                    <TransactionEntry status=get_status(&get_block_datetime(&transaction))
                                        date=get_block_datetime(&transaction)
                                        moments_ago=print_time_since(&get_block_datetime(&transaction))
                                        from=get_from(&transaction)
                                        to=get_to(&transaction)
                                        fee=get_fee(&transaction)
                                        amount=get_amount(&transaction)
                                        hash=get_hash(&transaction) />
                                },
                                None => view! { <span /> }.into_view()
                            }
                        })
                        .collect::<Vec<_>>()}
                </AccountDialogSectionContainer>
            },
            _ => view! { <span /> }.into_view()

        }}

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
    #[prop(default = false)] with_link: bool,
) -> impl IntoView {
    let (pk, _set_public_key) = create_signal(public_key.unwrap_or_default());

    let resource = create_resource(move || pk.get(), move |value| {
            async move {
                let limit = 10;
                load_data(limit, Some(value)).await
            }
        }
    );

    view! {
        {move || match resource.get() {
            Some(Ok(data)) => view! {
                <TableSection section_heading="Transactions".to_owned()>
                    {match data.transactions.len() {
                        0 => view! { <EmptyTable message="This public key has no transactions".to_string() /> },
                        _ => view! { 
                            <Table data=data.transactions/>
                            {match with_link {
                                false => view! {<div />}.into_view(),
                                true => {
                                    let pk_inner = pk.get();
                                    let link = match pk_inner.len() { 
                                        0 => "/transactions".to_string(),
                                        _ => format!("/transactions?account={}", pk_inner)
                                    };
                                    view! {<TableLink href=link text="See all transactions".to_string() />}
                                }.into_view()
                            }}
                        }.into_view()
                    }}
                </TableSection>
             },
            _ => view! { <span /> }.into_view()
        }}
    }
}
