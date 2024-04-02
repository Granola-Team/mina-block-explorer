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
        functions::*,
        models::{MyError, NavEntry, NavIcon},
        search::*,
        spotlight::*,
        table::*,
    },
    icons::*,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn AccountsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_account_summaries(9000);

    view! {
        <Title text="Accounts | Search For Mina Account"/>
        <PageContainer>
            <TableSection section_heading="Accounts" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let (transactions, set_transactions) = create_signal(None);
    let (snarks, set_snarks) = create_signal(None);
    let (blocks, set_blocks) = create_signal(None);
    let (username, set_username) = create_signal(None);

    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_account_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    let activity_resource = create_resource(
        move || (memo_params_map.get(), canonical_sig.get()),
        |(value, canonical_opt)| async move {
            if value.get("id").is_some() {
                load_data(
                    value.get("id").cloned(),
                    Some(50),
                    Some(50),
                    Some(50),
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
            set_transactions.set(Some(transactions));
            set_snarks.set(Some(res.snarks));
            set_blocks.set(Some(res.blocks));
        };
    });

    create_effect(move |_| {
        canonical_sig.get();
        set_transactions.set(None);
        set_snarks.set(None);
        set_blocks.set(None);
    });

    create_effect(move |_| {
        if let Some(Ok(data)) = resource.get() {
            logging::log!("Username: {}", data.account.username);
            set_username.set(Some(data.account.username))
        };
    });

    view! {
        <Title
            formatter=move |text| format!("Account Overview | '{text}'")
            text=move || username.get().unwrap_or_default()
        />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(res)) => {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight"
                            spotlight_items=get_spotlight_data(&res.account)
                            meta=Some(format!("Username: {}", res.account.username))
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
                _ => view! { <NullView/> },
            }}
            {move || match transactions.get() {
                Some(transactions) => view! { <AccountTransactionsSection transactions/> },
                None => {
                    view! {
                        <TableSection section_heading="Transactions" controls=|| ().into_view()>
                            <Table data=LoadingPlaceholder {}/>
                        </TableSection>
                    }
                }
            }}
            <SubSectionContainer>
                <AppSubSection heading="SNARK Jobs" position=SubSectionPosition::Left>
                    {move || match snarks.get() {
                        Some(snarks) => {
                            view! {
                                <AccountOverviewSnarkJobTable
                                    snarks
                                    public_key=memo_params_map.get().get("id").cloned()
                                />
                            }
                        }
                        None => view! { <Table data=LoadingPlaceholder {}/> },
                    }}

                </AppSubSection>
                <AppSubSection heading="Block Production" position=SubSectionPosition::Right>
                    {move || match blocks.get() {
                        Some(blocks) => {
                            view! {
                                <AccountOverviewBlocksTable
                                    blocks
                                    public_key=memo_params_map.get().get("id").cloned()
                                />
                            }
                        }
                        None => view! { <Table data=LoadingPlaceholder {}/> },
                    }}

                </AppSubSection>
            </SubSectionContainer>
        </PageContainer>
    }
}

#[component]
pub fn AddressesTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/addresses/accounts".to_string(),
            text: "Accounts".to_string(),
            icon: NavIcon::Addresses,
            ..Default::default()
        },
        NavEntry {
            href: "/addresses/tokens".to_string(),
            text: "Tokens".to_string(),
            icon: NavIcon::Tokens,
            ..Default::default()
        },
        NavEntry {
            href: "/addresses/zk-apps".to_string(),
            text: "zk-apps".to_string(),
            icon: NavIcon::ZKApps,
            ..Default::default()
        },
    ];
    view! {
        <SearchBar/>
        <TabbedPage tabs/>
    }
}
