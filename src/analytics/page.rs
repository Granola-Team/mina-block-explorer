use crate::common::{components::*, constants::*, functions::*, models::*};
use leptos::*;
use leptos_meta::*;
use serde::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlockAnalyticsData {
    pub epoch_num_blocks: i64,
    pub total_num_blocks: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlocksAnalyticsData {
    pub blocks: Vec<BlockAnalyticsData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BlocksAnalyticsResponse {
    pub data: BlocksAnalyticsData,
}

async fn load_block_summary_data() -> Result<BlocksAnalyticsResponse, MyError> {
    let query_body = r#"{"query":"query BlocksQuery(\n  $limit: Int = 1\n) {\n  blocks(limit: $limit) {\n    epoch_num_blocks\n    total_num_blocks\n  }\n}\n","variables":{"limit":1},"operationName":"BlocksQuery"}"#;
    let client = reqwest::Client::new();
    let response = client
        .post(GRAPHQL_ENDPOINT)
        .body(query_body)
        .send()
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlocksAnalyticsResponse>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

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

                                    variant=ColorVariant::Transparent
                                />

                            </AnalyticsSmContainer>
                            <AnalyticsSmContainer>
                                <AnalyticsSimpleInfo
                                    label=convert_to_span("Blocks This Epoch".into())
                                    value=convert_to_span("...".to_string())

                                    variant=ColorVariant::Transparent
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
