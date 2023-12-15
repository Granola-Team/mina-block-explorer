use leptos::*;

use crate::nav::Nav;

#[component]
pub fn Header() -> impl IntoView {
    
    view! {
        <header class="flex sm:justify-around justify-center items-center fixed top-0 left-0 w-screen h-8">
            <h1>Logo</h1>
            <Nav />
        </header>
    }
}
