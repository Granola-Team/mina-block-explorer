use super::{functions::*, models::*};
use crate::{
    blocks::components::BlocksSection,
    common::{components::*, functions::format_mina},
};
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let blockchain_summary_resource = create_resource(|| (), |_| async move { load_data().await });

    view! {
        <Title text="Blocks | Search for blocks on Mina Blockchain"/>
        <PageContainer>
            {move || match blockchain_summary_resource.get() {
                Some(Ok(summary)) => view! { <SummaryGrid summary=Some(summary)/> },
                _ => view! { <SummaryGrid summary=None/> },
            }}
            <BlocksSection/>
        </PageContainer>
    }
}

#[component]
fn SummaryGrid(summary: Option<BlockchainSummary>) -> impl IntoView {
    view! {
        <section class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 xl:grid-cols-6 auto-rows-min gap-4 p-4 pt-0">
            <h1 class="h-0 w-0 overflow-hidden absolute">"Summary"</h1>
            <SummaryItem
                id="blockchainLength"
                label="Block Height"
                value=summary.as_ref().map(|s| s.blockchain_length.to_string())
            />
            <SummaryItem
                id="circulatingSupply"
                label="Circulating Supply"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.circ_supply().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

            <SummaryItem
                id="epoch"
                label="Epoch"
                value=summary.as_ref().map(|s| s.epoch.to_string())
            />
            <SummaryItem
                id="totalCurrency"
                label="Total Currency"
                value=summary
                    .as_ref()
                    .map(|s| {
                        format_mina(s.tot_currency().to_string())
                            .split('.')
                            .collect::<Vec<_>>()[0]
                            .to_string()
                    })
            />

        </section>
    }
}
