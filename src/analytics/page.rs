use crate::common::{components::*, models::*};
use leptos::*;
use leptos_meta::*;

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
            icon: NavIcon::Analytics,
            disabled: true,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/commands/user".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Analytics,
            ..Default::default()
        },
        NavEntry {
            href: "/analytics/commands/internal".to_string(),
            text: "Internal Commands".to_string(),
            icon: NavIcon::Analytics,
            ..Default::default()
        },
    ];
    view! { <TabbedPage tabs/> }
}
