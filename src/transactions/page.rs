use leptos::*;
use leptos_router::*;

use super::components::TransactionsSection;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let binding = query_params_map.get();
    let public_key = binding.get("account");

    view! {
        <TransactionsSection public_key=public_key.cloned()/>
    }
}
