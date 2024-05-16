use crate::{
    account_activity::{
        dialog::AccountDialogView,
        page::{AccountSpotlightPage, AccountsPage, AddressesTabbedPage},
    },
    analytics::page::{AnalyticsTabbedPage, InternalCommandsAnalayticsPage},
    blocks::page::{
        BlockAnalyticsTab, BlockInternalCommandsTab, BlockSnarkJobsTab, BlockSpotlightTab,
        BlockTabbedPage, BlockUserCommandsTab,
    },
    broadcast::page::{
        BroadcastDelegationPage, BroadcastFromLedgerPage, BroadcastTransactionPage,
        DelegationTabbedPage,
    },
    common::{constants::*, search::*},
    footer::Footer,
    header::navigation::Header,
    internal_commands::components::InternalCommandsTab,
    next_stakes::page::NextStakesPage,
    snarks::page::SnarksPage,
    stakes::page::StakesPage,
    summary::page::SummaryPage,
    tokens::page::TokensPage,
    user_commands::page::{CommandSpotlightPage, CommandsTabbedPage, UserCommandsPage},
    zk_apps::page::{
        ZkAppSpotlight, ZkAppTransactionSpotlightPage, ZkAppTransactionsPage, ZkAppsPage,
    },
};
use leptos::*;
use leptos_router::*;
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <script>
            {format!(r#"const config = {{ graphql_endpoint: "{}" }}"#, GRAPHQL_ENDPOINT)}
        </script>
        <Router>
            <Header/>
            <GlobalSearchBar/>
            <main>
                <Routes>
                    // redirect any non-existent URL back to the blocks page
                    <Route path="/*" view=move || view! { <Redirect path="/blocks"/> }/>

                    <Route path="/addresses" view=AddressesTabbedPage>
                        <Route path="/*" view=move || view! { <Redirect path="accounts"/> }/>
                        <Route path="/accounts" view=AccountsPage/>
                        <Route path="/accounts/:id" view=AccountSpotlightPage/>
                        <Route
                            path="/tokens"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED == "true" {
                                    view! { <TokensPage/> }
                                } else {
                                    view!().into_view()
                                }
                            }
                        />

                        <Route
                            path="/zk-apps"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED == "true" {
                                    view! { <ZkAppsPage/> }
                                } else {
                                    view!().into_view()
                                }
                            }
                        />

                        <Route path="/zk-apps/:id" view=ZkAppSpotlight/>
                    </Route>

                    <Route path="/blocks" view=SummaryPage>
                        <Route path="/accounts/:id" view=AccountDialogView/>
                        <Route path="/*" view=|| ().into_view()/>
                    </Route>
                    <Route path="/blocks/:id" view=BlockTabbedPage>
                        <Route path="/*" view=move || view! { <Redirect path="spotlight"/> }/>
                        <Route path="/spotlight" view=BlockSpotlightTab/>
                        <Route path="/snark-jobs" view=BlockSnarkJobsTab/>
                        <Route path="/commands/user" view=BlockUserCommandsTab/>
                        <Route path="/commands/internal" view=BlockInternalCommandsTab/>
                        <Route path="/analytics" view=BlockAnalyticsTab/>
                    </Route>

                    <Route path="/commands" view=CommandsTabbedPage>
                        <Route path="*" view=move || view! { <Redirect path="user"/> }/>
                        <Route path="/user" view=UserCommandsPage/>
                        <Route path="/internal" view=InternalCommandsTab/>
                        <Route
                            path="/zk-app"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED == "true" {
                                    view! { <ZkAppTransactionsPage/> }
                                } else {
                                    view!().into_view()
                                }
                            }
                        />

                    </Route>
                    <Route path="/commands/:id" view=CommandSpotlightPage/>
                    <Route
                        path="/commands/zk-app/:id"
                        view=move || {
                            if BERKELEY_FEATURES_ENABLED == "true" {
                                view! { <ZkAppTransactionSpotlightPage/> }
                            } else {
                                view!().into_view()
                            }
                        }
                    />

                    <Route path="/snarks" view=SnarksPage/>

                    <Route path="/staking-ledgers" view=StakesPage/>

                    <Route path="/next-stakes" view=NextStakesPage/>

                    <Route path="/broadcast" view=DelegationTabbedPage>
                        <Route path="/*" view=move || view! { <Redirect path="transaction"/> }/>
                        <Route path="/transaction" view=BroadcastTransactionPage/>
                        <Route path="/delegation" view=BroadcastDelegationPage/>
                        <Route path="/ledger" view=BroadcastFromLedgerPage/>
                    </Route>
                    <Route path="/analytics" view=AnalyticsTabbedPage>
                        <Route
                            path="*"
                            view=move || view! { <Redirect path="commands/internal"/> }
                        />
                        <Route path="/commands/internal" view=InternalCommandsAnalayticsPage/>
                    </Route>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
