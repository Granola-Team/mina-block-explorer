use leptos::*;
use leptos_router::*;

use super::functions::*;
use crate::blocks::components::AccountOverviewBlocksTable;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::MyError;
use crate::common::search::*;
use crate::common::spotlight::*;
use crate::common::table::*;
use crate::icons::WalletIcon;
use crate::snarks::components::AccountOverviewSnarkJobTable;
use crate::transactions::components::*;

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
        <SearchBar />
        <PageContainer>
            <TableSection section_heading="Accounts".to_string() controls=|| ().into_view()>
                {move || match resource.get() {
                    Some(Ok(data)) => {
                        let pag = build_pagination(data.data.len(), records_per_page, current_page.get(), set_current_page);
                        let subset = get_subset(&data.data.into_iter().map(Some).collect(), records_per_page, current_page.get()-1);
                        view! {
                            <Table data=subset pagination=pag />
                        }
                    },
                    _ => view! { <NullView />}
                }}
            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn AccountSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();

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

    let public_key = move || memo_params_map.with(|p| p.get("id").cloned());

    view! {
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(res)) =>{
                    view! {
                        <SpotlightSection header="Account Spotlight".to_string()
                            spotlight_items=get_spotlight_data(res.account.clone())
                            meta=Some(format!("Username: {}",res.account.username))
                            id=Some(public_key().unwrap_or_default())>
                            <WalletIcon width=40/>
                        </SpotlightSection>
                    }.into_view()
                },
                None => view! {
                    <SpotlightSection header="Account Spotlight".to_string()
                        spotlight_items=get_spotlight_loading_data()
                        meta=None
                        id=None>
                        <WalletIcon width=40/>
                    </SpotlightSection>
                },
                _ => view! { <NullView /> }
            }}
            <TransactionsSection public_key=Some(public_key().unwrap_or_default()) with_link=true/>
            <SubSectionContainer>
                <AppSubSection heading="SNARK Jobs".to_string() position=SubSectionPosition::Left>
                    <AccountOverviewSnarkJobTable public_key=Some(public_key().unwrap_or_default())/>
                </AppSubSection>
                <AppSubSection heading="Block Production".to_string() position=SubSectionPosition::Right>
                    <AccountOverviewBlocksTable public_key=Some(public_key().unwrap_or_default()) />
                </AppSubSection>
            </SubSectionContainer>
        </PageContainer>
    }
}
