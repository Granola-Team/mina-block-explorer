use leptos::*;
use crate::common::components::*;
use super::components::*;

#[component]
pub fn BroadcastTransactionPage() -> impl IntoView {
    view! {
        <PageContainer>
            <AppSection>
                <AppHeading heading="Broadcast Signed Transaction".to_string() />
                <p class="px-8 text-sm">"Generate a new offline transaction using the Javascript SDK and submit the transaction to the network using the form below."</p>
                <BroadcastForm endpoint="https://api.minaexplorer.com/broadcast/transaction".to_string() />
            </AppSection>
        </PageContainer>
    }
}

#[component]
pub fn BroadcastDelegationPage() -> impl IntoView {
    view! {
        <PageContainer>
            <AppSection>
                <AppHeading heading="Broadcast Signed Delegation".to_string() />
                <p class="px-8 text-sm">"Generate a new offline delegation using the Javascript SDK and submit the transaction to the network using the form below."</p>
                <BroadcastForm endpoint="https://api.minaexplorer.com/broadcast/delegation".to_string() />
            </AppSection>
        </PageContainer>
    }
}