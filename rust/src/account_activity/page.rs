use super::{
    functions::*,
    models::{DelegateCount, *},
};
use crate::{
    account_activity::{
        components::{
            AccountDelegationsSection, AccountInternalCommandsSection, AccountOverviewBlocksTable,
            AccountOverviewSnarkJobTable, AccountOverviewTokensTable, AccountTransactionsSection,
        },
        graphql::account_activity_query::{
            AccountActivityQueryAccounts, AccountActivityQueryBlocks,
            AccountActivityQueryInternalCommands, AccountActivityQuerySnarks,
            AccountActivityQueryTokenHolders,
        },
        models::AccountActivityQueryDirectionalTransactions,
    },
    common::{
        components::*,
        constants::*,
        functions::{decorate_with_mina_tag, nanomina_to_mina},
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
        .expect("Expected an optional account provided");

    let is_loading_sig = use_context::<ReadSignal<Option<bool>>>()
        .expect("Expected a bool signal to detect if resource is loading");

    let username = move || {
        account
            .get()
            .and_then(|acc| acc.username)
            .unwrap_or("Unknown".to_string())
    };

    view! {
        <Title formatter=move |text| format!("Account Overview | {text}") text=username />
        <PageContainer>
            {move || {
                let account_data = account.get();
                let is_loading = is_loading_sig.get();
                let genesis_balance = account_data.as_ref().and_then(|a| a.genesis_account);
                match (account_data, is_loading, genesis_balance) {
                    (None, Some(false), Some(balance)) => {

                        // No account, not loading, with genesis balance
                        view! {
                            <SpotlightSection
                                header="Account Spotlight"
                                spotlight_items=vec![
                                    SpotlightEntry {
                                        label: String::from("Genesis Balance"),
                                        any_el: Some(
                                            decorate_with_mina_tag(
                                                nanomina_to_mina(balance.try_into().unwrap()),
                                            ),
                                        ),
                                        copiable: false,
                                    },
                                ]
                                meta=Some(format!("Username: {}", username()))
                                id=memo_params_map.get().get("id").cloned()
                            >
                                <WalletIcon width=40 />
                            </SpotlightSection>
                        }
                            .into_view()
                    }
                    (Some(acc), Some(false), Some(_) | None) => {

                        // Account, not loading, unconcerned with genesis balance
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
                    (None, Some(false), None) => {

                        // No account, not loading, no genesis balance
                        view! {
                            <SpotlightSection
                                header="Account Spotlight"
                                spotlight_items=vec![]
                                meta=Some("Account has no MINA balance.".to_string())
                                id=memo_params_map.get().get("id").cloned()
                            >
                                <WalletIcon width=40 />
                            </SpotlightSection>
                        }
                            .into_view()
                    }
                    (_, _, _) => {

                        // Loading or unknown state
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
                            .into_view()
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
        .expect("Expected there to be an optional AccountActivityQuerySnarks signal provided");
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
        .expect("Expected there to be an optional AccountActivityQueryBlocks signal provided");
    view! {
        <AccountOverviewBlocksTable
            blocks_sig=blocks
            is_loading=Signal::derive(move || blocks.get().is_none())
        />
    }
}

#[component]
pub fn AccountTokensPage() -> impl IntoView {
    let tokens = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryTokenHolders>>>>>()
        .expect(
            "Expected there to be an optional AccountActivityQueryTokenHolders signal provided",
        );
    view! {
        <AccountOverviewTokensTable
            tokens_sig=tokens
            is_loading=Signal::derive(move || tokens.get().is_none())
        />
        <Outlet />
    }
}

#[component]
pub fn AccountInternalCommandsPage() -> impl IntoView {
    let txn: ReadSignal<Option<Vec<Option<_>>>> = use_context::<
        ReadSignal<Option<Vec<Option<AccountActivityQueryInternalCommands>>>>,
    >()
    .expect("Expectedthere to be an optional AccountActivityQueryInternalCommands signal provided");
    view! {
        <AccountInternalCommandsSection
            txn_sig=txn
            is_loading=Signal::derive(move || txn.get().is_none())
        />
    }
}

#[component]
pub fn AccountAccountTokensPageDelegationsPage() -> impl IntoView {
    let delegations_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>> =
        use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>>>().expect(
            "Expected there to be an optional AccountActivityQueryInternalCommands signal provided",
        );
    let delegator_count: ReadSignal<Option<DelegateCount>> =
        use_context::<ReadSignal<Option<DelegateCount>>>()
            .expect("Expected there to be an optional delegator count signal provided");
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
pub fn AccountDelegationsPage() -> impl IntoView {
    let delegations_sig: ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>> =
        use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryDelegatorExt>>>>>().expect(
            "Expected there to be an optional AccountActivityQueryInternalCommands signal provided",
        );
    let delegator_count: ReadSignal<Option<DelegateCount>> =
        use_context::<ReadSignal<Option<DelegateCount>>>()
            .expect("Expected there to be an optional delegator count signal provided");
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
    let route = use_route();
    let location = use_location();
    let (account, set_account) = create_signal(None);
    let (transactions, set_transactions) = create_signal(None);
    let (internal_transactions, set_int_txn) = create_signal(None);
    let (snarks, set_snarks) = create_signal(None);
    let (blocks, set_blocks) = create_signal(None);
    let (delegators, set_delegators) = create_signal(None);
    let (delegators_count, set_delegators_counts) = create_signal(None);
    let (tokens, set_tokens) = create_signal(None);
    let (is_loading_sig, set_is_loading) = create_signal(None);
    let get_tab = move || {
        let path = route.path();
        let pathname = location.pathname.get();
        pathname[path.len()..].to_string()
    };

    let query_params_map = use_query_map();
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let (block_height_sig, _) = create_query_signal::<i64>(QUERY_PARAM_HEIGHT);
    let (nonce_sig, _) = create_query_signal::<u64>(QUERY_PARAM_NONCE);
    let (slot_sig, _) = create_query_signal::<u64>(QUERY_PARAM_SLOT);
    let (q_type_sig, _) = create_query_signal::<String>(QUERY_PARAM_TYPE);
    let (q_direction_sig, _) = create_query_signal::<String>(QUERY_PARAM_DIRECTION);
    let (row_limit_sig, _) = create_query_signal::<i64>("row-limit");

    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let current_epoch_staking_ledger = move || {
        summary_sig
            .get()
            .chain
            .as_ref()
            .and_then(|c| c.get(MAINNET_2_CHAIN_ID))
            .map(|c| c.latest_epoch)
    };
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
                row_limit_sig.get(),
                q_type_sig.get(),
                q_direction_sig.get(),
                get_tab(),
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
            row_limit,
            q_type,
            q_direction,
            tab,
        )| async move {
            let (
                blocks_limit,
                snarks_limit,
                trans_limit,
                delegators_limit,
                internal_commands_limit,
            ) = set_tab_limits(&tab, row_limit);
            if let Some(id) = value.get("id").cloned() {
                // Attempt to load data and handle any potential errors more gracefully
                match load_data(
                    Some(id.clone()),
                    blocks_limit,
                    snarks_limit,
                    trans_limit,
                    delegators_limit,
                    internal_commands_limit,
                    block_height,
                    qp_map.get(QUERY_PARAM_TXN_HASH).cloned(),
                    qp_map.get(QUERY_PARAM_STATE_HASH).cloned(),
                    Some(id.clone()),
                    nonce,
                    qp_map.get(QUERY_PARAM_COUNTERPARTY).cloned(),
                    slot,
                    Some(id),
                    current_epoch_staking_ledger,
                    canonical_opt,
                    q_type.map(|q_type| q_type != TYPE_SEARCH_OPTION_ZKAPP),
                    q_direction.map(|d| d == DIRECTION_IN),
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
        set_is_loading.set(Some(activity_resource.loading().get()));
    });

    create_effect(move |_| {
        if let Some(res) = activity_resource.get().and_then(|res| res.ok()) {
            let transactions =
                merge_transactions(res.incoming_transactions, res.outgoing_transactions);
            let end_index = res.snarks.len().min(50);
            set_transactions.set(Some(transactions));
            set_snarks.set(Some(res.snarks[..end_index].to_vec()));
            set_blocks.set(Some(res.blocks));
            set_int_txn.set(Some(res.internal_commands));
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
            set_tokens.set(Some(res.token_holders));
        }
    });

    create_effect(move |_| {
        if is_loading_sig.get().is_some_and(|b| b) {
            set_transactions.set(None);
            set_snarks.set(None);
            set_blocks.set(None);
            set_int_txn.set(None);
            set_tokens.set(None);
        }
    });

    provide_context(transactions);
    provide_context(internal_transactions);
    provide_context(snarks);
    provide_context(blocks);
    provide_context(account);
    provide_context(delegators);
    provide_context(delegators_count);
    provide_context(tokens);
    provide_context(is_loading_sig);

    view! {
        {move || {
            transactions.get();
            internal_transactions.get();
            snarks.get();
            blocks.get();
            account.get();
            delegators.get();
            delegators_count.get();
            tokens.get();
            view! { <AccountSpotlightTabs /> }
        }}
        <AccountSpotlightPage />
    }
}

#[component]
pub fn AccountSpotlightTabs() -> impl IntoView {
    let memo_params_map = use_params_map();
    let id = move || memo_params_map.get().get("id").cloned().unwrap_or_default();

    let delegator_count: ReadSignal<Option<DelegateCount>> =
        use_context::<ReadSignal<Option<DelegateCount>>>()
            .expect("Expected there to be an optional DelegateCount signal provided");

    let account = use_context::<ReadSignal<Option<AccountActivityQueryAccounts>>>()
        .expect("Expected an account to be provided in the context");

    let tokens = use_context::<ReadSignal<Option<Vec<Option<AccountActivityQueryTokenHolders>>>>>()
        .expect("Expected there to be an optional tokens provided");

    let tabs = vec![
        NavEntry {
            href: format!(
                "/addresses/accounts/{}/{}/commands/user",
                MINA_TOKEN_ADDRESS,
                id()
            ),
            text: "User Commands".to_string(),
            icon: NavIcon::Transactions,
            number_bubble: account.get().and_then(|a| {
                a.pk_total_num_user_commands
                    .and_then(|t| usize::try_from(t).ok())
            }),
            ..Default::default()
        },
        NavEntry {
            href: format!(
                "/addresses/accounts/{}/{}/commands/internal",
                MINA_TOKEN_ADDRESS,
                id()
            ),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Transactions,
            number_bubble: account.get().and_then(|a| {
                a.pk_total_num_internal_commands
                    .and_then(|t| usize::try_from(t).ok())
            }),
            ..Default::default()
        },
        NavEntry {
            href: format!(
                "/addresses/accounts/{}/{}/snark-jobs",
                MINA_TOKEN_ADDRESS,
                id()
            ),
            text: "SNARK Jobs".to_string(),
            icon: NavIcon::SNARKs,
            number_bubble: account
                .get()
                .and_then(|a| a.pk_total_num_snarks.and_then(|t| usize::try_from(t).ok())),
            ..Default::default()
        },
        NavEntry {
            href: format!(
                "/addresses/accounts/{}/{}/block-production",
                MINA_TOKEN_ADDRESS,
                id()
            ),
            text: "Block Production".to_string(),
            icon: NavIcon::Blocks,
            number_bubble: account
                .get()
                .and_then(|a| a.pk_total_num_blocks.and_then(|t| usize::try_from(t).ok())),
            ..Default::default()
        },
        NavEntry {
            href: format!(
                "/addresses/accounts/{}/{}/delegations",
                MINA_TOKEN_ADDRESS,
                id()
            ),
            text: "Delegations".to_string(),
            icon: NavIcon::Delegates,
            number_bubble: Some(delegator_count.get().map(|c| c.0).unwrap_or(0)), // Wrap in Some
            ..Default::default()
        },
        NavEntry {
            href: format!("/addresses/accounts/{}/{}/tokens", MINA_TOKEN_ADDRESS, id()),
            text: "Tokens".to_string(),
            icon: NavIcon::Tokens,
            number_bubble: tokens.get().map(|tokens| tokens.len()), // Wrap in Some
            ..Default::default()
        },
    ];

    view! { <TabbedPage tabs exclude_outlet=true /> }
}
