use super::{components::*, functions::*};
use crate::common::{components::*, functions::*, models::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn BlocksAnalyticsPage() -> impl IntoView {
    let resource = create_resource(
        || (),
        move |_| async move { load_block_summary_data().await },
    );
    view! {
        <Title text="Analytics | Blocks"/>
        <PageContainer>
            <AppSection>
                <AppHeading heading="Blocks Analytics"/>
                <AnalyticsLayout>
                    <Suspense fallback=move || {
                        view! {
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
                        }
                    }>
                        {resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|data| {
                                let data_clone = data.clone();
                                view! {
                                    <AnalyticsSmContainer>
                                        <AnalyticsSimpleInfo
                                            label=convert_to_span("Total Blocks".into())
                                            value=convert_to_span(
                                                data_clone
                                                    .data
                                                    .blocks
                                                    .first()
                                                    .map(|b| b.total_num_blocks.to_string())
                                                    .unwrap_or_default(),
                                            )

                                            variant=ColorVariant::Blue
                                        />

                                    </AnalyticsSmContainer>
                                    <AnalyticsSmContainer>
                                        <AnalyticsSimpleInfo
                                            label=convert_to_span("Blocks This Epoch".into())
                                            value=convert_to_span(
                                                data
                                                    .data
                                                    .blocks
                                                    .first()
                                                    .map(|b| b.epoch_num_blocks.to_string())
                                                    .unwrap_or_default(),
                                            )

                                            variant=ColorVariant::Green
                                        />

                                    </AnalyticsSmContainer>
                                }
                            })}

                    </Suspense>
                    <AnalyticsXLContainer>
                        <div id="chart" class="w-full h-96"></div>
                        <script src="/scripts/analytics/blocks-rewards.js" defer=true></script>
                    </AnalyticsXLContainer>
                </AnalyticsLayout>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn SnarksAnalyticsPage() -> impl IntoView {
    view! {
        <Title text="Analytics | SNARKs"/>
        <PageContainer>
            <AppSection>
                <AnalyticsLayout>
                    <AnalyticsXLContainer>
                        <div id="avg-snark-fee" class="w-full h-96"></div>
                        <script
                            src="/scripts/analytics/avg-snark-fee-per-block.js"
                            defer=true
                        ></script>
                    </AnalyticsXLContainer>
                </AnalyticsLayout>
            </AppSection>
            <SnarkFees/>
        </PageContainer>
    }
}

#[component]
pub fn UserCommandsAnalyticsPage() -> impl IntoView {
    view! {
        <Title text="Analytics | User Commands"/>
        <PageContainer>
            <AppSection>
                <AppHeading heading="User Commands Analytics"/>
                <AnalyticsLayout>
                    <AnalyticsXLContainer>
                        <div id="chart" class="w-full h-96"></div>
                        <script
                            src="/scripts/analytics/user-commands-per-day.js"
                            defer=true
                        ></script>
                    </AnalyticsXLContainer>
                </AnalyticsLayout>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn InternalCommandsAnalayticsPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Internal Commands"/>
        <PageContainer>
            <AppSection>
                <AppHeading heading="Internal Commands Analytics"/>
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
    view! { <TabbedPage tabs/> }
}
