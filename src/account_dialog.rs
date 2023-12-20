use leptos::*;
use leptos_router::*;
use web_sys::console;


use chrono::{DateTime, Utc, Duration};
use crate::account_page::{AccountSummary, load_data as load_account_summary};
use crate::transactions_page::{TransactionsResponse, Data as TransactionData, Transaction, load_data as load_transaction_data};

enum Status {
    Pending,
    Complete
}

// Function to format the duration in a human-readable way
fn format_duration(duration: &Duration) -> String {
    let total_minutes = duration.num_minutes();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{} hour(s) and {} minute(s) ago", hours, minutes)
}

// Function to calculate and print the time elapsed since the given timestamp
fn print_time_since(timestamp: &str) -> String {
    // Parse the input timestamp
    let past_time = match timestamp.parse::<DateTime<Utc>>() {
        Ok(time) => time,
        Err(e) => {
            Utc::now()
        }
    };

    // Get the current time
    let now = Utc::now();

    // Calculate the duration since the given timestamp
    let duration_since = now.signed_duration_since(past_time);

    // Format and print the duration
    format_duration(&duration_since)
}

pub fn AccountDialogView() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = memo_params_map.with(|params| params.get("id").cloned()).unwrap_or_default();
    let id_for_other = id.clone();

    let account_resource = create_resource(|| (), move |_| {
        let id_clone_for_async = id.clone(); // Clone the ID for the async block
        async move { 
            load_account_summary(&id_clone_for_async).await
        }
    });
    
    let trans_resource = create_resource(|| (), move |_| {
        let id_clone_for_async = id_for_other.clone(); // Clone the ID for the async block
        async move { 
            let limit = 3;
            load_transaction_data(limit, Some(id_clone_for_async)).await 
        }
    });

    view! {
        {move || match (account_resource.get(), trans_resource.get()) {
            (Some(Ok(a_res)), Some(Ok(t_res))) => view!{
                <AccountDialog account=a_res.account transactions=t_res.data.transactions />
            },
            _ => view! { <span/>   }.into_view()
        }}
    }
}

#[component]
fn AccountDialog(account: AccountSummary, transactions: Vec<Transaction>) -> impl IntoView {
    let summary_items = vec![
        ("Balance", account.balance.total ,true),
        ("Nonce", account.nonce.to_string(),true),
        (
            "Receipt Chain Hash",
            account.receipt_chain_hash,
            false
        ),
        (
            "Delegate",
            account.delegate,
            false
        ),
        (
            "Voting For",
            account.voting_for,
            false
        ),
    ];
   

    view! {
        <dialog id="accountdialog" class="w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <section>
                <div class="flex justify-between">
                    <h2 class="text-bold text-xl">"Account Overview"</h2>
                    <button>X</button>
                </div>
                <div class="flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
                    <div class="w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%]">
                        <img src="/assets/img/account_balance_wallet.svg" alt="account balance wallet logo"/>
                    </div>
                    <div class="text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden">
                        {account.public_key}
                    </div>
                    <div class="text-slate-400 text-sm">
                        "Username: "{account.username}
                    </div>
                </div>
                <div class="bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
                    {summary_items.into_iter()
                        .map(|(label, value, has_pill)| view! {
                            <OverviewEntry label=label.to_owned() value=value.to_owned() has_pill=has_pill />
                        })
                        .collect::<Vec<_>>()}

                </div>
            </section>
            <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-8 p-4 h-[100%]">
                <div class="flex justify-between w-full">
                    <h2 class="text-xl">"Transactions"</h2>
                    <span class="text-table-row-text-color text-xs">"Showing 5 of 110"</span>
                </div>
                <div class="flex flex-col md:flex-row md:flex-wrap overflow-y-auto">
                    {transactions.into_iter()
                        .map(|transaction| view! {
                            <Transaction status=Status::Complete
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
        </dialog>
    }
}

#[component]
fn Transaction(status: Status, date:String, moments_ago:String, from:String, to:String, fee:String, amount:String, hash:String) -> impl IntoView {

    let img_attr = match status {
        Status::Pending => ("/assets/img/timelapse.svg","Pending"),
        Status::Complete => ("/assets/img/down-arrow.svg","Complete")
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
                <span class="text-sm">{img_attr.1}</span>
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

#[component]
fn OverviewEntry(label: String, value: String, has_pill: bool) -> impl IntoView {
    let value_class_str_base = "py-1 my-1 text-sm";

    let value_class_str = match has_pill {
        true => format!("{} {}",value_class_str_base.to_owned(),"p-1 rounded-full bg-light-granola-orange"),
        false => format!("{} {}",value_class_str_base.to_owned(),"w-3/4 text-ellipsis overflow-hidden"),
    };

    view! {
        <div class="flex flex-col items-start md:flex-row md:items-baseline md:justify-start">
            <span class="w-1/4 text-slate-400 text-sm whitespace-nowrap">{label}:</span>
            <span class=value_class_str>{value}</span>
        </div>
    }
}
