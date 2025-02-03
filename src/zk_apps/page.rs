use super::functions::*;
use crate::{
    account_activity::models::AccountActivityQueryDirectionalTransactions,
    common::{
        components::*,
        constants::{PAYMENT_TYPE, TABLE_ROW_LIMIT},
        functions::*,
        models::*,
        spotlight::*,
        table::*,
    },
    icons::*,
};
use indoc::indoc;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ZkAppSpotlight() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    let (txn_sig, _) = create_signal(Some(stub_zk_app_txn_data(10)));
    let (loading_sig, _) = create_signal(false);
    let (fees_sig, _) = create_signal(Some(vec![Some(
        AccountActivityQueryDirectionalTransactions {
            fee: Some(0.01_f64),
            counterparty: Some(
                "B62qmQsEHcsPUs5xdtHKjEmWqqhUPRSF2GNmdguqnNvpEZpKftPC69f".to_string(),
            ),
            direction: Some("IN".to_string()),
            hash: Some("5JunUf7Niybx1d2CdLLZWL1D9wwtce5dBFM7nXsQ9GtiyopSh1Ee".to_string()),
            amount: Some(0.01_f64),
            date_time: Some(chrono::Utc::now()),
            height: Some(5822_u64),
            kind: Some(PAYMENT_TYPE.to_string()),
            nonce: Some(1),
            failure_reason: None,
            memo: None,
            canonical: Some(true),
        },
    )]));
    view! {
        <Title text="zkApp Spotlight" />
        <PageContainer>
            <SpotlightSection
                header="zkApp Spotlight".to_string()
                spotlight_items=vec![
                    SpotlightEntry {
                        label: String::from("Balance"),
                        any_el: Some(decorate_with_mina_tag("1324.593847562".to_string())),
                        ..Default::default()
                    },
                    SpotlightEntry {
                        label: String::from("Total Txn"),
                        any_el: Some(convert_to_pill("52".to_string(), ColorVariant::Blue)),
                        ..Default::default()
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
                        convert_to_local_timezone_formatted(
                            &generate_random_datetime_within_days(1).to_string(),
                        ),
                    ),
                )

                id=Some(generate_base58_string(44))
            >
                <ZKAppSymbol width=40 />
            </SpotlightSection>
            <TableSection metadata=metadata.into() section_heading="zkApp Details".to_string()>

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
            <TableSectionTemplate
                table_columns=vec![
                    TableColumn::<AnySort> {
                        column: "Account".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Balance".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Commands".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Delegate".to_string(),
                        ..Default::default()
                    },
                ]

                data_sig=txn_sig
                is_loading=loading_sig.into()
                section_heading="zkApp Commands"
            />

            <TableSectionTemplate
                table_columns=vec![
                    TableColumn::<AnySort> {
                        column: "Prover".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "txn Hash".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Date".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Account Updates".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Updated Accounts".to_string(),
                        ..Default::default()
                    },
                    TableColumn::<AnySort> {
                        column: "Fee".to_string(),
                        ..Default::default()
                    },
                ]

                data_sig=fees_sig
                is_loading=loading_sig.into()
                section_heading="zkApp Internal Commands"
            />

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
    let (data_sig, _) = create_signal(Some(stub_zk_app_txn_data(TABLE_ROW_LIMIT)));
    let (loading_sig, _) = create_signal(false);
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Account".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Balance".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Commands".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Delegate".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Counterparties".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Fee".to_string(),
            ..Default::default()
        },
    ];
    view! {
        <Title text="Commands | zkApps" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=loading_sig.into()
                section_heading="zkApp Commands"
            />

        </PageContainer>
    }
}

#[component]
pub fn ZkAppsPage() -> impl IntoView {
    let (data_sig, _) = create_signal(Some(stub_zk_apps_data(TABLE_ROW_LIMIT)));
    let (loading_sig, _) = create_signal(false);
    let table_columns: Vec<TableColumn<AnySort>> = vec![
        TableColumn {
            column: "Account".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Balance".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Commands".to_string(),
            ..Default::default()
        },
        TableColumn {
            column: "Delegate".to_string(),
            ..Default::default()
        },
    ];
    view! {
        <Title text="zkApps | Search for zkApps" />
        <PageContainer>
            <TableSectionTemplate
                table_columns
                data_sig
                is_loading=loading_sig.into()
                section_heading="zkApps"
            />

        </PageContainer>
    }
}

#[component]
pub fn ZkAppTransactionSpotlightPage() -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata::default()));
    let spotlight_items = vec![
        SpotlightEntry {
            label: "Transaction Type".to_string(),
            any_el: Some(convert_to_pill("ZK".to_string(), ColorVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Txn Hash".to_string(),
            any_el: Some(convert_to_span(
                "5JvEERgjGA3dYZSKNAz7DnDVNgERvJQrek3tCdMhTUsacov6LUzy".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Block Height".to_string(),
            any_el: Some(convert_to_pill("7,326".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Block State Hash".to_string(),
            any_el: Some(convert_to_span(
                "3NLoTnbvaSwU6zNwxVQd8vR6UcuEDrf9YuQusbjkgNzjEAHjwboG".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Fee Payer".to_string(),
            any_el: Some(convert_to_span(
                "B62qpGSaBUHzKExDXp2N3ZPNPtFMFFXjSuAB84h4DSks12PWsRq5SEB".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Account Updates".to_string(),
            any_el: Some(convert_to_pill("3".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
    ];

    let account_updates_1 = vec![
        SpotlightEntry {
            label: "Balance Change".to_string(),
            any_el: Some(convert_to_pill("+20 MINA".to_string(), ColorVariant::Green)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Increment Nonce".to_string(),
            any_el: Some(convert_to_pill("true".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Token ID".to_string(),
            any_el: Some(convert_to_span(
                "wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf".to_string(),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Call Data".to_string(),
            any_el: Some(convert_to_pill("0".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Call Depth".to_string(),
            any_el: Some(convert_to_pill("1".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Use Full Com.".to_string(),
            any_el: Some(convert_to_pill("No".to_string(), ColorVariant::Grey)),
            ..Default::default()
        },
    ];

    view! {
        <PageContainer>
            <SpotlightSection
                header="[zk] Command Spotlight"
                spotlight_items
                id=Some("3NK8nzfotTNSUopF4oEzJUHJ2EeLATBDnMRRgaqaTfR3zpfHK2yo".to_string())
                meta=Some("2024-02-28 20:45:00 UTC (8 minutes ago)".to_string())
            >
                <ZKAppSymbol width=40 />
            </SpotlightSection>
            <TableSection metadata=metadata.into() section_heading="Account Update #1">

                <SpotlightTable>
                    {account_updates_1
                        .into_iter()
                        .map(|entry| {
                            view! {
                                <ZkAppDetailTr>
                                    <ZkAppDetailTh>{entry.label}</ZkAppDetailTh>
                                    <ZkAppDetailTd>{entry.any_el}</ZkAppDetailTd>
                                </ZkAppDetailTr>
                            }
                        })
                        .collect::<Vec<_>>()} <ZkAppDetailTr>
                        <ZkAppDetailTh>"App State :"</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"[
    "13085319543788982998999669060227968584120410722425376027756703205043792631731",
    "88814049655838941284774570817345763621809698732252711808042102595406818641",
    "525481201097986652723544789857104441"
]"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr> <ZkAppDetailTr>
                        <ZkAppDetailTh>"Permissions: "</ZkAppDetailTh>
                        <ZkAppDetailTd>
                            <CodeBlock>

                                {
                                    indoc! {
                                        r#"{
    "access":"",
    "editActionState":"proof",
    "editState":"proof",
    "incrementNonce":"signature",
    "receive":"none",
    "send":"proof",
    "setDelegate":"signature",
    "setPermissions":"signature",
    "setTiming":"",
    "setTokenSymbol":"signature",
    "setVerificationKey":"signature",
    "setVotingFor":"signature",
    "setZkAppUri":"signature"
}"#
                                    }
                                }

                            </CodeBlock>
                        </ZkAppDetailTd>
                    </ZkAppDetailTr>
                </SpotlightTable>
            </TableSection>
        </PageContainer>
    }
}
