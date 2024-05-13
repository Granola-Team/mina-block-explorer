use crate::{
    account_activity::{
        dialog::AccountDialogView,
        page::{AccountSpotlightPage, AccountsPage, AddressesTabbedPage},
    },
    blocks::page::{
        BlockAnalyticsTab, BlockInternalCommandsTab, BlockSnarkJobsTab, BlockSpotlightTab,
        BlockTabbedPage, BlockUserCommandsTab,
    },
    broadcast::page::{
        BroadcastDelegationPage, BroadcastFromLedgerPage, BroadcastTransactionPage,
        DelegationTabbedPage,
    },
    common::search::*,
    config::BERKELEY_FEATURES_ENABLED,
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
        <Router>
            <Header/>
            <GlobalSearchBar/>
            <main>
                <Routes>
                    // redirect any non-existent URL back to the blocks page
                    <Route path="/*" view=move || view! { <Redirect path="/blocks"/> } />

                    <Route path="/addresses" view=AddressesTabbedPage>
                        <Route path="" view=AccountsPage/>
                        <Route path="/accounts" view=AccountsPage/>
                        <Route path="/accounts/:id" view=AccountSpotlightPage/>
                        <Route
                            path="/tokens"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED {
                                    view! { <TokensPage/> }
                                } else {
                                    view!().into_view()
                                }
                            }
                        />

                        <Route
                            path="/zk-apps"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED {
                                    view! { <ZkAppsPage/> }
                                } else {
                                    view!().into_view()
                                }
                            }
                        />

                        <Route path="/zk-apps/:id" view=ZkAppSpotlight/>
                        <Route path="/*any" view=AccountsPage/>
                    </Route>

                    <Route path="/blocks" view=SummaryPage>
                        <Route path="/accounts/:id" view=AccountDialogView/>
                        <Route path="/*" view=|| ().into_view()/>
                    </Route>
                    <Route path="/blocks/:id" view=BlockTabbedPage>
                        <Route path="/spotlight" view=BlockSpotlightTab/>
                        <Route path="/user-commands" view=BlockUserCommandsTab/>
                        <Route path="/snark-jobs" view=BlockSnarkJobsTab/>
                        <Route path="/internal-commands" view=BlockInternalCommandsTab/>
                        <Route path="/analytics" view=BlockAnalyticsTab/>
                        <Route path="/*any" view=BlockSpotlightTab/>
                    </Route>
                    <Route path="/commands" view=CommandsTabbedPage>
                        <Route path="/" view=UserCommandsPage/>
                        <Route path="/user-commands" view=UserCommandsPage/>
                        <Route path="/internal-commands" view=InternalCommandsTab/>
                        <Route
                            path="/zk-app"
                            view=move || {
                                if BERKELEY_FEATURES_ENABLED {
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
                            if BERKELEY_FEATURES_ENABLED {
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
                        <Route path="/transaction" view=BroadcastTransactionPage/>
                        <Route path="/delegation" view=BroadcastDelegationPage/>
                        <Route path="/ledger" view=BroadcastFromLedgerPage/>
                        <Route path="/*any" view=BroadcastTransactionPage/>
                    </Route>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
