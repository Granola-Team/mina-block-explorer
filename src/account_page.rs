use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::api_models::MyError;
use crate::summary_item::{SummaryItem, SummaryItemKind};

#[derive(Params, PartialEq)]
struct URLParams {
    id: Option<String>,
}

#[derive(Params, PartialEq)]
struct QueryParams {
    f: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AccountBalance {
    total: String,
}

impl AccountBalance {
    fn total(&self) -> f64 {
        self.total.trim().parse().expect("Cannot parse total")
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    public_key: String,
    nonce: u32,
    receipt_chain_hash: String,
    delegate: String,
    voting_for: String,
    total_tx: u32,
    count_pending_transactions: u32,
    username: String,
    balance: AccountBalance,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AccountResponse {
    account: AccountSummary,
}

async fn load_data(id: &str) -> Result<AccountResponse, MyError> {
    let response = reqwest::get(format!("https://api.minaexplorer.com/accounts/{}", id)).await;

    match response {
        Ok(res) => match res.json::<AccountResponse>().await {
            Ok(account) => Ok(account),
            Err(_) => Err(MyError::ParseError(String::from(
                "Error deserializing JSON",
            ))),
        },
        Err(_) => Err(MyError::NetworkError(String::from("API error"))),
    }
}

#[component]
pub fn AccountSummary() -> impl IntoView {
    let url_params = use_params::<URLParams>();

    let public_key_fn = move || {
        url_params.with(|url_params| {
            url_params
                .as_ref()
                .map(|url_params| url_params.id.clone())
                .unwrap_or_default()
        })
    };

    let public_key = match public_key_fn() {
        Some(pc) => pc,
        None => todo!(),
    };

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
