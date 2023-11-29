use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="/summary">Summary</a>
            <a href="/blocks">Blocks</a>
            <a href="/accounts">Accounts</a>
        </nav>
    }
}
