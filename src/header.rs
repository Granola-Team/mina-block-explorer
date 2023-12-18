use leptos::*;

use crate::nav::Nav;

#[component]
pub fn Header() -> impl IntoView {
    
    view! {
        <header class="bg-main-background flex justify-center items-center md:grid md:grid-cols-[10%_20%_60%_10%] md:col-start-2 md:col-end-3 fixed top-0 left-0 w-screen h-16">
            <div class="md:col-start-2 md:col-end-3 flex items-center justify-start">
                <img src="../assets/img/logo.svg" width="45px" height="29px" alt="Minasearch" />
                <span class="ml-1 text-white font-bold text-xl">Mina</span><span class="text-granola-orange font-bold text-xl">Search</span>
            </div>
            <Nav />
        </header>
    }
}
