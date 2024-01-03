use leptos::*;

use crate::accounts::components::AccountDialogSectionContainer;

use super::{functions::*, models::*};

#[component]
pub fn AccountDialogTransactionSection(limit: i32, account_id: String) -> impl IntoView {
    let resource = create_resource(|| (), move |_| {
        let account_id_clone = account_id.clone(); 
        async move { 
            load_data(limit, Some(account_id_clone)).await 
        }
    });

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
fn TransactionEntry(status: Status, date:String, moments_ago:String, from:String, to:String, fee:String, amount:String, hash:String) -> impl IntoView {

    let img_attr = match status {
        Status::Pending => ("/img/timelapse.svg","Pending"),
        Status::Complete => ("/img/down-arrow.svg","Complete"),
        Status::Unknown => ("","Unknown")
    };

    let entries = vec![
        ("From", from),
        ("To", to),
        ("Fee", fee),
        ("Amount", amount),
        ("Hash", hash)
    ];

    view! {
        <div class="flex justify-between w-full">
            <div class="flex items-center">
                <img src=img_attr.0 alt=img_attr.1 />
                {move || match status {
                    Status::Complete => view! {<span class="text-sm">{date.clone()}</span>}.into_view(),
                    Status::Pending => view! {<span class="text-sm">"Pending"</span>}.into_view(),
                    Status::Unknown => view! {<span class="text-sm">"Unkonwn"</span>}.into_view(),
                }}
                
            </div>
            <div class="text-xs text-slate-400">{moments_ago}</div>
        </div>
        {entries.into_iter()
            .map(|(label, value)| view! {
                <div class="w-full md:w-1/2 flex my-1">
                    <span class="text-xs text-slate-400 w-1/4">{label}:</span>
                    <span class="text-xs overflow-hidden text-ellipsis w-3/4">{value}</span>
                </div>        
            })
            .collect::<Vec<_>>()}
        <div class="border-b border-slate-100 my-2 h-1 w-full" />
        
    }
}
