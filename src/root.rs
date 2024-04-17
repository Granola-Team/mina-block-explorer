use crate::{
    account_activity::{
        dialog::AccountDialogView,
        page::{AccountSpotlightPage, AccountsPage, AddressesTabbedPage},
    },
    blocks::page::{
        BlockAnalyticsTab, BlockInternalCommandsTab, BlockSnarkJobsTab, BlockSpotlightTab,
        BlockTabbedPage, BlockUserCommandsTab, LatestBlocksPage,
    },
    broadcast::page::{
        BroadcastDelegationPage, BroadcastFromLedgerPage, BroadcastTransactionPage,
        DelegationTabbedPage,
    },
    config::BERKELEY_FEATURES_ENABLED,
    footer::Footer,
    header::navigation::Header,
    next_stakes::page::NextStakesPage,
    snarks::page::SnarksPage,
    stakes::page::StakesPage,
    summary::page::SummaryPage,
    tokens::page::TokensPage,
    transactions::page::{TransactionSpotlightPage, TransactionTabbedPage, TransactionsPage},
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
            <main>
                <Routes>
                    <Route path="/summary" view=SummaryPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=|| ().into_view()/>
                    </Route>
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
                    <Route path="/blocks" view=LatestBlocksPage>
                        <Route path="accounts/:id" view=AccountDialogView/>
                        <Route path="/*any" view=|| ().into_view()/>
                    </Route>
                    <Route path="/blocks/:id" view=BlockTabbedPage>
                        <Route path="/spotlight" view=BlockSpotlightTab/>
                        <Route path="/user-commands" view=BlockUserCommandsTab/>
                        <Route path="/snark-jobs" view=BlockSnarkJobsTab/>
                        <Route path="/internal-commands" view=BlockInternalCommandsTab/>
                        <Route path="/analytics" view=BlockAnalyticsTab/>
                        <Route path="/*any" view=BlockSpotlightTab/>
                    </Route>
                    <Route path="/commands" view=TransactionTabbedPage>
                        <Route path="/" view=TransactionsPage/>
                        <Route path="/user" view=TransactionsPage/>
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
                    <Route path="/commands/:id" view=TransactionSpotlightPage/>
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
                    <Route path="/stakes" view=StakesPage/>
                    <Route path="/next-stakes" view=NextStakesPage/>
                    <Route path="/broadcast" view=DelegationTabbedPage>
                        <Route path="/transaction" view=BroadcastTransactionPage/>
                        <Route path="/delegation" view=BroadcastDelegationPage/>
                        <Route path="/ledger" view=BroadcastFromLedgerPage/>
                        <Route path="/*any" view=BroadcastTransactionPage/>
                    </Route>
                    <Route path="/*any" view=SummaryPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
