use super::components::*;
use crate::common::{components::*, models::*};
use leptos::*;

#[component]
pub fn BroadcastTransactionPage() -> impl IntoView {
    view! {
        <PageContainer>
            <AppSection>
                <AppHeading heading="Broadcast Signed Transaction".to_string()/>
                <p class="px-8 text-sm">
                    "Generate a new offline transaction using the Javascript SDK and submit the transaction to the network using the form below."
                </p>
                <BroadcastForm endpoint="https://api.minaexplorer.com/broadcast/transaction"
                    .to_string()/>
            </AppSection>
            <AppSection>
                <AppHeading heading="Sample Payment".to_string()/>
                <Sample>
                    r#"{
                    "publicKey": "B62qrPN5Y5...",
                    "signature": {
                    "field": "1912885630...",
                    "scalar": "48899066..."
                    },
                    "payload": {
                    "to": "B62qqUzKC9H...",
                    "from": "B62qrPN5Y5y...",
                    "fee": "100000000",
                    "amount": "1000000000",
                    "nonce": "305",
                    "memo": "SDK payment",
                    "validUntil": "4294967295"
                    }
                    }
                    "#
                </Sample>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn BroadcastDelegationPage() -> impl IntoView {
    view! {
        <PageContainer>
            <AppSection>
                <AppHeading heading="Broadcast Signed Delegation".to_string()/>
                <p class="px-8 text-sm">
                    "Generate a new offline delegation using the Javascript SDK and submit the transaction to the network using the form below."
                </p>
                <BroadcastForm endpoint="https://api.minaexplorer.com/broadcast/delegation"
                    .to_string()/>
            </AppSection>
            <AppSection>
                <AppHeading heading="Sample Delegation Transaction".to_string()/>
                <Sample>
                    r#"{
                    "publicKey": "B62qrPN5Y5...",
                    "signature": {
                    "field": "1912885630...",
                    "scalar": "48899066..."
                    },
                    "payload": {
                    "to": "B62qqUzKC9H...",
                    "from": "B62qrPN5Y5y...",
                    "fee": "100000000",
                    "nonce": "305",
                    "memo": "SDK delegation",
                    "validUntil": "4294967295"
                    }
                    }               "#
                </Sample>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn BroadcastFromLedgerPage() -> impl IntoView {
    view! {
        <PageContainer>
            <AppSection>
                <AppHeading heading="Broadcast Signed Transaction From Ledger".to_string()/>
                <p class="px-8 text-sm">
                    "Generate an offline transaction using the Ledger and submit the transaction to the network using the form below. This works for both payment and delegation transaction types."
                </p>
                <BroadcastForm endpoint="https://api.minaexplorer.com/broadcast/transaction"
                    .to_string()/>
            </AppSection>
            <AppSection>
                <AppHeading heading="Sample Ledger Payment".to_string()/>
                <Sample>
                    r#"{
                    "signature": "389ac7d4077f3d485c1494782870979faa222cd906b25b2687333a92f41e40b925adb08705eddf2a7098e5ac9938498e8a0ce7c70b25ea392f4846b854086d43",
                    "payment": {
                    "to": "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
                    "from": "B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV",
                    "fee": "10000000",
                    "token": "1",
                    "nonce": "0",
                    "memo": null,
                    "amount": "1000000000",
                    "valid_until": "4294967295"
                    },
                    "stake_delegation": null,
                    "create_token": null,
                    "create_token_account": null,
                    "mint_tokens": null
                    }               "#
                </Sample>
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn DelegationTabbedPage() -> impl IntoView {
    let tabs = vec![
        NavEntry {
            href: "/broadcast/transaction".to_string(),
            text: "Transaction".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/broadcast/delegation".to_string(),
            text: "Delegation".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
        NavEntry {
            href: "/broadcast/ledger".to_string(),
            text: "Ledger".to_string(),
            icon: NavIcon::Transactions,
            ..Default::default()
        },
    ];
    view! { <TabbedPage tabs/> }
}
