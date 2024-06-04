use super::components::*;
use crate::{common::components::*, summary::functions::load_data as load_summary_data};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

    let get_data = move || summary_resource.get().and_then(|res| res.ok());
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
                let current_epoch = get_data().map(|data| data.epoch);
                let slot_in_epoch = get_data().map(|data| data.slot);
                let selected_epoch = epoch_sig.get();
                match (current_epoch, slot_in_epoch) {
                    (Some(epoch), Some(slot)) => {
                        view! {
                            <StakesPageContents
                                selected_epoch=selected_epoch
                                current_epoch=epoch
                                slot_in_epoch=slot
                            />
                        }
                    }
                    _ => ().into_view(),
                }
            }}

        </PageContainer>
    }
}
