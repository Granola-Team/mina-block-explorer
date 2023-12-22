use leptos::*;

use super::{functions::*, models::*};

#[component]
pub fn TransactionsSubsection(limit: i32, account_id: String) -> impl IntoView {
    let resource = create_resource(|| (), move |_| {
        let account_id_clone = account_id.clone(); 
        async move { 
            load_data(limit, Some(account_id_clone)).await 
        }
    });

    view! {
        {move || match resource.get() {
            Some(Ok(res)) => view! {
                <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-8 p-4 h-[100%]">
                    <div class="flex justify-between w-full">
                        <h2 class="text-xl">"Transactions"</h2>
                        <span class="text-table-row-text-color text-xs">{format!("Showing latest {} transactions", res.data.transactions.len())}</span>
                    </div>
                    <div class="flex flex-col md:flex-row md:flex-wrap overflow-y-auto">
                        {res.data.transactions.into_iter()
                            .map(|transaction| view! {
                                <TransactionEntry status=get_status(&transaction.block.date_time)
                                    date=transaction.block.date_time.to_owned()
                                    moments_ago=print_time_since(&transaction.block.date_time)
                                    from=transaction.from.to_owned()
                                    to=transaction.to.to_owned()
                                    fee=transaction.fee.to_string()
                                    amount=transaction.amount.to_string()
                                    hash=transaction.hash.to_owned() />
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </section>
            }.into_view(),
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
