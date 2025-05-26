use super::components::*;
use crate::common::{components::*, constants::*, functions::*, models::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn BlocksAnalyticsPage() -> impl IntoView {
    let (blockheight_lte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_LTE);
    let (blockheight_gte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_GTE);
    view! {
        <Title text="Analytics | Blocks" />
        <PageContainer>
            <AppSection>
                <AnalyticsFilters by_block=true />
                <AnalyticsLayout>
                    {move || {
                        blockheight_lte_sig.get();
                        blockheight_gte_sig.get();
                        view! {
                            <CacheBustScript src="/scripts/analytics/blocks.js" defer=true />
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Canonical Blocks".into())
                                    value=convert_to_span("...".to_string())
                                    id="canonical-blocks-count"
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Non-Canonical Blocks".into())
                                    value=convert_to_span("...".to_string())
                                    id="non-canonical-blocks-count"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("# of unique block producers".into())
                                    value=convert_to_span("...".to_string())
                                    id="unique-block-producers-count"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <span></span>
                            </AnalyticsSmContainer>
                            <AnalyticsXLContainer>
                                <div id="tree-container" class="w-full h-72 overflow-x-auto">
                                    <div id="tree" class="w-[3000px] h-full"></div>
                                </div>
                                <script type="text/javascript">
                                    (function() {
                                        let container = document.getElementById("tree-container");
                                        container.scrollLeft = container.scrollWidth / 2;
                                    })();
                                </script>
                            </AnalyticsXLContainer>
                            <AnalyticsLgContainer>
                                <div id="rewards" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="blocks" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="top-block-producers" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="top-block-earners" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                        }
                    }}
                </AnalyticsLayout>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn SnarksAnalyticsPage() -> impl IntoView {
    let (blockheight_lte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_LTE);
    let (blockheight_gte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_GTE);
    view! {
        <Title text="Analytics | SNARKs" />
        <PageContainer>
            <AppSection>
                <AnalyticsFilters by_block=true />
                <AnalyticsLayout>
                    {move || {
                        blockheight_lte_sig.get();
                        blockheight_gte_sig.get();
                        view! {
                            <CacheBustScript src="/scripts/analytics/snarks.js" defer=true />
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Fee-free SNARK work".into())
                                    value=convert_to_span("...".to_string())
                                    id="fee-free-work"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("For-fee SNARK work".into())
                                    value=convert_to_span("...".to_string())
                                    id="for-fee-jobs"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total SNARK jobs".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-snark-jobs"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Highest Fee".into())
                                    subtext=""
                                    value=convert_to_span("...".to_string())
                                    id="highest-fee"
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsLgContainer>
                                <div id="fee-distribution" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="snark-jobs-count" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="avg-snark-fee" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="fees-per-block" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="top-snark-provers" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="top-snark-workers" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                        }
                    }}

                </AnalyticsLayout>
            </AppSection>
            <SnarkFees />
        </PageContainer>
    }
}

#[component]
pub fn SnarkerLeaderboardPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Snarker Leaderboard" />
        <PageContainer>
            <AppSection>
                <SnarkerLeaderboard />
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn UserCommandsAnalyticsPage() -> impl IntoView {
    let (blockheight_lte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_LTE);
    let (blockheight_gte_sig, _) = create_query_signal::<u64>(QUERY_PARAM_BLOCKHEIGHT_GTE);
    view! {
        <Title text="Analytics | User Commands" />
        <PageContainer>
            <AppSection>
                <AnalyticsFilters by_block=true />
                <AnalyticsLayout>
                    {move || {
                        blockheight_lte_sig.get();
                        blockheight_gte_sig.get();
                        view! {
                            <CacheBustScript src="/scripts/analytics/user-commands.js" defer=true />
                            <CacheBustScript
                                src="/scripts/analytics/internal-commands.js"
                                defer=true
                            />
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Transferred".into())
                                    subtext="In millions of MINA"
                                    value=convert_to_span("...".to_string())
                                    id="total-transferred"
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Fees".into())
                                    value=convert_to_span("...".to_string())
                                    subtext="In MINA"
                                    id="total-fees"
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Number Of Transactions".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-number-of-transactions"
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Failed Account Creations".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-failed-account-creations"
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsLgContainer>
                                <div id="user-commands-volume" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="user-commands-top-recipients" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="user-commands-top-transfers" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="fee-spread" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="transfer-count" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                        }
                    }}
                </AnalyticsLayout>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn AnalyticsTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/analytics/blocks".to_string(),
            text: "Blocks".to_string(),
            icon: NavIcon::Blocks,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/commands/user".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/snarks".to_string(),
            text: "SNARKs".to_string(),
            icon: NavIcon::SNARKs,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/staker-leaderboard".to_string(),
            text: "Staker Leaderboard".to_string(),
            icon: NavIcon::Leaderboard,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/snarker-leaderboard".to_string(),
            text: "Snarker Leaderboard".to_string(),
            icon: NavIcon::Leaderboard,
            ..Default::default()
        },
    ];
    view! { <TabbedPage tabs /> }
}
