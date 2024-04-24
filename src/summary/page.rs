use super::{functions::*, models::*};
use crate::{
    blocks::components::SummaryPageBlocksSection,
    common::{components::*, functions::*, search::*},
};
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <TitledSearchBar
            title="Mina Blockchain Explorer"
            subtext=convert_array_to_span(
                vec![
                    convert_to_span("Powered by ".to_string()).attr("class", "whitespace-pre"),
                    html::a()
                        .attr("href", "https://granola.team")
                        .attr(
                            "class",
                            "hover:text-granola-orange hover:underline hover:decoration-2",
                        )
                        .child("Granola")
                        .into(),
                ],
            )

            search_placeholder="Height: H348465; Txn Hash: 3N...; Address: B62..."
        />
        <Title text="Mina Blockchain Explorer | Search For Blocks"/>
        <PageContainer>
            {move || match blockchain_summary_resource.get() {
                Some(Ok(summary)) => view! { <SummaryGrid summary=Some(summary)/> },
                _ => view! { <SummaryGrid summary=None/> },
            }}
            <SummaryPageBlocksSection/>
        </PageContainer>
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
                            imgsrc="/assets/img/blockchain_length.svg"
                            id="blockchainLength"
                            label="Block Height"
                            value=Some(smry.blockchain_length.to_string())
                        />
                        <SummaryItem
                            imgsrc="/assets/img/circulating_supply.svg"
                            id="circulatingSupply"
                            label="Circulating Supply"
                            value=Some(format!("{:.2}", smry.circ_supply()))
                        />
                        <SummaryItem
                            imgsrc="/assets/img/epoch.svg"
                            id="epoch"
                            label="Epoch"
                            value=Some(smry.epoch.to_string())
                        />
                        <SummaryItem
                            imgsrc="/assets/img/total_currency.svg"
                            id="totalCurrency"
                            label="Total Currency"
                            value=Some(format!("{:.2}", smry.tot_currency()))
                        />
                    }
                }
                None => {
                    view! {
                        <SummaryItem
                            imgsrc="/assets/img/blockchain_length.svg"
                            id="blockchainLength"
                            label="Height"
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/assets/img/circulating_supply.svg"
                            id="circulatingSupply"
                            label="Circulating Supply"
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/assets/img/epoch.svg"
                            id="epoch"
                            label="Epoch"
                            value=None
                        />
                        <SummaryItem
                            imgsrc="/assets/img/total_currency.svg"
                            id="totalCurrency"
                            label="Total Currency"
                            value=None
                        />
                    }
                }
            }}

        </section>
    }
}
