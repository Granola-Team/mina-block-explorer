use leptos::*;
use serde::{Deserialize, Serialize};

use crate::api_models::{MyError};
use crate::summary_item::{SummaryItem, SummaryItemKind};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainSummary {
    pub blockchain_length: u64,
    pub circulating_supply: String,
    pub epoch: u16,
    pub slot: u16,
    pub total_currency: String,
}

impl BlockchainSummary {
    fn circ_supply(&self) -> f64 {
        self.circulating_supply.trim().parse().expect("Cannot parse circulating_supply")
    }
    fn tot_currency(&self) -> f64 {
        self.total_currency.trim().parse().expect("Cannot parse total_currency")
    }
}

#[test]
fn test_parsing_floats(){
    let bs = BlockchainSummary {
        circulating_supply: "2345345.4312431243".to_owned(),
        blockchain_length: 314394,
        epoch: 67,
        slot: 4194,
        total_currency: "1105297372.840039233".to_owned(),
    };
    assert_eq!(bs.circ_supply(), 2345345.4312431243);
    assert_eq!(bs.tot_currency(), 1105297372.840039233)
}


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
        <h1 class="h-0 w-0 overflow-hidden">Summary</h1>
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
        <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 md:p-0 pt-0">
            <SummaryItem id="blockchainLength".to_string() label="Height".to_string() value={SummaryItemKind::Int64(summary.blockchain_length)} />
            <SummaryItem id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={SummaryItemKind::Float64(summary.circ_supply())} />
            <SummaryItem id="epoch".to_string() label="Epoch".to_string() value={SummaryItemKind::Int16(summary.epoch)} />
            <SummaryItem id="slot".to_string() label="Slot".to_string() value={SummaryItemKind::Int16(summary.slot)} />
            <SummaryItem id="totalCurrency".to_string() label="Total Currency".to_string() value={SummaryItemKind::Float64(summary.tot_currency())} />
        </section>
    }
}
