use leptos::*;

use super::functions::*;
use crate::accounts::components::*;
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

    view! {
        <AccountDialogSectionEntryHeader date=date status=status moments_ago=moments_ago />
        {entries.into_iter()
            .map(|(label, value)| view! {
                <AccountDialogSectionSubEntry label=label.to_string() value=value />
            })
            .collect::<Vec<_>>()}
        <AccountDialogEntryDivider />

    }
}
