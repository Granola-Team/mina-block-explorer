use leptos::*;
use leptos_router::*;

use super::functions::*;
use super::components::*;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    let query_params_map = use_query_map();

    let binding = query_params_map.get();
    let public_key = binding.get("account");

    view! {
        <TransactionsSection public_key=public_key.cloned()/>
    }
}

#[component]
pub fn TransactionSpotlightPage() -> impl IntoView {
    let memo_params_map = use_params_map();
    let _resource = create_resource(
        move || memo_params_map.get(),
        |value| async move {
            let state_hash = value.get("id");
            load_data(10, None, state_hash.cloned()).await
        },
    );

    view! { <span>"spotlight page"</span> }
}
