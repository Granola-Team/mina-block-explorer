use leptos::*;

use crate::nav::Nav;

#[component]
pub fn Header() -> impl IntoView {
    
    view! {
        <header class="bg-white flex justify-center items-center md:grid md:grid-cols-[10%_20%_60%_10%] md:col-start-2 md:col-end-3 fixed top-0 left-0 w-screen h-16">
            <h1 class="md:col-start-2 md:col-end-3">Logo</h1>
            <Nav />
        </header>
    }
}
