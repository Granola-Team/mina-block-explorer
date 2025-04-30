use super::{
    components::*,
    functions::*,
    models::{BlockchainSummary, ChainSummary},
};
use crate::{
    blocks::components::BlocksSection,
    common::{components::*, constants::*},
    summary::models::{BlockchainStat, BlockchainStatData, BlockchainStatResponse},
};
use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_meta::Title;
use leptos_use::{storage::*, use_document_visibility, use_interval, UseIntervalReturn};
use std::collections::HashMap;
use web_sys::VisibilityState;

#[component]
pub fn SummaryPage() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (stat_sig, _, _) = use_local_storage::<BlockchainStat, JsonSerdeCodec>("blockchain-stat");

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
        use_local_storage::<BlockchainSummary, JsonSerdeCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
    let (_, set_stat, _) = use_local_storage::<BlockchainStat, JsonSerdeCodec>("blockchain-stat");
    let visibility = use_document_visibility();
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || counter.get(),
        move |_| async move {
            if visibility.get_untracked() == VisibilityState::Visible {
                load_data().await
            } else {
                logging::log!("Document not visible. Data polling skipped for summary endpoint.");
                Ok(summary_sig.get_untracked())
            }
        },
    );

    let unique_blocks_producers_resource = create_resource(
        move || (counter.get()),
        move |_| async move {
            if visibility.get_untracked() == VisibilityState::Visible {
                load_block_producers_stat(10000).await
            } else {
                logging::log!("Document not visible. Data polling skipped for summary endpoint.");
                Ok(BlockchainStatResponse {
                    data: BlockchainStatData { blocks: vec![] },
                })
            }
        },
    );

    let get_data = move || resource.get().and_then(|res| res.ok());
    let augment_data = move || {
        get_data().map(|data: BlockchainSummary| {
            let mut chain = HashMap::new();
            chain.insert(
                BERKELEY_CHAIN_ID.to_string(),
                ChainSummary {
                    #[allow(deprecated)]
                    latest_epoch: data.epoch,
                    #[allow(deprecated)]
                    latest_slot: data.slot,
                },
            );
            chain.insert(
                MAINNET_CHAIN_ID.to_string(),
                ChainSummary {
                    latest_epoch: 79,
                    latest_slot: EPOCH_SLOTS as u64,
                },
            );
            BlockchainSummary {
                chain: Some(chain),
                ..data
            }
        })
    };

    create_effect(move |_| {
        if let Some(blockchain_summary) = augment_data() {
            set_summary.set(blockchain_summary);
        }
        if let Some(blockchain_stat) = unique_blocks_producers_resource
            .get()
            .and_then(|res| res.ok())
        {
            if let Some(block) = blockchain_stat.data.blocks.first() {
                set_stat.set(block.clone());
            }
        }
    });

    ().into_view()
}
