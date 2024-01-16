use leptos::*;
use serde::{Deserialize, Serialize};

use crate::blocks::components::SummaryPageBlocksSection;
use crate::common::components::*;
use crate::common::models::MyError;

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
        self.circulating_supply
            .trim()
            .parse()
            .expect("Cannot parse circulating_supply")
    }
    fn tot_currency(&self) -> f64 {
        self.total_currency
            .trim()
            .parse()
            .expect("Cannot parse total_currency")
    }
}

#[test]
fn test_parsing_floats() {
    let bs = BlockchainSummary {
        circulating_supply: "2345345.4312431243".to_owned(),
        blockchain_length: 314394,
        epoch: 67,
        slot: 4194,
        total_currency: "1105297372.840039233".to_owned(),
    };
    assert_eq!(bs.circ_supply(), 2345345.4312431243);
    assert_eq!(bs.tot_currency(), 1_105_297_372.840_039_3)
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
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        {move || match blockchain_summary_resource.get() {
            Some(Ok(summary)) => view! { <SummaryGrid summary=summary /> },
            _ => view! { <span /> }.into_view()
        }}
        <SummaryPageBlocksSection />
    }
}

#[component]
fn SummaryGrid(summary: BlockchainSummary) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 xl:grid-cols-4 md:col-start-2 auto-rows-min gap-4 py-4 pt-0">
            <h1 class="h-0 w-0 overflow-hidden absolute">Summary</h1>
            <SummaryItem imgsrc="/img/blockchain_length.svg".to_string() id="blockchainLength".to_string() label="Height".to_string() value={SummaryItemKind::Int64(summary.blockchain_length)} />
            <SummaryItem imgsrc="/img/circulating_supply.svg".to_string() id="circulatingSupply".to_string() label="Circulating Supply".to_string() value={SummaryItemKind::Float64(summary.circ_supply())} />
            <SummaryItem imgsrc="/img/epoch.svg".to_string() id="epoch".to_string() label="Epoch".to_string() value={SummaryItemKind::Int16(summary.epoch)} />
            <SummaryItem imgsrc="/img/total_currency.svg".to_string() id="totalCurrency".to_string() label="Total Currency".to_string() value={SummaryItemKind::Float64(summary.tot_currency())} />
        </section>
    }
}
