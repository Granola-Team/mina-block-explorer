use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::api_models::MyError;
use crate::summary_item::{SummaryItem,SummaryItemKind};

#[derive(Params, PartialEq)]
struct URLParams {
    id: Option<String>
}


#[derive(Params, PartialEq)]
struct QueryParams {
    f: Option<bool>
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
#[allow(non_snake_case)]
struct AccountSummary {
    publicKey: String,
    nonce: u32,
    receiptChainHash: String,
    delegate: String,
    votingFor: String,
    totalTx: u32,
    countPendingTransactions: u32,
    username: String,
    balance: AccountBalance
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AccountResponse {
    account: AccountSummary,
}

async fn load_data(id: &str) -> Result<AccountResponse, MyError> {
    let response = reqwest::get(String::from("https://api.minaexplorer.com/accounts/") + &id).await;

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

async fn load_fixture_data() -> Result<AccountResponse, MyError> {
    let fixture_account_balance = AccountBalance {
        total: String::from("358.334")
    };
    let fixture_account_summary = AccountSummary {
        publicKey: String::from("B62qpWaQoQoPL5AGta7Hz2DgJ9CJonpunjzCGTdw8KiCCD1hX8fNHuR"),
        balance: fixture_account_balance,
        nonce: 36,
        countPendingTransactions: 1,
        receiptChainHash: String::from("2mzubGsgiNhFjoJAqzYsVi97yMUMSwydjMtzybB6wb4SYgBhUY9v"),
        delegate: String::from("B62qopHVr6nGCsQgrvsBsoxDm1E5CEdMkDSN3jneRnxKpR5iiXnTbas"),
        votingFor: String::from("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x"),
        totalTx: 78,
        username: String::from("OKEX Wallet")
    };
    let fixture_account_response = AccountResponse {
        account: fixture_account_summary
    };

    Ok(fixture_account_response)
}

#[component]
pub fn AccountSummary() -> impl IntoView {
    let url_params = use_params::<URLParams>();
    let q_params = use_query::<QueryParams>();

    let public_key_fn = move || {
        url_params.with(|url_params| {
            url_params
                .as_ref()
                .map(|url_params| url_params.id.clone())
                .unwrap_or_default()
        })
    };

    let use_fixture_data_fn = move || {
        q_params.with(|q_params| {
            q_params
                .as_ref()
                .map(|q_params| q_params.f.clone())
                .unwrap_or_default()
        })
    };

    let public_key = match public_key_fn() {
        Some(pc) => pc,
        None => todo!(),
    };

    let fixture_data_switch: bool = match use_fixture_data_fn() {
        Some(f) => f,
        None => false
    };

    let resource: Resource<(), Result<AccountResponse, MyError>> = {
        let public_key_clone = public_key.clone();
        create_resource(
            || (),
            move |_| {
                let public_key_for_async = public_key_clone.clone();
                async move { 
                    match fixture_data_switch {
                        true => load_fixture_data().await,
                        false => load_data(&public_key_for_async).await 
                    }
                }
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
            <SummaryItem id="publicKey".to_string() label="Public Key".to_string() value={SummaryItemKind::StrValue(summary.account.publicKey)} />
            <SummaryItem id="username".to_string() label="Username".to_string() value={SummaryItemKind::StrValue(summary.account.username)} />
            <SummaryItem id="balance".to_string() label="Balance".to_string() value={SummaryItemKind::Float64Value(summary.account.balance.total())} />
            <SummaryItem id="nonce".to_string() label="Nonce".to_string() value={SummaryItemKind::Int32Value(summary.account.nonce)} />
            <SummaryItem id="receiptChainHash".to_string() label="Receipt Chain Hash".to_string() value={SummaryItemKind::StrValue(summary.account.receiptChainHash)} />
            <SummaryItem id="delegate".to_string() label="Delegate".to_string() value={SummaryItemKind::StrValue(summary.account.delegate)} />
            <SummaryItem id="votingFor".to_string() label="Voting For".to_string() value={SummaryItemKind::StrValue(summary.account.votingFor)} />
            <SummaryItem id="pendingTransactions".to_string() label="Pending Transactions".to_string() value={SummaryItemKind::Int32Value(summary.account.countPendingTransactions)} />
        </section>
    }
}
