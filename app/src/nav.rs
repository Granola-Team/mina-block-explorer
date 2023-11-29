use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="w-screen flex">
            <a class="m-1.5" href="/summary">Summary</a>
            <a class="m-1.5" href="/blocks">Blocks</a>
            <a class="m-1.5" href="/accounts">Accounts</a>
        </nav>
    }
}
