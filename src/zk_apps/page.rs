use super::functions::*;
use crate::{
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ZkAppSpotlight() -> impl IntoView {
    view! {
        <Title text="ZK App Spotlight"/>
        <PageContainer>
            <SpotlightSection
                header="ZK App Spotlight".to_string()
                spotlight_items=vec![
                    SpotlightEntry {
                        label: String::from("Balance"),
                        any_el: Some(
                            wrap_in_pill(
                                decorate_with_currency_tag(
                                    "1324.593847562".to_string(),
                                    "mina".to_string(),
                                ),
                                ColorVariant::Green,
                            ),
                        ),
                        ..Default::default()
                    },
                    SpotlightEntry {
                        label: String::from("Receipt Chain Hash"),
                        any_el: Some(convert_to_span(generate_base58_string(44))),
                        copiable: true,
                    },
                    SpotlightEntry {
                        label: String::from("Voting For"),
                        any_el: Some(convert_to_span(generate_base58_string(44))),
                        copiable: true,
                    },
                    SpotlightEntry {
                        label: String::from("Ver. Key Hash"),
                        any_el: Some(convert_to_span(generate_base58_string(44))),
                        copiable: true,
                    },
                ]

                meta=Some(
                    format!(
                        "Last Active: {}",
                        print_time_since(&generate_random_datetime_within_days(1).to_string()),
                    ),
                )
                id=Some(generate_base58_string(44))
            >
                <ZKAppSymbol width=40/>
            </SpotlightSection>
        </PageContainer>
    }
}

#[component]
pub fn ZkAppTransactionsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_app_txn_data(1000);
    view! {
        <Title text="Transactions | ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK App Transactions" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}

#[component]
pub fn ZkAppsPage() -> impl IntoView {
    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);
    let data = stub_zk_apps_data(9000);
    view! {
        <Title text="ZK Apps | Search For ZK Apps"/>
        <PageContainer>
            <TableSection section_heading="ZK Apps" controls=|| ().into_view()>

                {move || {
                    let data = data.clone();
                    let pag = build_pagination(
                        data.len(),
                        records_per_page,
                        current_page.get(),
                        set_current_page,
                    );
                    let subset = get_subset(
                        &data.into_iter().map(Some).collect::<Vec<_>>(),
                        records_per_page,
                        current_page.get() - 1,
                    );
                    view! { <Table data=subset pagination=pag/> }
                }}

            </TableSection>
        </PageContainer>
    }
}
