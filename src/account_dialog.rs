use leptos::*;

// use crate::account_page::AccountSummary;

#[component]
pub fn AccountDialog() -> impl IntoView {
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

    view! {
        <dialog id="accountdialog" class="w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <section>
                <div class="flex justify-between">
                    <h2 class="text-bold text-xl">"Account Overview"</h2>
                    <button>X</button>
                </div>
                <div class="flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
                    <div class="w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%]">
                        <img src="assets/img/account_balance_wallet.svg" alt="account balance wallet logo"/>
                    </div>
                    <div class="text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden">
                        "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9"
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
            
        </dialog>
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
