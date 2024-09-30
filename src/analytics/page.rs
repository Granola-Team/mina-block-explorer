use super::components::*;
use crate::common::{components::*, functions::*, models::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

#[component]
pub fn BlocksAnalyticsPage() -> impl IntoView {
    let (limit_sig, _) = create_query_signal::<u64>("limit");
    view! {
        <Title text="Analytics | Blocks" />
        <PageContainer>
            <AppSection>
                <AppHeading heading="Blocks Analytics" />
                <AnalyticsFilters />
                <AnalyticsLayout>
                    {move || {
                        limit_sig.get();
                        view! {
                            <script src="/scripts/analytics/blocks.js" defer=true></script>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Canonical Blocks".into())
                                    value=convert_to_span("...".to_string())
                                    id="canonical-blocks-count"
                                    variant=ColorVariant::Blue
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Non-Canonical Blocks".into())
                                    value=convert_to_span("...".to_string())
                                    id="non-canonical-blocks-count"
                                    variant=ColorVariant::Green
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Number of supercharged blocks".into())
                                    value=convert_to_span("...".to_string())
                                    id="supercharged-blocks-count"
                                    variant=ColorVariant::DarkBlue
                                />
                            </AnalyticsSmContainer>

                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Number of unique block producers".into())
                                    value=convert_to_span("...".to_string())
                                    id="unique-block-producers-count"
                                    variant=ColorVariant::Orange
                                />
                            </AnalyticsSmContainer>
                            <AnalyticsLgContainer>
                                <div id="rewards" class="w-full h-96"></div>
                            </AnalyticsLgContainer>
                            <AnalyticsLgContainer>
                                <div id="blocks" class="w-full h-96"></div>
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
    let (limit_sig, _) = create_query_signal::<u64>("limit");
    view! {
        <Title text="Analytics | SNARKs" />
        <PageContainer>
            <AppSection>
                <AppHeading heading="SNARK Analytics" />
                <AnalyticsFilters />
                <AnalyticsLayout>
                    {move || {
                        limit_sig.get();
                        view! {
                            // redraw

                            <AnalyticsXLContainer>
                                <div id="avg-snark-fee" class="w-full h-96"></div>
                                <script
                                    src="/scripts/analytics/avg-snark-fee-per-block.js"
                                    defer=true
                                ></script>
                            </AnalyticsXLContainer>
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
pub fn StakerLeaderboardPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Staker Leaderboard" />
        <PageContainer>
            <AppSection>
                <StakerLeaderboard />
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn UserCommandsAnalyticsPage() -> impl IntoView {
    let (limit_sig, _) = create_query_signal::<u64>("limit");
    view! {
        <Title text="Analytics | User Commands" />
        <PageContainer>
            <AppSection>
                <AppHeading heading="User Commands Analytics" />
                <AnalyticsFilters />
                <AnalyticsLayout>
                    {move || {
                        limit_sig.get();
                        view! {
                            <script src="/scripts/analytics/user-commands.js" defer=true></script>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Transferred".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-transferred"
                                    variant=ColorVariant::Blue
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Fees".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-fees"
                                    variant=ColorVariant::Green
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Number Of Transactions".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-number-of-transactions"
                                    variant=ColorVariant::Orange
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Total Failed Account Creations".into())
                                    value=convert_to_span("...".to_string())
                                    id="total-failed-account-creations"
                                    variant=ColorVariant::DarkBlue
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
                        }
                    }}
                </AnalyticsLayout>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn InternalCommandsAnalayticsPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Internal Commands" />
        <PageContainer>
            <AppSection>
                <AppHeading heading="Internal Commands Analytics" />
                <AnalyticsLayout>
                    <script src="/scripts/analytics/internal-commands.js" defer=true></script>
                    <AnalyticsLgContainer>
                        <div id="fee-spread" class="w-full h-96"></div>
                    </AnalyticsLgContainer>
                    <AnalyticsLgContainer>
                        <div id="transfer-count" class="w-full h-96"></div>
                    </AnalyticsLgContainer>
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
            href: "/analytics/commands/internal".to_string(),
            text: "Internal Commands".to_string(),
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
