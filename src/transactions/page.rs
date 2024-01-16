use leptos::*;
use leptos_router::*;

use super::components::*;
use super::functions::*;
use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;
use crate::common::spotlight::*;
use crate::icons::*;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let query_params_map: Memo<ParamsMap> = use_query_map();

    let public_key = move || query_params_map.with(|params| params.get("account").cloned());

    view! {
        <PageContainer>
            <TransactionsSection public_key=public_key()/>
        </PageContainer>
    }
}

#[component]
pub fn TransactionSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(10, None, state_hash.cloned()).await
        },
    );

    view! {
        <PageContainer>
            { move || match resource.get() {
                Some(Ok(data)) => {
                    match data.transactions.first() {
                        Some(Some(transaction)) => {
                            let state_hash = get_hash(transaction);
                            let date_time = get_block_datetime(transaction);
                            let spotlight_items=vec![
                                SpotlightEntry { label: "Date".to_string(), value: get_block_datetime(transaction), pill_variant: None },
                                SpotlightEntry { label: "Transaction Hash".to_string(), value: get_hash(transaction), pill_variant: None },
                                SpotlightEntry { label: "Payment ID".to_string(), value: get_payment_id(transaction), pill_variant: None },
                                SpotlightEntry { label: "Block Height".to_string(), value: get_block_height(transaction), pill_variant: None },
                                SpotlightEntry { label: "Canonical".to_string(), value: get_canonical(transaction), pill_variant: None },
                                SpotlightEntry { label: "Block State Hash".to_string(), value: get_block_state_hash(transaction), pill_variant: None },
                                SpotlightEntry { label: "Amount".to_string(), value: get_amount(transaction), pill_variant: Some(PillVariant::Green) },
                                SpotlightEntry { label: "Fee".to_string(), value: get_fee(transaction), pill_variant: Some(PillVariant::Orange) },
                                SpotlightEntry { label: "From".to_string(), value: get_from(transaction), pill_variant: None },
                                SpotlightEntry { label: "To".to_string(), value: get_to(transaction), pill_variant: None },
                                SpotlightEntry { label: "Nonce".to_string(), value: get_nonce(transaction), pill_variant: None },
                                SpotlightEntry { label: "Memo".to_string(), value: get_memo(transaction), pill_variant: None },
                                SpotlightEntry { label: "Kind".to_string(), value: get_kind(transaction), pill_variant: Some(PillVariant::Blue) },
                            ];
                            view!{
                                <SpotlightSection header="Transaction Spotlight".to_string() spotlight_items=spotlight_items id=state_hash meta=format!("{} ({})", date_time, print_time_since(&date_time))>
                                    <TransactionIcon width=40/>
                                </SpotlightSection>
                            }.into_view()
                        }
                        _ => view! { <NullView />}
                    }
                },
                _ => view! { <NullView /> },
            }}
        </PageContainer>
    }
}
