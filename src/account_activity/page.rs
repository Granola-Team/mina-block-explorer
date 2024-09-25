use super::models::DelegateCount;
use super::{functions::*, models::*};
use crate::account_activity::graphql::account_activity_query::AccountActivityQueryIncomingTransactions;
use crate::account_activity::graphql::account_activity_query::AccountActivityQueryOutgoingTransactions;
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
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use leptos_use::storage::use_local_storage;

#[component]
fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("there to be an optional account provided");

    let username = move || {
        account
            .get()
            .and_then(|acc| acc.username)
            .unwrap_or_default()
    };

    view! {
        <Title formatter=move |text| format!("Account Overview | {text}") text=username />
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
                            <WalletIcon width=40 />
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
                            <WalletIcon width=40 />
                        </SpotlightSection>
                    }
                }
            }} <Outlet />
        </PageContainer>
    }
}

#[component]
pub fn AccountUserCommandsPage() -> impl IntoView {
    let transactions = use_context::<
        ReadSignal<Option<Vec<Option<AccountActivityQueryDirectionalTransactions>>>>,
    >()
    .expect("Expected there to be an optional AccountActivityQueryDirectionalTransactions signal provided");
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
    let delegator_count: ReadSignal<Option<DelegateCount>> =
        use_context::<ReadSignal<Option<DelegateCount>>>()
            .expect("there to be an optional delegator count signal provided");
    {
        move || {
            view! {
                <AccountDelegationsSection
                    delegations_sig=delegations_sig
                    delegator_count=delegator_count.get().map(|c| c.0)
                    is_loading=Signal::derive(move || delegations_sig.get().is_none())
                />
            }
        }
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
    let (block_height_sig, _) = create_query_signal::<i64>("q-height");
    let (nonce_sig, _) = create_query_signal::<u64>("q-nonce");
    let (slot_sig, _) = create_query_signal::<u64>("q-slot");

    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let current_epoch_staking_ledger = move || Some(summary_sig.get().epoch);
    let activity_resource = create_resource(
        move || {
            (
                memo_params_map.get(),
                canonical_sig.get(),
                query_params_map.get(),
                block_height_sig.get(),
                nonce_sig.get(),
                slot_sig.get(),
                current_epoch_staking_ledger(),
            )
        },
        |(
            value,
            canonical_opt,
            qp_map,
            block_height,
            nonce,
            slot,
            current_epoch_staking_ledger,
        )| async move {
            if let Some(id) = value.get("id").cloned() {
                // Attempt to load data and handle any potential errors more gracefully
                match load_data(
                    Some(id.clone()),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    Some(TABLE_ROW_LIMIT),
                    block_height,
                    qp_map.get("q-txn-hash").cloned(),
                    qp_map.get("q-state-hash").cloned(),
                    Some(id.clone()),
                    nonce,
                    qp_map.get("q-counterparty").cloned(),
                    slot,
                    Some(id),
                    current_epoch_staking_ledger,
                    canonical_opt,
                )
                .await
                {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        logging::error!("Error loading data: {:?}", e); // Log the error
                        Err(e) // Return the error for further handling
                    }
                }
            } else {
                logging::error!("Could not parse id parameter from URL");
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    create_effect(move |_| {
        if let Some(res) = activity_resource.get().and_then(|res| res.ok()) {
            let mut transactions: Vec<Option<AccountActivityQueryDirectionalTransactions>> = res
                .incoming_transactions
                .into_iter()
                .filter_map(|t| {
                    t.map(|x| {
                        Some(<AccountActivityQueryIncomingTransactions as Into<
                            AccountActivityQueryDirectionalTransactions,
                        >>::into(x))
                    })
                })
                .chain(res.outgoing_transactions.into_iter().filter_map(|t| {
                    t.map(|x| {
                        Some(<AccountActivityQueryOutgoingTransactions as Into<
                            AccountActivityQueryDirectionalTransactions,
                        >>::into(x))
                    })
                }))
                .collect();
            transactions.sort_by(|a, b| {
                match (
                    a.as_ref().and_then(|x| x.date_time),
                    b.as_ref().and_then(|x| x.date_time),
                ) {
                    (Some(date_time_a), Some(date_time_b)) => date_time_b.cmp(&date_time_a),
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
                set_account.set(Some(account.clone()));
            }
            if let Some(Some(delegate)) = res.delegate.first() {
                let delegators: Vec<Option<AccountActivityQueryDelegatorExt>> = res
                    .delegators
                    .into_iter()
                    .map(|stake_opt| {
                        stake_opt.map(|delegator| extend_delegator_info(&delegator, delegate))
                    })
                    .collect();

                set_delegators.set(Some(delegators));
                set_delegators_counts.set(Some(DelegateCount(
                    delegate
                        .delegation_totals
                        .as_ref()
                        .and_then(|totals| {
                            totals.count_delegates.and_then(|c| usize::try_from(c).ok())
                        })
                        .unwrap_or_default(),
                )));
            }
        }
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
    provide_context(delegators_count);

    view! {
        { move || {
            transactions.get();
            internal_transactions.get();
            snarks.get();
            blocks.get();
            account.get();
            delegators.get();
            view! {
                <AccountSpotlightTabs />
            }
        }}
        <AccountSpotlightPage />
    }
}

#[component]
pub fn AccountSpotlightTabs() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.get().get("id").cloned().unwrap_or_default();

    let transactions = use_context::<
        ReadSignal<Option<Vec<Option<AccountActivityQueryDirectionalTransactions>>>>,
    >()
    .expect("there to be an optional AccountActivityQueryDirectionalTransactions signal provided");

    let delegator_count: ReadSignal<Option<DelegateCount>> =
        use_context::<ReadSignal<Option<DelegateCount>>>()
            .expect("there to be an optional DelegateCount signal provided");

    let txn: ReadSignal<Option<Vec<Option<AccountActivityQueryFeetransfers>>>> =
        use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryFeetransfers>>>>>()
            .expect("there to be an optional AccountActivityQueryFeetransfers signal provided");

    let blocks = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryBlocks>>>>>()
        .expect("there to be an optional AccountActivityQueryBlocks signal provided");

    let snarks = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQuerySnarks>>>>>()
        .expect("there to be an optional AccountActivityQuerySnarks signal provided");

    let tabs = vec![
        NavEntry {
            href: format!("/addresses/accounts/{}/commands/user", id()),
            text: "User Commands".to_string(),
            icon: NavIcon::Transactions,
            number_bubble: transactions
                .get()
                .map(|t| t.len())
                .map(Some)
                .unwrap_or(Some(0)), // Wrap in Some(usize)
            ..Default::default()
        },
        NavEntry {
            href: format!("/addresses/accounts/{}/commands/internal", id()),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Transactions,
            number_bubble: txn.get().map(|t| t.len()).map(Some).unwrap_or(Some(0)),
            ..Default::default()
        },
        NavEntry {
            href: format!("/addresses/accounts/{}/snark-jobs", id()),
            text: "SNARK Jobs".to_string(),
            icon: NavIcon::SNARKs,
            number_bubble: snarks.get().map(|t| t.len()).map(Some).unwrap_or(Some(0)),
            ..Default::default()
        },
        NavEntry {
            href: format!("/addresses/accounts/{}/block-production", id()),
            text: "Block Production".to_string(),
            icon: NavIcon::Blocks,
            number_bubble: blocks.get().map(|t| t.len()).map(Some).unwrap_or(Some(0)),
            ..Default::default()
        },
        NavEntry {
            href: format!("/addresses/accounts/{}/delegations", id()),
            text: "Delegations".to_string(),
            icon: NavIcon::Delegates,
            number_bubble: Some(delegator_count.get().map(|c| c.0).unwrap_or(0)), // Wrap in Some
            ..Default::default()
        },
    ];

    view! {
        <TabbedPage tabs exclude_outlet=true />
    }
}
