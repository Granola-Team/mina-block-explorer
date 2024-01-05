use leptos::*;
use crate::icons::NoIcon;

#[component]
pub fn EmptyTable(message: String) -> impl IntoView {
    view! { 
        <div class="flex text-base text-slate-400 items-center justify-center p-8">
            <NoIcon /> 
            <span class="text-sm">{message}</span>
        </div>
    }
}