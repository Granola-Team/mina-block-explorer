use super::{components::AccountDialogTransactionSection, functions::*, models::*};
use crate::{
    account_activity::components::AccountDialogBlocksSection,
    common::{constants::*, models::MyError, spotlight::*},
    icons::*,
    snarks::components::AccountDialogSnarkJobSection,
    summary::models::BlockchainSummary,
};
use leptos::*;
use leptos_router::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

#[component]
pub fn AccountDialogView() -> impl IntoView {
    let location = use_location();
    let (base, _set_base) = create_signal(get_base_page_path(location));
    let memo_params_map = use_params_map();
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let public_key = move || memo_params_map.with(|p| p.get("id").cloned().unwrap_or_default());

    let account_activity_resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if value.get("id").cloned().is_some() {
                load_data(
                    value.get("id").cloned(),
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    None,
                    None,
                    value.get("id").cloned(),
                    None,
                    None,
                    None,
                    value.get("id").cloned(),
                    Some(true),
                )
                .await
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
            fallback=move || ().into_view()
        >
            <dialog
                id="accountdialog"
                class="z-20 w-full max-w-3xl h-screen fixed top-0 mr-0 ml-auto flex flex-col items-stretch bg-background"
            >
                <Suspense fallback=move || {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight"
                            top_right=Some(
                                Box::new(move || Fragment::new(
                                    vec![
                                        view! {
                                            <button id="closedialog" class="mr-4 cursor-pointer">
                                                <a href=base.get()>
                                                    <CloseIcon/>
                                                </a>
                                            </button>
                                        }
                                            .into_view(),
                                    ],
                                )),
                            )

                            spotlight_items=get_spotlight_loading_data()
                            id=None
                            meta=None
                        >
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                }>
                    {move || {
                        account_activity_resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|res| {
                                if let Some(Some(account)) = &res.accounts.first() {
                                    let summary_items = get_spotlight_data(
                                        account,
                                        summary_sig.get().blockchain_length,
                                    );
                                    view! {
                                        <SpotlightSection
                                            header="Account Spotlight"
                                            top_right=Some(
                                                Box::new(move || Fragment::new(
                                                    vec![
                                                        view! {
                                                            <button id="closedialog" class="mr-4 cursor-pointer">
                                                                <a href=base.get()>
                                                                    <CloseIcon/>
                                                                </a>
                                                            </button>
                                                        }
                                                            .into_view(),
                                                    ],
                                                )),
                                            )

                                            spotlight_items=summary_items
                                            id=Some(public_key())
                                            meta=Some(
                                                format!(
                                                    "Username: {}",
                                                    account
                                                        .username
                                                        .clone()
                                                        .map(|u| u.to_string())
                                                        .unwrap_or_default(),
                                                ),
                                            )
                                        >

                                            <WalletIcon width=40/>
                                        </SpotlightSection>
                                    }
                                } else {
                                    ().into_view()
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
                                let mut transactions: Vec<_> = res
                                    .incoming_transactions
                                    .into_iter()
                                    .filter(|t| t.is_some())
                                    .map(|r| r.map(|t| t.into()))
                                    .chain(
                                        res
                                            .outgoing_transactions
                                            .into_iter()
                                            .filter(|t| t.is_some())
                                            .map(|r| r.map(|t| t.into())),
                                    )
                                    .collect();
                                transactions
                                    .sort_by(|a, b| {
                                        match (
                                            <std::option::Option<
                                                AccountActivityQueryDirectionalTransactions,
                                            > as Clone>::clone(a)
                                                .unwrap()
                                                .date_time,
                                            <std::option::Option<
                                                AccountActivityQueryDirectionalTransactions,
                                            > as Clone>::clone(b)
                                                .unwrap()
                                                .date_time,
                                        ) {
                                            (Some(date_time_a), Some(date_time_b)) => {
                                                date_time_b.cmp(&date_time_a)
                                            }
                                            (Some(_), None) => std::cmp::Ordering::Greater,
                                            (None, Some(_)) => std::cmp::Ordering::Less,
                                            (None, None) => std::cmp::Ordering::Equal,
                                        }
                                    });
                                view! {
                                    <AccountDialogTransactionSection transactions/>
                                    <AccountDialogSnarkJobSection snarks=res
                                        .snarks
                                        .into_iter()
                                        .map(|r| r.map(|t| t.into()))
                                        .collect()/>
                                    <AccountDialogBlocksSection blocks=res.blocks/>
                                }
                            })}

                    </div>
                </Suspense>
                <div class="absolute bottom-0 left-0 w-full h-20 flex justify-stretch items-center bg-white">
                    <button
                        id="viewmore"
                        class="disabled:bg-slate-400 disabled:text-slate-200 disabled:cursor-not-allowed bg-granola-orange text-white uppercase mx-8 h-11 w-full rounded-lg"
                    >
                        <a href=format!(
                            "/addresses/accounts/{}",
                            public_key(),
                        )>"View all details"</a>
                    </button>
                </div>
            </dialog>
        </Show>
    }
}
