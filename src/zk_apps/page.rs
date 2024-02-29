use crate::common::components::*;
use crate::common::models::*;
use crate::common::functions::*;
use crate::common::spotlight::*;
use crate::icons::*;
use leptos::*;

#[component]
pub fn ZkAppTransactionSpotlightPage() -> impl IntoView {
    let spotlight_items = vec![SpotlightEntry {
        label: "Transaction Type".to_string(),
        any_el: Some(convert_to_pill("ZK".to_string(), PillVariant::Green)),
        ..Default::default()
    },SpotlightEntry {
        label: "Transaction Hash".to_string(),
        any_el: Some(convert_to_span("5JvEERgjGA3dYZSKNAz7DnDVNgERvJQrek3tCdMhTUsacov6LUzy".to_string())),
        copiable: true,
        ..Default::default()
    },SpotlightEntry {
        label: "Block Height".to_string(),
        any_el: Some(convert_to_pill("7,326".to_string(), PillVariant::Grey)),
        ..Default::default()
    },SpotlightEntry {
        label: "Block State Hash".to_string(),
        any_el: Some(convert_to_span("3NLoTnbvaSwU6zNwxVQd8vR6UcuEDrf9YuQusbjkgNzjEAHjwboG".to_string())),
        copiable: true,
        ..Default::default()
    },SpotlightEntry {
        label: "Fee Payer".to_string(),
        any_el: Some(convert_to_span("B62qpGSaBUHzKExDXp2N3ZPNPtFMFFXjSuAB84h4DSks12PWsRq5SEB".to_string())),
        copiable: true,
        ..Default::default()
    },SpotlightEntry {
        label: "Account Updates".to_string(),
        any_el: Some(convert_to_pill("3".to_string(), PillVariant::Grey)),
        ..Default::default()
    }];

    let account_updates_1 = vec![SpotlightEntry {
        label: "Balance Change".to_string(),
        any_el: Some(convert_to_pill("+20 MINA".to_string(), PillVariant::Green)),
        ..Default::default()
    },SpotlightEntry {
        label: "Increment Nonce".to_string(),
        any_el: Some(convert_to_pill("true".to_string(), PillVariant::Grey)),
        ..Default::default()
    },SpotlightEntry {
        label: "Token ID".to_string(),
        any_el: Some(convert_to_span("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf".to_string())),
        copiable: true,
        ..Default::default()
    },SpotlightEntry {
        label: "Call Data".to_string(),
        any_el: Some(convert_to_pill("0".to_string(), PillVariant::Grey)),
        ..Default::default()
    },SpotlightEntry {
        label: "Call Depth".to_string(),
        any_el: Some(convert_to_pill("1".to_string(), PillVariant::Grey)),
        ..Default::default()
    },SpotlightEntry {
        label: "Use Full Committment".to_string(),
        any_el: Some(convert_to_pill("No".to_string(), PillVariant::Grey)),
        ..Default::default()
    },SpotlightEntry {
        label: "appState".to_string(),
        any_el: Some(json_to_code(r#"[
            "13085319543788982998999669060227968584120410722425376027756703205043792631731",
            "88814049655838941284774570817345763621809698732252711808042102595406818641",
            "525481201097986652723544789857104441"
         ]"#.to_string())),
        is_full_width: true,
        ..Default::default()
    },SpotlightEntry {
        label: "Permissions".to_string(),
        any_el: Some(json_to_code(r#"{
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
         }"#.to_string())),
        is_full_width: true,
        ..Default::default()
    }];

    view! {
        <PageContainer>
            <SpotlightSection
                header="[zk] Transaction Spotlight".to_string()
                spotlight_items
                id=Some("3NK8nzfotTNSUopF4oEzJUHJ2EeLATBDnMRRgaqaTfR3zpfHK2yo".to_string())
                meta=Some("2024-02-28 20:45:00 UTC (8 minutes ago)".to_string())
            >
                <ZKAppSymbol width=40/>
            </SpotlightSection>
            <AppSection>
                <AppHeading heading="Account Update #1".to_string()/>
                <SpotlightTable spotlight_items=account_updates_1/>
            </AppSection>
        </PageContainer>
    }
}
