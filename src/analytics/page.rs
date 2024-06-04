use crate::common::{components::*, models::*, table::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn UserCommandsAnalyticsPage() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    view! {
        <Title text="Analytics | User Commands"/>
        <PageContainer>
            <TableSection
                metadata
                section_heading="User Commands Analytics"
                controls=|| ().into_view()
            >
                <AnalyticsLayout>
                    <AnalyticsXLContainer>
                        <div id="chart" class="w-full h-96"></div>
                        <script src="/scripts/analytics/user-commands-per-day.js" defer=true></script>
                    </AnalyticsXLContainer>
                </AnalyticsLayout>
            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn InternalCommandsAnalayticsPage() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    view! {
        <Title text="Analytics | Internal Commands"/>
        <PageContainer>
            <TableSection
                metadata
                section_heading="Internal Commands Analytics"
                controls=|| ().into_view()
            >
                <AnalyticsLayout>
                    <AnalyticsXLContainer>
                        <div id="chart" class="w-full h-96"></div>
                        <script src="/scripts/analytics/internal-commands.js" defer=true></script>
                    </AnalyticsXLContainer>
                </AnalyticsLayout>
            </TableSection>
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
