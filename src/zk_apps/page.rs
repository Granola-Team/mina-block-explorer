use super::functions::*;
use crate::{
    account_activity::models::AccountActivityQueryDirectionalTransactions,
    common::{components::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
};
use indoc::indoc;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ZkAppSpotlight() -> impl IntoView {
    let records_per_page = 5;
    let (current_txn_page, set_current_txn_page) = create_signal(1);
    let (current_fees_page, set_current_fees_page) = create_signal(1);
    let txn = stub_zk_app_txn_data(10);
    let fees = vec![AccountActivityQueryDirectionalTransactions {
        fee: Some(0.01_f64),
        counterparty: Some("B62qmQsEHcsPUs5xdtHKjEmWqqhUPRSF2GNmdguqnNvpEZpKftPC69f".to_string()),
        direction: Some("IN".to_string()),
        hash: Some("5JunUf7Niybx1d2CdLLZWL1D9wwtce5dBFM7nXsQ9GtiyopSh1Ee".to_string()),
        amount: Some(0.01_f64),
        date_time: Some(chrono::Utc::now()),
        height: Some(5822_i64),
        kind: Some("PAYMENT".to_string()),
        nonce: Some(1),
        failure_reason: None,
        memo: None,
        canonical: Some(true),
    }];
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
                        label: String::from("Total Txn"),
                        any_el: Some(convert_to_pill("52".to_string(), ColorVariant::Blue)),
                        ..Default::default()
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
            <TableSection section_heading="ZK App Details".to_string() controls=|| ().into_view()>
                <SpotlightTable>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"Permissions :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"{
    "access":"none"
    "editActionState":"proof"
    "editState":"proof"
    "incrementNonce":"signature"
    "receive":"none"
    "send":"proof"
    "setDelegate":"signature"
    "setPermissions":"signature"
    "setTiming":"signature"
    "setTokenSymbol":"signature"
    "setVerificationKey":"signature"
    "setVotingFor":"signature"
    "setZkAppUri":"signature"
}"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"Events :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    0:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    1:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    2:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    3:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
    4:"25079927036070901246064867767436987657692091363973573142121686150614948079097"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                    <ZkAppDetailTr>
                        <ZkAppDetailTh>"App State :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    0:"1"
    1:"0"
    2:"0"
    3:"0"
    4:"0"
    5:"0"
    6:"0"
    7:"0"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                </SpotlightTable>
            </TableSection>
            <SubSectionContainer>
                <AppSubSection
                    heading="App Transactions".to_string()
                    position=SubSectionPosition::Left
                >
                    {move || {
                        let data = txn.clone();
                        let pag = build_pagination(
                            data.len(),
                            records_per_page,
                            current_txn_page.get(),
                            set_current_txn_page,
                        );
                        let subset = get_subset(
                            &data.into_iter().map(Some).collect::<Vec<_>>(),
                            records_per_page,
                            current_txn_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }}

                </AppSubSection>
                <AppSubSection
                    heading="Fee Payments".to_string()
                    position=SubSectionPosition::Right
                >
                    {move || {
                        let data = fees.clone();
                        let pag = build_pagination(
                            data.len(),
                            records_per_page,
                            current_fees_page.get(),
                            set_current_fees_page,
                        );
                        let subset = get_subset(
                            &data.into_iter().map(Some).collect::<Vec<_>>(),
                            records_per_page,
                            current_fees_page.get() - 1,
                        );
                        view! { <Table data=subset pagination=pag/> }
                    }}

                </AppSubSection>
            </SubSectionContainer>
        </PageContainer>
    }
}

#[component]
fn ZkAppDetailTd(children: Children) -> impl IntoView {
    view! {
        <td class="flex justify-start items-center m-1 p-1 text-left text-xs md:text-sm whitespace-nowrap">
            {children()}
        </td>
    }
}

#[component]
fn ZkAppDetailTr(children: Children) -> impl IntoView {
    view! { <tr class="w-full flex flex-col lg:flex-row justify-start">{children()}</tr> }
}

#[component]
fn ZkAppDetailTh(children: Children) -> impl IntoView {
    view! {
        <th class="flex justify-start items-start m-1 p-1 text-xs md:text-sm whitespace-nowrap w-36 md:w-40 min-w-36 md:min-w-40 font-normal text-slate-400">
            {children()}
        </th>
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
