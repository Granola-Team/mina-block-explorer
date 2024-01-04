use leptos::*;

use super::functions::*;
use super::models::*;
use crate::common::models::*;
use crate::snarks::components::AccountDialogSnarkJobSection;
use crate::transactions::components::AccountDialogTransactionSection;

#[component]
pub fn AccountDialog(path_base: String, account: AccountSummary) -> impl IntoView {
    let id = account.public_key.clone();
    let summary_items = get_summary_items(account.clone());
    let public_key = account.public_key.clone();

    view! {
        <dialog id="accountdialog" class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <section>
                <div class="flex justify-between">
                    <h2 class="text-bold text-xl">"Account Overview"</h2>
                    <button id="closedialog">
                        <a href=path_base>X</a>
                    </button>
                </div>
                <AccountSummarySubsection summary_items=summary_items public_key=account.public_key username=account.username />
            </section>
            <div class="overflow-y-auto flex flex-col pb-20">
                <AccountDialogTransactionSection limit=3 account_id=public_key.clone() />
                <AccountDialogSnarkJobSection public_key=Some(public_key)/>
            </div>
            <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                <button id="viewmore" class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg">
                    <a href={format!("/accounts/{}", id)}>"View all details"</a>
                </button>
            </div>
        </dialog>
    }.into_view()
}

#[component]
pub fn AccountDialogSectionContainer(
    title: String,
    showing_message: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="flex flex-col bg-white rounded-xl flex flex-col items-stretch mt-8 p-4 h-fit">
            <div class="flex justify-between w-full mb-4">
                <h2 class="text-xl">{title}</h2>
                <span class="text-table-row-text-color text-xs">{showing_message}</span>
            </div>
            {children()}
        </section>
    }.into_view()
}

#[component]
pub fn AccountDialogSectionSubEntry(label: String, value: String) -> impl IntoView {
    match label.len() {
        0 => view! {<div />},
        _ => view! {
            <div class="w-full md:w-1/2 flex my-1">
                <span class="text-xs text-slate-400 w-1/4">{label}:</span>
                <span class="text-xs overflow-hidden text-ellipsis w-3/4">{value}</span>
            </div>
        }
    }
}

pub struct StatusImg<'a> {
    src: &'a str,
    alt: &'a str,
}

#[component]
pub fn AccountDialogSectionEntryHeader(
    date: String,
    status: Status,
    moments_ago: String,
) -> impl IntoView {
    let img_attr = match status {
        Status::Pending => StatusImg {
            src: "/img/timelapse.svg",
            alt: "Pending",
        },
        Status::Complete => StatusImg {
            src: "/img/success.svg",
            alt: "Complete",
        },
        Status::Unknown => StatusImg {
            src: "",
            alt: "Unknown",
        },
    };
    view! {
        <div class="flex justify-between w-full">
            <div class="flex items-center">
                <img src=img_attr.src alt=img_attr.alt class="mr-2"/>
                {move || match status {
                    Status::Complete => view! {<span class="text-sm">{date.clone()}</span>}.into_view(),
                    Status::Pending => view! {<span class="text-sm">"Pending"</span>}.into_view(),
                    Status::Unknown => view! {<span class="text-sm">"Unknown"</span>}.into_view(),
                }}
            </div>
            <div class="text-xs text-slate-400">{moments_ago}</div>
        </div>
    }
}

#[component]
pub fn AccountDialogEntryDivider() -> impl IntoView {
    view! {
        <div class="border-b border-slate-100 my-2 h-1 w-full" />
    }
}

#[component]
pub fn AccountSummarySubsection(
    summary_items: Vec<(String, String, bool)>,
    username: String,
    public_key: String,
) -> impl IntoView {
    view! {
        <div class="@lg:grid @lg:grid-cols-[10rem_5rem_auto_10rem] @lg:grid-rows-[2.5rem_2.5rem] @lg:gap-x-[2rem] @lg:h-auto flex flex-col items-center mt-16 bg-light-granola-orange rounded-3xl h-36">
            <div class="@lg:col-start-2 @lg:col-end-3 @lg:row-start-1 @lg:row-end-2 w-20 h-20 rounded-full bg-main-background flex justify-center items-center translate-y-[-25%]">
                <img src="/img/account_balance_wallet.svg" alt="account balance wallet logo"/>
            </div>
            <div class="@lg:col-start-3 text-granola-orange text-base text-bold text-ellipsis w-10/12 overflow-hidden text-center @lg:text-left">
                {public_key}
            </div>
            <div class="@lg:col-start-3 @lg:row-start-2 text-slate-400 text-sm">
                "Username: "{username}
            </div>
        </div>
        <div class="@lg:grid @lg:grid-cols-[10rem_auto_10rem] bg-white rounded-xl flex flex-col items-stretch mt-8 p-4">
            {summary_items.into_iter()
                .map(|(label, value, has_pill)| view! {
                    <OverviewEntry label=label.to_owned() value=value.to_owned() has_pill=has_pill />
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn OverviewEntry(label: String, value: String, has_pill: bool) -> impl IntoView {
    let value_class_str_base = "py-1 my-1 text-sm";

    let value_class_str = match has_pill {
        true => format!(
            "{} {}",
            value_class_str_base.to_owned(),
            "p-1 rounded-full bg-light-granola-orange"
        ),
        false => format!(
            "{} {}",
            value_class_str_base.to_owned(),
            "w-3/4 text-ellipsis overflow-hidden"
        ),
    };

    view! {
        <div class="@lg:col-start-2 @lg:col-end-3 flex flex-col items-start md:flex-row md:items-baseline md:justify-start">
            <span class="w-1/4 text-slate-400 text-sm whitespace-nowrap">{label}:</span>
            <span class=value_class_str>{value}</span>
        </div>
    }
}
