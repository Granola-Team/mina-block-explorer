use leptos::*;

use super::components::TransactionsSection;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    view! {
        <TransactionsSection public_key=None/>
    }
}
