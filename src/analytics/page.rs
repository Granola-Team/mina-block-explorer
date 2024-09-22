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
                <AppHeading heading="Filters" />
                <AnalayticsFilters />
            </AppSection>
            <AppSection>
                <AppHeading heading="Blocks Analytics" />
                <AnalyticsLayout>
                    <AnalyticsSmContainer>
                        <AnalyticsSimpleInfo
                            label=convert_to_span("Total Blocks".into())
                            value=convert_to_span("...".to_string())

                            variant=ColorVariant::Blue
                        />

                    </AnalyticsSmContainer>
                    <AnalyticsSmContainer>
                        <AnalyticsSimpleInfo
                            label=convert_to_span("Blocks This Epoch".into())
                            value=convert_to_span("...".to_string())

                            variant=ColorVariant::Green
                        />

                    </AnalyticsSmContainer>
                    {move || {
                        limit_sig.get();
                        view! {
                            <AnalyticsXLContainer>
                                <div id="chart" class="w-full h-96"></div>
                                <script
                                    src="/scripts/analytics/blocks-rewards.js"
                                    defer=true
                                ></script>
                            </AnalyticsXLContainer>
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
                <AppHeading heading="Filters" />
                <AnalayticsFilters />
            </AppSection>
            <AppSection>
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
pub fn UserCommandsAnalyticsPage() -> impl IntoView {
    let (limit_sig, _) = create_query_signal::<u64>("limit");
    view! {
        <Title text="Analytics | User Commands" />
        <PageContainer>
            <AppSection>
                <AppHeading heading="Filters" />
                <AnalayticsFilters />
            </AppSection>
            <AppSection>
                <AppHeading heading="User Commands Analytics" />
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
                            <AnalyticsXLContainer>
                                <div id="user-commands-volume" class="w-full h-96"></div>
                            </AnalyticsXLContainer>
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
                    <AnalyticsXLContainer>
                        <div id="chart" class="w-full h-96"></div>
                        <script src="/scripts/analytics/internal-commands.js" defer=true></script>
                    </AnalyticsXLContainer>
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
    ];
    view! { <TabbedPage tabs /> }
}
