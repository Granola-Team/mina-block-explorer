use leptos::*;
use leptos_router::*;

use super::models::*;
use super::functions::*;
use super::components::*;

use crate::blocks::components::AccountOverviewBlocksTable;
use crate::common::models::MyError;
use crate::snarks::components::AccountOverviewSnarkJobTable;
use crate::transactions::transactions_page::TransactionsSection;
use crate::common::components::*;

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
        {move || match resource.get() {
            Some(Ok(res)) =>{
                let pk =res.account.public_key.clone();
                view! {
                    <section class="@container md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section p-0 md:p-4 mb-2">
                        <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Account Overview"</h1>
                        <AccountSummarySubsection summary_items=get_summary_items(res.account.clone()) username=res.account.username public_key=pk.clone() />
                    </section>
                    <TransactionsSection public_key=Some(pk.clone())/>
                    <div class="md:col-start-2 md:col-end-3 grid grid-cols-1 md:grid-cols-2 gap-4">
                        <section class="md:col-start-1 md:col-end-2 md:rounded-lg bg-table-section">
                            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Snark Jobs"</h1>    
                            <AccountOverviewSnarkJobTable public_key=Some(pk.clone())/>
                        </section>
                        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section">
                            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">"Block Production"</h1>    
                            <AccountOverviewBlocksTable public_key=Some(pk) />
                        </section>
                    </div>
                }.into_view()
            },
            _ => view! { <span/>  }.into_view()
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
