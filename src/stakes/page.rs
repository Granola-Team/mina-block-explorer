use super::components::*;
use crate::{common::components::*, summary::functions::load_data as load_summary_data};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");
    let summary_resource = create_resource(|| (), |_| async move { load_summary_data().await });

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
                summary_resource
                    .get()
                    .and_then(|res| res.ok())
                    .map(|data| {
                        view! {
                            <StakesPageContents current_epoch=data.epoch slot_in_epoch=data.slot/>
                        }
                    })
                    .unwrap_or(().into_view())
            }}

        </PageContainer>
    }
}
