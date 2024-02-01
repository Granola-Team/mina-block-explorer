use leptos::*;
use leptos_router::*;

use super::functions::get_base_page_path;
use crate::common::models::MyError;

use super::functions::load_data;
use crate::accounts::functions::{load_data as load_summary_data, *};
use crate::common::components::*;
use crate::common::spotlight::*;

use crate::icons::*;
use crate::transactions::components::AccountDialogTransactionSection;
use crate::snarks::components::AccountDialogSnarkJobSection;

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let (base, _set_base) = create_signal(get_base_page_path(location));
    let memo_params_map = use_params_map();

    let account_resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_summary_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    let public_key = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());

    let account_activity_resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_data(Some(id_clone)).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    view! {
        <Show
            when=move || { memo_params_map.get().get("id").is_some() && !base.get().is_empty() }
            fallback=move || view! { <NullView/> }
        >
            <dialog
                id="accountdialog"
                class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch p-4 bg-background"
            >
                <button id="closedialog" class="absolute top-0 right-0 p-12 z-30">
                    <a href=base.get()>X</a>
                </button>
                <Suspense fallback=move || {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight".to_string()
                            spotlight_items=get_spotlight_loading_data()
                            id=None
                            meta=None
                        >
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                }>
                    {move || {
                        account_resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|res| {
                                let summary_items = get_spotlight_data(&res.account);
                                view! {
                                    <SpotlightSection
                                        header="Account Spotlight".to_string()
                                        spotlight_items=summary_items
                                        id=Some(public_key())
                                        meta=Some(format!("Username: {}", res.account.username))
                                    >
                                        <WalletIcon width=40/>
                                    </SpotlightSection>
                                }
                            })
                    }}
                </Suspense>
                <Suspense>
                    <div class="overflow-y-auto flex flex-col pb-20">
                        {account_activity_resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|res| {
                                view! {
                                    <AccountDialogTransactionSection transactions=res
                                        .transactions
                                        .into_iter()
                                        .map(|r| r.map(|t| t.into()))
                                        .collect()/>
                                    <AccountDialogSnarkJobSection snarks=res
                                        .snarks
                                        .into_iter()
                                        .map(|r| r.map(|t| t.into()))
                                        .collect()/>
                                }
                            })}

                    // <AccountDialogSnarkJobSection public_key=Some(public_key.clone())/>
                    // <AccountDialogBlocksSection public_key=Some(public_key.clone())/>
                    </div>
                </Suspense>
                <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                    <button
                        id="viewmore"
                        class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg"
                    >
                        <a href=format!("/accounts/{}", public_key())>"View all details"</a>
                    </button>
                </div>
            </dialog>
        </Show>
    }
}
