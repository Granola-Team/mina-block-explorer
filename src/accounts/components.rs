use leptos::*;

use crate::transactions_page::Transaction;

use super::functions::*;
use super::models::*;

#[component]
pub fn AccountDialog(path_base: String, account: AccountSummary, transactions: Vec<Transaction>) -> impl IntoView {
    // let id = account.public_key.clone();
    let summary_items = vec![
        (String::from("Balance"), account.balance.total ,true),
        (String::from("Nonce"), account.nonce.to_string(),true),
        (
            String::from("Receipt Chain Hash"),
            account.receipt_chain_hash,
            false
        ),
        (
            String::from("Delegate"),
            account.delegate,
            false
        ),
        (
            String::from("Voting For"),
            account.voting_for,
            false
        ),
    ];
   

    view! {
        <dialog id="accountdialog" class="w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <section>
                <div class="flex justify-between">
                    <h2 class="text-bold text-xl">"Account Overview"</h2>
                    <button>
                        <a href=path_base>X</a>
                    </button>
                </div>
                <AccountSummarySubsection summary_items=summary_items public_key=account.public_key username=account.username />
            </section>
            <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-8 p-4 h-[100%]">
                <div class="flex justify-between w-full">
                    <h2 class="text-xl">"Transactions"</h2>
                    <span class="text-table-row-text-color text-xs">{format!("Showing latest {} transactions", transactions.len())}</span>
                </div>
                <TransactionsSubsection transactions=transactions />
            </section>
            <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                <button disabled class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg">
                    // <a href={format!("/accounts/{}", id)}>"View all details"</a>
                    "View all details"
                </button>
            </div>
        </dialog>
    }
}

#[component]
pub fn TransactionsSubsection(transactions: Vec<Transaction>) -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row md:flex-wrap overflow-y-auto">
            {transactions.into_iter()
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
    }
}

#[component]
pub fn AccountSummarySubsection(summary_items: Vec<(String, String, bool)>, username: String, public_key: String) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div class="w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%]">
                <img src="/assets/img/account_balance_wallet.svg" alt="account balance wallet logo"/>
            </div>
            <div class="text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden">
                {public_key}
            </div>
            <div class="text-slate-400 text-sm">
                "Username: "{username}
            </div>
        </div>
        <div class="bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
            {summary_items.into_iter()
                .map(|(label, value, has_pill)| view! {
                    <OverviewEntry label=label.to_owned() value=value.to_owned() has_pill=has_pill />
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn TransactionEntry(status: Status, date:String, moments_ago:String, from:String, to:String, fee:String, amount:String, hash:String) -> impl IntoView {

    let img_attr = match status {
        Status::Pending => ("/assets/img/timelapse.svg","Pending"),
        Status::Complete => ("/assets/img/down-arrow.svg","Complete"),
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