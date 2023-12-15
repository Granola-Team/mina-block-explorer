use leptos::*;

use crate::nav::Nav;

#[component]
pub fn Header() -> impl IntoView {
    
    view! {
        <header class="bg-blue-800 fixed top-0 left-0 w-screen text-center">
            <h1 class="mt-2 mb-2">Logo</h1>
            <Nav />
        </header>
    }
}
