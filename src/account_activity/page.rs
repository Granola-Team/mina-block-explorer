use super::functions::*;
use crate::{
    account_activity::{
        components::{
            AccountOverviewBlocksTable, AccountOverviewSnarkJobTable, AccountTransactionsSection,
        },
        models::AccountActivityQueryDirectionalTransactions,
    },
    common::{
        components::*,
        constants::*,
        models::{MyError, NavEntry, NavIcon},
        spotlight::*,
    },
    icons::*,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let query_params_map = use_query_map();
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let (block_height_sig, _) = create_query_signal::<u64>("q-height");
    let (nonce_sig, _) = create_query_signal::<u64>("q-nonce");
    let (slot_sig, _) = create_query_signal::<u64>("q-slot");
    let (transactions, set_transactions) = create_signal(None);
    let (snarks, set_snarks) = create_signal(None);
    let (blocks, set_blocks) = create_signal(None);
    let (username, set_username) = create_signal(None);

    let activity_resource = create_resource(
        move || {
            (
                memo_params_map.get(),
                canonical_sig.get(),
                query_params_map.get(),
                block_height_sig.get(),
                nonce_sig.get(),
                slot_sig.get(),
            )
        },
        |(value, canonical_opt, qp_map, block_height, nonce, slot)| async move {
            if value.get("id").is_some() {
                load_data(
                    value.get("id").cloned(),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    block_height,
                    qp_map.get("q-txn-hash").cloned(),
                    qp_map.get("q-state-hash").cloned(),
                    value.get("id").cloned(),
                    nonce,
                    qp_map.get("q-counterparty").cloned(),
                    slot,
                    value.get("id").cloned(),
                    canonical_opt,
                )
                .await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    create_effect(move |_| {
        if let Some(res) = activity_resource.get().and_then(|res| res.ok()) {
            let mut transactions: Vec<_> = res
                .incoming_transactions
                .into_iter()
                .filter(|t| t.is_some())
                .map(|r| r.map(|t| t.into()))
                .chain(
                    res.outgoing_transactions
                        .into_iter()
                        .filter(|t| t.is_some())
                        .map(|r| r.map(|t| t.into())),
                )
                .collect();
            transactions.sort_by(|a, b| {
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
            let end_index = res.snarks.len().min(50);
            set_transactions.set(Some(transactions));
            set_snarks.set(Some(res.snarks[..end_index].to_vec()));
            set_blocks.set(Some(res.blocks));
            if let Some(Some(account)) = &res.accounts.first() {
                set_username.set(Some(
                    account
                        .username
                        .clone()
                        .map(|b| b.to_string())
                        .unwrap_or_default(),
                ))
            }
        };
    });

    create_effect(move |_| {
        canonical_sig.get();
        set_transactions.set(None);
        set_snarks.set(None);
        set_blocks.set(None);
    });

    view! {
        <Title
            formatter=move |text| format!("Account Overview | {text}")
            text=move || username.get().unwrap_or_default()
        />
        <PageContainer>
            {move || match activity_resource.get() {
                Some(Ok(res)) => {
                    if let Some(Some(account)) = &res.accounts.first() {
                        view! {
                            <SpotlightSection
                                header="Account Spotlight"
                                spotlight_items=get_spotlight_data(account)
                                meta=Some(
                                    format!("Username: {}", username.get().unwrap_or_default()),
                                )

                                id=memo_params_map.get().get("id").cloned()
                            >
                                <WalletIcon width=40/>
                            </SpotlightSection>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                }
                None => {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight"
                            spotlight_items=get_spotlight_loading_data()
                            meta=None
                            id=None
                        >
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                }
                _ => ().into_view(),
            }}
            <AccountTransactionsSection
                transactions_sig=transactions
                is_loading=activity_resource.loading()
            />
            <AccountOverviewSnarkJobTable snarks_sig=snarks is_loading=activity_resource.loading()/>
            <AccountOverviewBlocksTable blocks_sig=blocks is_loading=activity_resource.loading()/>
        </PageContainer>
    }
}

#[component]
pub fn AddressesTabbedPage() -> impl IntoView {
    let mut tabs = vec![NavEntry {
        href: "/addresses/accounts".to_string(),
        text: "Accounts".to_string(),
        icon: NavIcon::Addresses,
        ..Default::default()
    }];
    if BERKELEY_FEATURES_ENABLED == "true" {
        tabs.push(NavEntry {
            href: "/addresses/tokens".to_string(),
            text: "Tokens".to_string(),
            icon: NavIcon::Tokens,
            ..Default::default()
        });
        tabs.push(NavEntry {
            href: "/addresses/zk-apps".to_string(),
            text: "zk-apps".to_string(),
            icon: NavIcon::ZKApps,
            ..Default::default()
        })
    }
    view! { <TabbedPage tabs/> }
}
