use super::components::*;
use crate::common::components::*;
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn StakesPage() -> impl IntoView {
    let (epoch_sig, _) = create_query_signal::<i64>("epoch");

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
            <StakesPageContents/>
        </PageContainer>
    }
}
