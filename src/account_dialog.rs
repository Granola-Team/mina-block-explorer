use leptos::*;
use leptos_router::*;

enum Status {
    Pending,
    Complete
}

#[component]
pub fn AccountDialog() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = memo_params_map.with(|params| params.get("id").cloned()).unwrap_or_default();

    let summary_items = vec![
        ("Balance", "96891652.921500000",true),
        ("Nonce", "15",true),
        (
            "Receipt Chain Hash",
            "2n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XFG",
            false
        ),
        (
            "Delegate",
            "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
            false
        ),
        (
            "Voting For",
            "5GK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyP8c",
            false
        ),
    ];

    let transactions = vec![
        (Status::Pending, "2023-12-15 06:54:00", "29 minutes ago", "2n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XFG", "3n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XHI", "0.300000000", "4970037.516000000", "5PM2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE5y"),
        (Status::Complete, "2023-12-15 06:57:00", "32 minutes ago", "2n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XFG", "3n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XHI", "0.300000000", "4970037.516000000", "5PM2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE5y"),
        (Status::Complete, "2023-12-15 06:51:00", "38 minutes ago", "2n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XFG", "3n1YWuNHnjj6Y8C9SC1viqxXBLz99Mxks58VduA2X7uusZeS3XHI", "0.300000000", "4970037.516000000", "5PM2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE5y"),
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
                        {id}
                    </div>
                    <div class="text-slate-400 text-sm">
                        "Username: Aura Wallet"
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
                        .map(|(status, date, moments_ago, from, to, fee, amount, hash)| view! {
                            <Transaction status=status
                                date=date.to_owned()
                                moments_ago=moments_ago.to_owned()
                                from=from.to_owned()
                                to=to.to_owned()
                                fee=fee.to_owned()
                                amount=amount.to_owned()
                                hash=hash.to_owned() />
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
