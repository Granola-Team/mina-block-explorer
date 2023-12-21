use leptos::*;
use leptos_router::*;

use super::models::*;
use super::functions::*;

use crate::api_models::MyError;
use crate::summary_item::{SummaryItem, SummaryItemKind};


#[component]
pub fn AccountSummaryPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let public_key = memo_params_map.with(|params| params.get("id").cloned()).unwrap_or_default();

    let resource: Resource<(), Result<AccountResponse, MyError>> = {
        create_resource(
            || (),
            move |_| {
                let public_key_for_async = public_key.clone();
                async move { load_data(&public_key_for_async).await }
            },
        )
    };

    view! {
        <h1>"Account Summary"</h1>
        {move || match resource.get() {
            None => view! { <div>"Loading"</div>}.into_view(),
            Some(Ok(res)) => view! { <AccountSummarySection summary=res /> },
            Some(Err(err)) => view! { <div>{format!("{:#?}", err)}</div>}.into_view()
        }}
    }
}

#[component]
fn AccountSummarySection(summary: AccountResponse) -> impl IntoView {
    view! {
        <section class="grid grid-cols-2 gap-1">
            <SummaryItem id="publicKey".to_string() label="Public Key".to_string() value={SummaryItemKind::Str(summary.account.public_key)} />
            <SummaryItem id="username".to_string() label="Username".to_string() value={SummaryItemKind::Str(summary.account.username)} />
            <SummaryItem id="balance".to_string() label="Balance".to_string() value={SummaryItemKind::Float64(summary.account.balance.total())} />
            <SummaryItem id="nonce".to_string() label="Nonce".to_string() value={SummaryItemKind::Int32(summary.account.nonce)} />
            <SummaryItem id="receiptChainHash".to_string() label="Receipt Chain Hash".to_string() value={SummaryItemKind::Str(summary.account.receipt_chain_hash)} />
            <SummaryItem id="delegate".to_string() label="Delegate".to_string() value={SummaryItemKind::Str(summary.account.delegate)} />
            <SummaryItem id="votingFor".to_string() label="Voting For".to_string() value={SummaryItemKind::Str(summary.account.voting_for)} />
            <SummaryItem id="pendingTransactions".to_string() label="Pending Transactions".to_string() value={SummaryItemKind::Int32(summary.account.count_pending_transactions)} />
        </section>
    }
}
