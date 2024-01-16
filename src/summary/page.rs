use leptos::*;

use super::functions::*;
use super::models::*;
use crate::blocks::components::SummaryPageBlocksSection;
use crate::common::components::*;
use crate::common::functions::*;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <PageContainer>
            {move || match blockchain_summary_resource.get() {
                Some(Ok(summary)) => view! { <SummaryGrid summary=summary /> },
                _ => view! { <span /> }.into_view()
            }}
            <SummaryPageBlocksSection />
        </PageContainer>
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
