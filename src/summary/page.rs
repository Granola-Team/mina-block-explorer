use super::{components::*, functions::*, models::BlockchainSummary};
use crate::{
    blocks::components::BlocksSection,
    common::{components::*, constants::*},
    summary::models::{BlockchainStat, BlockchainStatData, BlockchainStatResponse},
};
use leptos::*;
use leptos_meta::Title;
use leptos_use::{
    storage::*, use_document_visibility, use_interval, utils::JsonCodec, UseIntervalReturn,
};
use web_sys::VisibilityState;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (stat_sig, _, _) = use_local_storage::<BlockchainStat, JsonCodec>("blockchain-stat");

    view! {
        <Title text="Blocks | Search for blocks on Mina Blockchain" />
        <PageContainer>
            {move || {
                view! { <SummaryGrid summary=Some(summary_sig.get()) stat=Some(stat_sig.get()) /> }
            }} <BlocksSection />
        </PageContainer>
    }
}

#[component]
pub fn SummaryLocalStorage() -> impl IntoView {
    let (summary_sig, set_summary, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (_, set_stat, _) = use_local_storage::<BlockchainStat, JsonCodec>("blockchain-stat");
    let visibility = use_document_visibility();
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || (counter.get()),
        move |_| async move {
            if visibility.get() == VisibilityState::Visible {
                load_data().await
            } else {
                logging::log!("Document not visible. Data polling skipped for summary endpoint.");
                Ok(summary_sig.get())
            }
        },
    );

    let unique_blocks_producers_resource = create_resource(
        move || (counter.get()),
        move |_| async move {
            if visibility.get() == VisibilityState::Visible {
                load_block_producers_stat(10000).await
            } else {
                logging::log!("Document not visible. Data polling skipped for summary endpoint.");
                Ok(BlockchainStatResponse {
                    data: BlockchainStatData { blocks: vec![] },
                })
            }
        },
    );

    create_effect(move |_| {
        if let Some(blockchain_summary) = resource.get().and_then(|res| res.ok()) {
            set_summary.set(blockchain_summary);
        }
        if let Some(blockchain_stat) = unique_blocks_producers_resource
            .get()
            .and_then(|res| res.ok())
        {
            logging::log!("{:#?}", blockchain_stat);
            if let Some(block) = blockchain_stat.data.blocks.first() {
                logging::log!("{:#?}", block.clone());
                set_stat.set(block.clone());
            }
        }
    });

    ().into_view()
}
