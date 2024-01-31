use leptos::*;

use super::functions::*;
use super::models::*;
use crate::blocks::components::AccountDialogBlocksSection;
use crate::common::models::*;
use crate::common::spotlight::*;
use crate::icons::*;
use crate::snarks::components::AccountDialogSnarkJobSection;
use crate::transactions::components::AccountDialogTransactionSection;

#[component]
pub fn AccountDialog(
    public_key: String,
    path_base: String,
    account: Option<AccountSummary>,
) -> impl IntoView {
    view! {
        <dialog id="accountdialog" class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <button id="closedialog" class="absolute top-0 right-0 p-12 z-30">
                <a href=path_base>X</a>
            </button>
            {match account {
                Some(acct) => {
                    let summary_items = get_spotlight_data(acct.clone());
                    let public_key = acct.public_key.clone();
                    view! {
                        <SpotlightSection header="Account Spotlight".to_string() spotlight_items=summary_items id=Some(public_key.clone()) meta=Some(format!("Username: {}",acct.username))>
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                },
                None => view! {
                    <SpotlightSection header="Account Spotlight".to_string() spotlight_items=get_spotlight_loading_data() id=None meta=None>
                        <WalletIcon width=40/>
                    </SpotlightSection>
                }
            }}
            <div class="overflow-y-auto flex flex-col pb-20">
                <AccountDialogTransactionSection limit=3 account_id=public_key.clone() />
                <AccountDialogSnarkJobSection public_key=Some(public_key.clone())/>
                <AccountDialogBlocksSection public_key=Some(public_key.clone())/>
            </div>
            <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                <button id="viewmore" class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg">
                    <a href={format!("/accounts/{}", public_key)}>"View all details"</a>
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
            <div class="w-1/2 flex my-1">
                <span class="text-xs text-slate-400 w-1/4">{label}:</span>
                <span class="text-xs overflow-hidden text-ellipsis w-3/4">{value}</span>
            </div>
        },
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
