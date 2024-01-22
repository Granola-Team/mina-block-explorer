use leptos::*;
use leptos_router::*;

use super::functions::*;
use super::models::*;

use super::components::*;
use crate::blocks::components::AccountOverviewBlocksTable;
use crate::common::components::*;
use crate::common::models::MyError;
use crate::common::search::*;
use crate::common::spotlight::*;
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

    view! {
        <SearchBar />
        <PageContainer>
            <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section mb-4">
                <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Accounts"</h1>
                <div class="sm:p-8 grid grid-cols-1 gap-2 md:grid-cols-2 xl:grid-cols-3">
                    {move || match resource.get() {
                        Some(Ok(data)) =>  {
                            data.data.into_iter()
                                .enumerate()
                                .map(|(i, account)| view! {
                                    <AccountCard username=account.username
                                        balance=account.balance
                                        nonce=account.nonce
                                        is_unlocked=true
                                        public_key=account.public_key
                                        delegate=account.delegate
                                        variant={match i%3 {
                                            0 => AccountCardVariant::Purple,
                                            1 => AccountCardVariant::Green,
                                            _ => AccountCardVariant::Blue
                                        }}/>
                                })
                                .collect::<Vec<_>>()
                        }.into_view(),
                        _ => view! {<NullView />}
                    }}
                </div>
            </section>
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

    view! {
        <PageContainer>
            {move || match resource.get() {
                Some(Ok(res)) =>{
                    let pk =res.account.public_key.clone();
                    let pk_1 =res.account.public_key.clone();
                    let pk_2 =res.account.public_key.clone();
                    let pk_3 =res.account.public_key.clone();
                    view! {
                        <SpotlightSection header="Account Spotlight".to_string()
                            spotlight_items=get_spotlight_data(res.account.clone())
                            meta=format!("Username: {}",res.account.username)
                            id=pk>
                            <WalletIcon width=40/>
                        </SpotlightSection>
                        <TransactionsSection public_key=Some(pk_1) with_link=true/>
                        <SubSectionContainer>
                            <AppSubSection heading="SNARK Jobs".to_string() position=SubSectionPosition::Left>
                                <AccountOverviewSnarkJobTable public_key=Some(pk_2)/>
                            </AppSubSection>
                            <AppSubSection heading="Block Production".to_string() position=SubSectionPosition::Right>
                                <AccountOverviewBlocksTable public_key=Some(pk_3) />
                            </AppSubSection>
                        </SubSectionContainer>
                    }.into_view()
                },
                _ => view! { <NullView /> }
            }}
        </PageContainer>
    }
}
