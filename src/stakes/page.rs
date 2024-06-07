use super::components::*;
use crate::{
    common::{components::*, constants::*},
    summary::models::BlockchainSummary,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

#[component]
pub fn StakesPage() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let (summary_sig, _, _) =
        use_local_storage::<BlockchainSummary, JsonCodec>(BLOCKCHAIN_SUMMARY_STORAGE_KEY);

    view! {
        <Title
            text=move || {
                if let Some(epoch) = epoch_sig.get() {
                    format!("Epoch {}", epoch)
                } else {
                    "Current".to_string()
                }
            }

            formatter=move |text| format!("Staking Ledger | {text}")
        />
        <PageContainer>
            {move || {
                view! {
                    <StakesPageContents
                        selected_epoch=epoch_sig.get()
                        current_epoch=summary_sig.get().epoch
                        slot_in_epoch=summary_sig.get().slot
                    />
                }
            }}

        </PageContainer>
    }
}
