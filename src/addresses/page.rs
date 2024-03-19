use super::functions::*;
use crate::{
    blocks::components::AccountOverviewBlocksTable,
    common::{
        components::*,
        functions::*,
        models::{MyError, NavEntry, NavIcon},
        search::*,
        spotlight::*,
        table::*,
    },
    icons::*,
    snarks::components::AccountOverviewSnarkJobTable,
    transactions::components::*,
};
use leptos::*;
use leptos_router::*;
use leptos_meta::Title;

#[component]
pub fn AccountsPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let resource = create_resource(
        move || query_params_map.get(),
        |value| async move {
            let mut public_key = value.get("id");
            if public_key.is_none() {
                public_key = value.get("query");
            }
            load_all_data(Some(0), Some(50), public_key.cloned()).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <Title text="Accounts | Search For Mina Account" />
        <PageContainer>
            <TableSection section_heading="Accounts".to_string() controls=|| ().into_view()>
                {move || match resource.get() {
                    Some(Ok(data)) => {
                        let pag = build_pagination(
                            data.data.len(),
                            records_per_page,
                            current_page.get(),
                            set_current_page,
                        );
                        let subset = get_subset(
                            &data.data.into_iter().map(Some).collect::<Vec<_>>(),
                            records_per_page,
                            current_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }
                    None => view! { <Table data=LoadingPlaceholder {}/> },
                    Some(Err(_)) => {
                        view! {
                            <EmptyTable message="Unable to list accounts at this time. Refresh to try again."
                                .to_string()/>
                        }
                    }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let (username, set_username) = create_signal(None);

    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            if let Some(id) = value.get("id").cloned() {
                let id_clone = id.clone();
                load_data(&id_clone).await
            } else {
                Err(MyError::ParseError(String::from(
                    "Could not parse id parameter from url",
                )))
            }
        },
    );

    let public_key = move || memo_params_map.get().get("id").cloned();
    
    create_effect(move |_| {
        resource.get()
            .and_then(|res| res.ok())
            .map(|res| {
                logging::log!("Username: {}",res.account.username);
                set_username.set(Some(res.account.username))
            });
    });

    view! {
        <Title formatter=move |text| format!("Account Overview | '{text}'") text=move || username.get().unwrap_or_default() />
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(res)) => {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight".to_string()
                            spotlight_items=get_spotlight_data(&res.account)
                            meta=Some(format!("Username: {}", res.account.username))
                            id=Some(public_key().unwrap_or_default())
                        >
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }
                        .into_view()
                }
                None => {
                    view! {
                        <SpotlightSection
                            header="Account Spotlight".to_string()
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
            {move || {
                view! {
                    <AccountTransactionsSection public_key=Some(public_key().unwrap_or_default())/>
                    <SubSectionContainer>
                        <AppSubSection
                            heading="SNARK Jobs".to_string()
                            position=SubSectionPosition::Left
                        >
                            <AccountOverviewSnarkJobTable public_key=Some(
                                public_key().unwrap_or_default(),
                            )/>
                        </AppSubSection>
                        <AppSubSection
                            heading="Block Production".to_string()
                            position=SubSectionPosition::Right
                        >
                            <AccountOverviewBlocksTable public_key=Some(
                                public_key().unwrap_or_default(),
                            )/>
                        </AppSubSection>
                    </SubSectionContainer>
                }
            }}

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
            disabled: true,
            ..Default::default()
        },
        NavEntry {
            href: "/addresses/tokens".to_string(),
            text: "Tokens".to_string(),
            icon: NavIcon::Tokens,
            disabled: true,
            ..Default::default()
        },
        NavEntry {
            href: "/addresses/zkApps".to_string(),
            text: "zkApps".to_string(),
            icon: NavIcon::ZKApps,
            disabled: true,
            ..Default::default()
        },
    ];
    view! {
        <SearchBar/>
        <TabbedPage tabs/>
    }
}
