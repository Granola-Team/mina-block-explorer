use leptos::html::AnyElement;
use leptos::*;

use super::functions::*;
use super::models::*;
use crate::blocks::components::AccountDialogBlocksSection;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::spotlight::*;
use crate::icons::*;
use crate::snarks::components::AccountDialogSnarkJobSection;
use crate::transactions::components::AccountDialogTransactionSection;

#[component]
pub fn AccountDialog(path_base: String, account: AccountSummary) -> impl IntoView {
    let id = account.public_key.clone();
    let summary_items = get_spotlight_data(account.clone());
    let public_key = account.public_key.clone();

    view! {
        <dialog id="accountdialog" class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background">
            <button id="closedialog" class="absolute top-0 right-0 p-12 z-30">
                <a href=path_base>X</a>
            </button>
            <SpotlightSection header="Account Spotlight".to_string() spotlight_items=summary_items id=account.public_key meta=format!("Username: {}",account.username)>
                <WalletIcon width=40/>
            </SpotlightSection>
            <div class="overflow-y-auto flex flex-col pb-20">
                <AccountDialogTransactionSection limit=3 account_id=public_key.clone() />
                <AccountDialogSnarkJobSection public_key=Some(public_key.clone())/>
                <AccountDialogBlocksSection public_key=Some(public_key)/>
            </div>
            <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                <button id="viewmore" class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg">
                    <a href={format!("/accounts/{}", id)}>"View all details"</a>
                </button>
            </div>
        </dialog>
    }.into_view()
}

struct AccountCardData {
    label: String,
    symbol: View,
    value: HtmlElement<AnyElement>,
}

#[component]
pub fn AccountCard(
    username: String,
    balance: f64,
    is_unlocked: bool,
    public_key: String,
    delegate: String,
    variant: AccountCardVariant,
) -> impl IntoView {
    let lock_icon = if is_unlocked {
        view! {<LockOpenIcon width=22 />}
    } else {
        view! {<LockClosedIcon />}
    };
    let info = vec![
        AccountCardData {
            label: "MINA".to_string(),
            symbol: lock_icon,
            value: convert_to_pill(balance.trunc().to_string(), PillVariant::Orange),
        },
        AccountCardData {
            label: "Public Key".to_string(),
            symbol: view! { <IDIcon width=22/> },
            value: convert_to_link(public_key.clone(), format!("/accounts/{}", public_key)),
        },
        AccountCardData {
            label: "Delegating To".to_string(),
            symbol: view! { <DelegateIcon width=22 />},
            value: convert_to_link(
                if public_key == delegate {
                    "Self".to_string()
                } else {
                    delegate.clone()
                },
                format!("/accounts/{}", delegate),
            ),
        },
    ];
    let card_bg_color = match variant {
        AccountCardVariant::Purple => "bg-card-purple",
        AccountCardVariant::Blue => "bg-card-blue",
        AccountCardVariant::Green => "bg-card-green",
    };
    let card_text_color = match variant {
        AccountCardVariant::Purple => "text-card-text-purple",
        _ => "text-card-text-blue",
    };
    view! {
        <div class=format!("w-full max-w-[640px] rounded-lg flex flex-col items-center p-4 grid grid-cols-1 gap-4 {}",card_bg_color)>
            <span class=format!("font-bold text-xl flex justify-start hover:underline {}",card_text_color)>
                <a href=format!("/accounts/{}",public_key)>{username}</a>
            </span>
            <div class="grid grid-cols-[25px_115px_1fr] gap-1">
                {info.into_iter()
                    .map(|i| view! {
                        <span class="m-1 font-normal text-slate-700 col-start-1 col-end-2">{i.symbol}</span>
                        <span class="m-1 font-normal text-slate-700 col-start-2 col-end-3">{i.label}:</span>
                        <span class="m-1 font-medium col-start-3 col-end-4 overflow-hidden text-ellipsis whitespace-nowrap">{i.value}</span>
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
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
