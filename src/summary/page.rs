use leptos::*;

use super::functions::*;
use super::models::*;
use crate::blocks::components::SummaryPageBlocksSection;
use crate::common::components::*;
use crate::common::search::*;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <TitledSearchBar
            title="Blockchain Explorer".to_string()
            subtext="Powered by Mina".to_string()
            search_placeholder="Exact search for block hash".to_string()
        />
        // <PageContainer>
        //     {move || match blockchain_summary_resource.get() {
        //         Some(Ok(summary)) => view! { <SummaryGrid summary=Some(summary)/> },
        //         _ => view! { <SummaryGrid summary=None/> },
        //     }}
        //     <SummaryPageBlocksSection/>
        // </PageContainer>
    }
}

#[component]
fn SummaryGrid(summary: Option<BlockchainSummary>) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 xl:grid-cols-4 md:col-start-2 auto-rows-min gap-4 py-4 pt-0">
            <h1 class="h-0 w-0 overflow-hidden absolute">"Summary"</h1>
            {match summary {
                Some(smry) => {
                    view! {
                        <SummaryItem
                            imgsrc="/img/blockchain_length.svg".to_string()
                            id="blockchainLength".to_string()
                            label="Height".to_string()
                            value=Some(smry.blockchain_length.to_string())
                        />
                        <SummaryItem
                            imgsrc="/img/circulating_supply.svg".to_string()
                            id="circulatingSupply".to_string()
                            label="Circulating Supply".to_string()
                            value=Some(format!("{:.2}", smry.circ_supply()))
                        />
                        <SummaryItem
                            imgsrc="/img/epoch.svg".to_string()
                            id="epoch".to_string()
                            label="Epoch".to_string()
                            value=Some(smry.epoch.to_string())
                        />
                        <SummaryItem
                            imgsrc="/img/total_currency.svg".to_string()
                            id="totalCurrency".to_string()
                            label="Total Currency".to_string()
                            value=Some(format!("{:.2}", smry.tot_currency()))
                        />
                    }
                }
                None => {
                    view! {
                        <SummaryItem
                            imgsrc="/img/blockchain_length.svg".to_string()
                            id="blockchainLength".to_string()
                            label="Height".to_string()
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/img/circulating_supply.svg".to_string()
                            id="circulatingSupply".to_string()
                            label="Circulating Supply".to_string()
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/img/epoch.svg".to_string()
                            id="epoch".to_string()
                            label="Epoch".to_string()
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/img/total_currency.svg".to_string()
                            id="totalCurrency".to_string()
                            label="Total Currency".to_string()
                            value=None
                        />
                    }
                }
            }}

        </section>
    }
}
