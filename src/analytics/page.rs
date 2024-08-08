use super::{functions::*, models::*};
use crate::common::{
    components::*,
    functions::*,
    models::*,
    table::{TableColumn, TableSectionTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::create_query_signal;

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
    let (limit_sig, set_limit) = create_query_signal::<u64>("limit");
    let resource = create_resource(
        move || limit_sig.get(),
        move |limit| async move { load_snark_fees(limit).await },
    );
    let (data_sig, set_data) = create_signal(None);

    let table_columns = vec![
        TableColumn {
            column: "Metric".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "All SNARKs".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "SNARKs with non-zero fees".to_string(),
            ..Default::default()
        },
    ];

    create_effect(move |_| {
        set_data.set(Some(
            resource
                .get()
                .and_then(|res| res.ok())
                .map(|data| SnarkStatsContainer::from(data.data.blocks)),
        ));
    });

    view! {
        <Title text="Analytics | SNARKs"/>
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=resource.loading()
                section_heading="SNARK Fees of latest blocks"
                controls=move || {
                    view! {
                        <input
                            type="number"
                            on:input=move |ev| {
                                set_limit.set(event_target_value(&ev).parse::<u64>().ok())
                            }

                            disabled=resource.loading()
                            name="block-selection"
                            step=200
                            value=limit_sig.get()
                            max=5000
                            min=200
                            class="block h-8 text-base text-sm font-normal font-mono p-2 text-right border rounded-sm border-slate-400 focus:border-granola-orange"
                        />
                        <label
                            for="block-selection"
                            class="flex items-center h-8 text-base text-sm font-normal font-mono p-2 whitespace-nowrap"
                        >
                            "latest blocks"
                        </label>
                    }
                }
            />

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
