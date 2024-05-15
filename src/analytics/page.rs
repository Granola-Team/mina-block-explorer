use crate::common::{components::*, models::*, table::*};
use indoc::indoc;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn InternalCommandsAnalayticsPage() -> impl IntoView {
    view! {
        <Title text="Analytics | Internal Commands"/>
        <PageContainer>
            <TableSection section_heading="Internal Commands Analytics" controls=|| ().into_view()>
                <AnalyticsLayout>
                    <AnalyticsXLContainer>
                        <span>"hello"</span>
                        <Script>
                        r#"
                        alert('Hello, world!');
                        "#
                        </Script>
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
            href: "/analytics/commands/user-commands".to_string(),
            text: "Transactions".to_string(),
            icon: NavIcon::Analytics,
            disabled: true,
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
