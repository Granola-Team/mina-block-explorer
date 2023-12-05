use leptos::*;

use crate::api_models::{BlockchainSummary, MyError};
use crate::summary_item::{SummaryItem, SummaryItemKind};

async fn load_data() -> Result<BlockchainSummary, MyError> {
    let response = reqwest::get("https://api.minaexplorer.com/summary")
        .await
        .map_err(|e| MyError::NetworkError(e.to_string()))?;

    if response.status().is_success() {
        let summary = response
            .json::<BlockchainSummary>()
            .await
            .map_err(|e| MyError::ParseError(e.to_string()))?;
        Ok(summary)
    } else {
        Err(MyError::NetworkError("Failed to fetch data".into()))
    }
}

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource: Resource<(), Result<BlockchainSummary, MyError>> =
        create_resource(|| (), |_| async move { load_data().await });

    view! {
        <h1>Summary</h1>
        {move || match blockchain_summary_resource.get() {
            None => view! {
                <div>"Loading..." </div>
            }.into_view(),
            Some(Ok(summary)) => view! { <SummaryGrid summary=summary /> },
            Some(Err(my_error)) => view! {
                <div> { format!("Error: {:#?}", my_error)}</div>
            }.into_view()
        }}
    }
}

#[component]
fn SummaryGrid(summary: BlockchainSummary) -> impl IntoView {
    view! {
        <section class="grid grid-cols-2 gap-1">
            <SummaryItem id="blockchainLength".to_string() label="Height".to_string() value={SummaryItemKind::Int64(summary.blockchainLength)} />
            <SummaryItem id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={SummaryItemKind::Str(summary.circulatingSupply)} />
            <SummaryItem id="epoch".to_string() label="Epoch".to_string() value={SummaryItemKind::Int16(summary.epoch)} />
            <SummaryItem id="slot".to_string() label="Slot".to_string() value={SummaryItemKind::Int16(summary.slot)} />
            <SummaryItem id="totalCurrency".to_string() label="Total Currency".to_string() value={SummaryItemKind::Str(summary.totalCurrency)} />
        </section>
    }
}

#[test]
fn fake_test() {
    assert_eq!(true, true)
}
