use super::{components::*, functions::*, models::BlockchainSummary};
use crate::{
    blocks::components::BlocksSection,
    common::{components::*, constants::*},
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

    view! {
        <Title text="Blocks | Search for blocks on Mina Blockchain"/>
        <PageContainer>
            {move || view! { <SummaryGrid summary=Some(summary_sig.get())/> }} <BlocksSection/>
        </PageContainer>
    }
}

#[component]
pub fn SummaryLocalStorage() -> impl IntoView {
    let (summary_sig, set_summary, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);
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

    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_summary.set(data))
    });

    ().into_view()
}
