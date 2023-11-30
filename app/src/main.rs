use leptos::*;
mod root;
mod nav;
mod summary_page;
mod api_models;

use root::Root;

fn main() {
    leptos::mount_to_body(|| view! { <Root/> })
}

