use crate::{
    account_activity::page::{
        AccountBlockProductionPage, AccountDelegationsPage, AccountInternalCommandsPage,
        AccountSnarkWorkPage, AccountSpotlightTabbedPage, AccountUserCommandsPage,
    },
    accounts::page::AccountsPage,
    analytics::page::{
        AnalyticsTabbedPage, BlocksAnalyticsPage, SnarkerLeaderboardPage, SnarksAnalyticsPage,
        StakerLeaderboardPage, UserCommandsAnalyticsPage,
    },
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
    snarks::page::SnarksPage,
    stakes::page::StakesPage,
    summary::page::{SummaryLocalStorage, SummaryPage},
    user_commands::page::{
        CommandSpotlightPage, CommandsTabbedPage, PendingCommandsPage, UserCommandsPage,
    },
};
use leptos::*;
use leptos_router::*;
#[component]
pub fn Root() -> impl IntoView {
    view! {
        <script>
            {format!(
                r#"const config = {{ graphql_endpoint: "{}", rest_endpoint: "{}" }}"#,
                GRAPHQL_ENDPOINT,
                REST_ENDPOINT,
            )}
        </script>
        <script src="/scripts/analytics/charts-common.js" defer=true></script>
        <SummaryLocalStorage />
        // TODO: loading 1000 blocks is too expensive for now
        // <BlocksLocalStorage/>
        <Router>
            <Header />
            <GlobalSearchBar />
            <main>
                <Routes>
                    // redirect any non-existent URL back to the blocks page
                    <Route
                        path="/*"
                        view=move || {
                            view! {
                                <Redirect
                                    path="/blocks"
                                    options=NavigateOptions {
                                        replace: true,
                                        ..Default::default()
                                    }
                                />
                            }
                        }
                    />
                    <Route path="/addresses/accounts" view=AccountsPage />
                    <Route path="/addresses/accounts/:id" view=AccountSpotlightTabbedPage>
                        <Route
                            path="*"
                            view=move || {
                                view! {
                                    <Redirect
                                        path="commands/user"
                                        options=NavigateOptions {
                                            replace: true,
                                            ..Default::default()
                                        }
                                    />
                                }
                            }
                        />
                        <Route path="/commands/user" view=AccountUserCommandsPage />
                        <Route path="/commands/internal" view=AccountInternalCommandsPage />
                        <Route path="/snark-jobs" view=AccountSnarkWorkPage />
                        <Route path="/block-production" view=AccountBlockProductionPage />
                        <Route path="/delegations" view=AccountDelegationsPage />
                    </Route>
                    // <Route path="/tokens" view=TokensPage />
                    // <Route path="/zk-apps" view=ZkAppsPage />
                    // <Route path="/zk-apps/:id" view=ZkAppSpotlight />

                    <Route path="/blocks" view=SummaryPage />
                    <Route path="/blocks/:id" view=BlockTabbedPage>
                        <Route
                            path="/*"
                            view=move || {
                                view! {
                                    <Redirect
                                        path="spotlight"
                                        options=NavigateOptions {
                                            replace: true,
                                            ..Default::default()
                                        }
                                    />
                                }
                            }
                        />
                        <Route path="/spotlight" view=BlockSpotlightTab />
                        <Route path="/snark-jobs" view=BlockSnarkJobsTab />
                        <Route path="/commands/user" view=BlockUserCommandsTab />
                        <Route path="/commands/internal" view=BlockInternalCommandsTab />
                        <Route path="/analytics" view=BlockAnalyticsTab />
                    </Route>

                    <Route path="/commands" view=CommandsTabbedPage>
                        <Route
                            path="*"
                            view=move || {
                                view! {
                                    <Redirect
                                        path="user"
                                        options=NavigateOptions {
                                            replace: true,
                                            ..Default::default()
                                        }
                                    />
                                }
                            }
                        />
                        <Route path="/user" view=UserCommandsPage />
                        <Route path="/pending" view=PendingCommandsPage />
                        <Route path="/internal" view=InternalCommandsTab />
                    // <Route path="/zk-app" view=ZkAppTransactionsPage />
                    </Route>
                    <Route path="/commands/:id" view=CommandSpotlightPage />
                    // <Route path="/commands/zk-app/:id" view=ZkAppTransactionSpotlightPage />
                    <Route path="/snarks" view=SnarksPage />

                    <Route path="/staking-ledgers" view=StakesPage />

                    <Route path="/broadcast" view=DelegationTabbedPage>
                        <Route
                            path="/*"
                            view=move || {
                                view! {
                                    <Redirect
                                        path="transaction"
                                        options=NavigateOptions {
                                            replace: true,
                                            ..Default::default()
                                        }
                                    />
                                }
                            }
                        />
                        <Route path="/transaction" view=BroadcastTransactionPage />
                        <Route path="/delegation" view=BroadcastDelegationPage />
                        <Route path="/ledger" view=BroadcastFromLedgerPage />
                    </Route>
                    <Route path="/analytics" view=AnalyticsTabbedPage>
                        <Route
                            path="*"
                            view=move || {
                                view! {
                                    <Redirect
                                        path="blocks"
                                        options=NavigateOptions {
                                            replace: true,
                                            ..Default::default()
                                        }
                                    />
                                }
                            }
                        />
                        <Route path="/commands/user" view=UserCommandsAnalyticsPage />
                        <Route path="/blocks" view=BlocksAnalyticsPage />
                        <Route path="/snarks" view=SnarksAnalyticsPage />
                        <Route path="/staker-leaderboard" view=StakerLeaderboardPage />
                        <Route path="/snarker-leaderboard" view=SnarkerLeaderboardPage />
                    </Route>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
