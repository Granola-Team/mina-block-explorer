use super::{components::*, functions::*, models::BlockchainSummary};
use crate::{
    blocks::components::BlocksSection,
    common::{components::*, constants::*},
};
use leptos::*;
use leptos_meta::Title;
use leptos_use::{storage::*, utils::JsonCodec};

#[component]
pub fn SummaryPage() -> impl IntoView {
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    view! {
        <Title text="Blocks | Search for blocks on Mina Blockchain"/>
        <PageContainer>
            {move || view! { <SummaryGrid summary=Some(summary_sig.get())/> }}
            <BlocksSection/>
        </PageContainer>
    }
}

#[component]
pub fn SummaryLocalStorage() -> impl IntoView {
    let (_, set_summary, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    let resource = create_resource(|| (), |_| async move { load_data().await });

    create_effect(move |_| {
        resource
            .get()
            .and_then(|res| res.ok())
            .map(|data| set_summary.set(data))
    });

    ().into_view()
}
