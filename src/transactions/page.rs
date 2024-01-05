use super::functions::load_data;
use crate::common::components::*;
use leptos::*;

#[component]
pub fn TransactionsPage() -> impl IntoView {
    view! {
        <TransactionsSection public_key=None/>
    }
}
