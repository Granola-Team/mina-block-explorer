use super::{functions::*, models::AccountActivityQueryDelegatorExt};
use crate::{
    account_activity::{
        components::{
            AccountDelegationsSection, AccountInternalCommandsSection, AccountOverviewBlocksTable,
            AccountOverviewSnarkJobTable, AccountTransactionsSection,
        },
        graphql::account_activity_query::{
            AccountActivityQueryAccounts, AccountActivityQueryBlocks,
            AccountActivityQueryFeetransfers, AccountActivityQuerySnarks,
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
    summary::models::BlockchainSummary,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

#[component]
fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("there to be an optional account provided");

    let username = move || {
        account
            .get()
            .and_then(|acc| acc.username)
            .unwrap_or_default()
    };

    view! {
        <Title formatter=move |text| format!("Account Overview | {text}") text=username/>
        <PageContainer>
            {move || match account.get() {
                Some(acc) => {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight"
                            spotlight_items=get_spotlight_data(
                                &acc,
                                summary_sig.get().blockchain_length,
                            )

                            meta=Some(format!("Username: {}", username()))

                            id=memo_params_map.get().get("id").cloned()
                        >
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                        .into_view()
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
            }}
            <Outlet/>
        </PageContainer>
    }
}

#[component]
pub fn AccountUserCommandsPage() -> impl IntoView {
    let transactions = use_context::<
        ReadSignal<Option<Vec<Option<AccountActivityQueryDirectionalTransactions>>>>,
    >()
    .expect("there to be an optional AccountActivityQueryDirectionalTransactions signal provided");
    view! {
        <AccountTransactionsSection
            transactions_sig=transactions
            is_loading=Signal::derive(move || transactions.get().is_none())
        />
    }
}

#[component]
pub fn AccountSnarkWorkPage() -> impl IntoView {
    let snarks = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQuerySnarks>>>>>()
        .expect("there to be an optional AccountActivityQuerySnarks signal provided");
    view! {
        <AccountOverviewSnarkJobTable
            snarks_sig=snarks
            is_loading=Signal::derive(move || snarks.get().is_none())
        />
    }
}

#[component]
pub fn AccountBlockProductionPage() -> impl IntoView {
    let blocks = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryBlocks>>>>>()
        .expect("there to be an optional AccountActivityQueryBlocks signal provided");
    view! {
        <AccountOverviewBlocksTable
            blocks_sig=blocks
            is_loading=Signal::derive(move || blocks.get().is_none())
        />
    }
}

#[component]
pub fn AccountInternalCommandsPage() -> impl IntoView {
    let txn: ReadSignal<Option<Vec<Option<_>>>> =
        use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryFeetransfers>>>>>()
            .expect("there to be an optional AccountActivityQueryFeetransfers signal provided");
    view! {
        <AccountInternalCommandsSection
            txn_sig=txn
            is_loading=Signal::derive(move || txn.get().is_none())
        />
    }
}

#[component]
pub fn AccountDelegationsPage() -> impl IntoView {
    let delegations_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>> =
        use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>>>()
            .expect("there to be an optional AccountActivityQueryFeetransfers signal provided");
    view! {
        <AccountDelegationsSection
            delegations_sig=delegations_sig
            is_loading=Signal::derive(move || delegations_sig.get().is_none())
        />
    }
}

#[component]
pub fn AccountSpotlightTabbedPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (account, set_account) = create_signal(None);
    let (transactions, set_transactions) = create_signal(None);
    let (internal_transactions, set_int_txn) = create_signal(None);
    let (snarks, set_snarks) = create_signal(None);
    let (blocks, set_blocks) = create_signal(None);
    let (delegators, set_delegators) = create_signal(None);
    let (delegators_count, set_delegators_counts) = create_signal(None);

    let query_params_map = use_query_map();
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let (block_height_sig, _) = create_query_signal::<u64>("q-height");
    let (nonce_sig, _) = create_query_signal::<u64>("q-nonce");
    let (slot_sig, _) = create_query_signal::<u64>("q-slot");

    let id = move || memo_params_map.get().get("id").cloned().unwrap_or_default();
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
            set_int_txn.set(Some(res.feetransfers));
            if let Some(Some(account)) = res.accounts.first() {
                let account_clone: AccountActivityQueryAccounts = account.clone();
                set_account.set(Some(account_clone));
            }
            if let Some(Some(delegate)) = res.delegate.first() {
                let delegators: Vec<Option<AccountActivityQueryDelegatorExt>> = res
                    .delegators
                    .into_iter()
                    .map(|stake_opt| {
                        stake_opt.map(|delegator| extend_delegator_info(&delegator, &delegate))
                    })
                    .collect::<Vec<_>>();
                set_delegators.set(Some(delegators));
                set_delegators_counts.set(
                    delegate
                        .delegation_totals
                        .as_ref()
                        .and_then(|totals| totals.count_delegates),
                )
            }
        };
    });

    create_effect(move |_| {
        canonical_sig.get();
        set_transactions.set(None);
        set_snarks.set(None);
        set_blocks.set(None);
        set_int_txn.set(None);
    });

    provide_context(transactions);
    provide_context(internal_transactions);
    provide_context(snarks);
    provide_context(blocks);
    provide_context(account);
    provide_context(delegators);

    let tabs = move || {
        vec![
            NavEntry {
                href: format!("/addresses/accounts/{}/commands/user", id()),
                text: "User Commands".to_string(),
                icon: NavIcon::Transactions,
                number_bubble: Some(transactions.get().map(|t| t.len()).unwrap_or(0)),
                ..Default::default()
            },
            NavEntry {
                href: format!("/addresses/accounts/{}/commands/internal", id()),
                text: "Internal Commands".to_string(),
                icon: NavIcon::Transactions,
                number_bubble: Some(blocks.get().map(|t| t.len()).unwrap_or(0)),
                ..Default::default()
            },
            NavEntry {
                href: format!("/addresses/accounts/{}/snark-jobs", id()),
                text: "SNARK Jobs".to_string(),
                icon: NavIcon::SNARKs,
                number_bubble: Some(snarks.get().map(|t| t.len()).unwrap_or(0)),
                ..Default::default()
            },
            NavEntry {
                href: format!("/addresses/accounts/{}/block-production", id()),
                text: "Block Production".to_string(),
                icon: NavIcon::Blocks,
                number_bubble: Some(blocks.get().map(|t| t.len()).unwrap_or(0)),
                ..Default::default()
            },
            NavEntry {
                href: format!("/addresses/accounts/{}/delegations", id()),
                text: "Delegations".to_string(),
                icon: NavIcon::Delegates,
                number_bubble: delegators_count
                    .get()
                    .and_then(|n| n.try_into().ok())
                    .or(Some(0)),
                ..Default::default()
            },
        ]
    };
    {
        move || {
            view! {
                <TabbedPage tabs=tabs() exclude_outlet=true/>
                <AccountSpotlightPage/>
            }
        }
    }
}
